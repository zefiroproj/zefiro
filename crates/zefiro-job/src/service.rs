use anyhow::{Error, Result};
use async_nats::{self, Message};
use k8s_openapi::api::batch::v1::Job;
use k8s_openapi::api::core::v1::Service;
use kube::{api::{Api, PostParams}, Client, ResourceExt};
use log::info;
use serde_json::json;
use async_nats::service::ServiceExt;
use tokio_stream::StreamExt;

use crate::{builder::JobBuilder, priority::JobPriority, resources::Resources};

pub struct KubeService {
    k8s_api: Api<Service>,
    nats_client: async_nats::Client,
    jobs_api: Api<Job>,
}

impl KubeService {
    pub async fn new(namespace: &str, nats_service_name: &str) -> Result<Self> {
        let k8s_client = Client::try_default().await?;
        let k8s_api: Api<Service> = Api::namespaced(k8s_client.clone(), namespace);
        let jobs_api: Api<Job> = Api::namespaced(k8s_client, namespace);

        let nats_service = k8s_api.get(nats_service_name).await?;
        if let Some(cluster_ip) = nats_service.spec.and_then(|spec| spec.cluster_ip) {
            info!("NATS Service IP: {}", cluster_ip);

            let nats_address = "localhost:4222"; // format!("{}:4222", cluster_ip);
            let nats_client = async_nats::connect(&nats_address).await?;
            info!("Connected to NATS at {}", nats_address);

            Ok(Self {
                k8s_api,
                nats_client,
                jobs_api,
            })
        } else {
            Err(Error::msg("NATS Service IP address not found"))
        }
    }

    pub async fn run(&self) -> Result<()> {
        let mut service = self
            .nats_client
            .service_builder()
            .description("A service to run jobs on kubernetes")
            .stats_handler(|endpoint, stats| json!({ "endpoint": endpoint }))
            .start("kube", "1.0.0")
            .await
            .unwrap();

        info!("Service started successfully: kube-service 1.0.0");


        let mut endpoint = service.endpoint("kube.get").await.unwrap();

        while let Some(request) = endpoint.next().await {
            info!("Received message: {:?}", request.message);
            self.launch_job(request.message).await?;
        }

        Ok(())
    }

    async fn launch_job(&self, data: Message) -> Result<()> {
        let job_name = "vidjil-job";
        let job = JobBuilder::new(
            job_name,
            job_name,
            "vidjil:latest",
            vec![
                "--in-fastq=/inputs/in_R12.fastq.gz".to_string(),
                "--out-fasta=/inputs/output.fasta.gz".to_string(),
                "--vdj-ref=/inputs/vidjil.germline.only_human.tar.gz".to_string(),
            ],
            Resources::new(2.0, 1024, 1024),
            Some(Resources::new(8.0, 10000, 1024)),
            JobPriority::Lowest,
            120,
        )
        .create();

        let created_job = self
            .jobs_api
            .create(&PostParams::default(), &job)
            .await?;
        info!("Created job: {}", created_job.name_any());

        Ok(())
    }
}
