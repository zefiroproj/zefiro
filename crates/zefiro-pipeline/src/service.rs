use anyhow::Result;
use async_nats;
use k8s_openapi::api::batch::v1::Job;
use kube::{api::{Api, PostParams}, Client, ResourceExt};
use log::info;
use serde_json::json;
use async_nats::service::ServiceExt;
use tokio_stream::StreamExt;

use petgraph::{algo::toposort, visit::EdgeRef};
use petgraph::graph::NodeIndex;
use futures::stream::FuturesUnordered;

use zefiro_cwl::CwlSchema;
use zefiro_job::{JobBuilder, JobResources, JobPriority};

const SERVICE_NAME: &str = "zefiro-pipeline";
const SERVICE_VERSION: &str = "1.0.0";

pub struct PipelineService {
    nats: async_nats::Client,
    k8s: Api<Job>,
}

impl PipelineService {
    pub async fn new(namespace: &str) -> Result<Self> {
        let k8s_client = Client::try_default().await?;
        let nats_address = "localhost:4222";
        info!("Connected to NATS at {}", nats_address);

        Ok(Self {
            nats: async_nats::connect(&nats_address).await?,
            k8s: Api::namespaced(k8s_client, namespace),
        })
    }

    pub async fn run(&self) -> Result<()> {
        let service = self
            .nats
            .service_builder()
            .description("A service to run jobs on Kubernetes")
            .stats_handler(|endpoint, _| json!({ "endpoint": endpoint }))
            .start(SERVICE_NAME, SERVICE_VERSION)
            .await
            .unwrap();

        info!("Service started successfully: {} {}", SERVICE_NAME, SERVICE_VERSION);

        let mut endpoint = service.endpoint(format!("{}.get", SERVICE_NAME)).await.unwrap();

        while let Some(request) = endpoint.next().await {
            info!("Received message: {:?}", request.message);
            self.launch_pipeline().await?;
        }

        Ok(())
    }

    async fn launch_job(&self, job_name: &str, image: &str, args: Vec<String>) -> Result<()> {
        let job = JobBuilder::new(
            job_name,
            job_name,
            image,
            args,
            JobResources::new(4.0, 8000, 1024),
            None,
            JobPriority::Medium,
            300,
            "/inputs",
            "/outputs"
        )
        .build();

        let created_job = self.k8s.create(&PostParams::default(), &job).await?;
        info!("Created job: {}", created_job.name_any());
        Ok(())
    }

    async fn launch_pipeline(&self) -> Result<()> {
        let file_path = "../zefiro-cwl/test_data/cwl/wf-schema.yml";
        let cwl_schema = CwlSchema::from_path(&file_path).expect("Failed to deserialize CWL schema");

        match cwl_schema {
            CwlSchema::Workflow(wf) => {
                let graph = wf.to_graph();
                let sorted = toposort(&graph, None).expect("Graph is not a DAG!");
                info!("Pipeline DAG topological order: {:?}", sorted);

                let mut job_futures = FuturesUnordered::new();
                let mut completed_jobs = Vec::new();

                for &node in &sorted {
                    let image = "vidjil-tool:latest";
                    let args = vec!["--input".to_string(), "data".to_string()];

                    let dependencies: Vec<NodeIndex> = graph
                        .edges_directed(node, petgraph::Incoming)
                        .map(|e| e.source())
                        .collect();

                    for dep in dependencies {
                        while !completed_jobs.contains(&dep) {
                            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                        }
                    }

                    let job_future = self.launch_job(graph[node], image, args);
                    job_futures.push(job_future);
                    completed_jobs.push(node);
                }

                while let Some(result) = job_futures.next().await {
                    result?;
                }
            }
            CwlSchema::CommandLineTool(_) => {
                todo!("Single job execution not implemented yet")
            }
        }

        Ok(())
    }
}
