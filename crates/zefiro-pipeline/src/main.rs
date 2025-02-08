mod message;
mod service;

use anyhow::Result;

const DEFAULT_K8S_NAMESPACE: &str = "default";
const NATS_SERVICE_NAME: &str = "nats";

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    let kube_service = service::PipelineService::new(DEFAULT_K8S_NAMESPACE).await?;
    kube_service.run().await?;

    Ok(())
}
