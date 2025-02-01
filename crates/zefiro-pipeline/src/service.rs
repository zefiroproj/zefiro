use anyhow::Result;
use async_nats;
use k8s_openapi::api::batch::v1::Job; 
use kube::{api::{Api, PostParams}, Client, ResourceExt};
use log::info;
use serde_json::json;
use async_nats::service::ServiceExt;
use tokio_stream::StreamExt;

use crate::{builder::JobBuilder, message::Message};

const SERVICE_NAME: &str = "zefiro-job";
const SERVICE_VERSION: &str = "1.0.0";

pub struct KubeService {
    nats: async_nats::Client,
    k8s: Api<Job>,
}

impl KubeService {
    pub async fn new(namespace: &str) -> Result<Self> {
        let k8s_client = Client::try_default().await?;
        let k8s_api: Api<Job> = Api::namespaced(k8s_client, namespace);

        let nats_address = "localhost:4222";
        let nats_client = async_nats::connect(&nats_address).await?;
        info!("Connected to NATS at {}", nats_address);

        Ok(Self {
            nats: nats_client,
            k8s: k8s_api,
        })
    }

    pub async fn run(&self) -> Result<()> {
        let service = self
            .nats
            .service_builder()
            .description("A service to run jobs on kubernetes")
            .stats_handler(|endpoint, _| json!({ "endpoint": endpoint }))
            .start(SERVICE_NAME, SERVICE_VERSION)
            .await
            .unwrap();

        info!("{}", format!(
            "Service started successfully: {} {}", SERVICE_NAME, SERVICE_VERSION
        ));


        let mut endpoint = service.endpoint(format!("{}.get", SERVICE_NAME)).await.unwrap();

        while let Some(request) = endpoint.next().await {
            info!("Received message: {:?}", request.message);
            let message = Message::from_string(&String::from_utf8(request.message.payload.to_vec()).unwrap()).unwrap();
            self.launch_job(message).await?;
        }

        Ok(())
    }

    async fn launch_job(&self, msg: Message) -> Result<()> {
        let job = JobBuilder::new(
            &msg.job_id,
            &msg.job_id,
            &msg.image,
            msg.args,
            msg.min_resources,
            msg.max_resources,
            msg.priority,
            msg.time_limit,
        )
        .create();

        info!("{:?}", job);

        let created_job = self.k8s
            .create(&PostParams::default(), &job)
            .await?;
        info!("Created job: {}", created_job.name_any());

        Ok(())
    }
}
