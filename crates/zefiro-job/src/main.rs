use service::KubeService;
use anyhow::Result;

mod resources;
mod status;
mod builder;
mod priority;
mod service;
mod message;

const DEFAULT_K8S_NAMESPACE: &str = "default";
const NATS_SERVICE_NAME: &str = "nats";

#[tokio::main]
async fn main() -> Result<()> {
    let kube_service = KubeService::new(DEFAULT_K8S_NAMESPACE, NATS_SERVICE_NAME).await?;
    kube_service.run().await;

    Ok(())
}
