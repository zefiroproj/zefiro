use service::KubeService;

mod resources;
mod status;
mod builder;
mod priority;
mod service;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let namespace = "default";
    let nats_service_name = "nats";

    let kube_service = KubeService::new(namespace, nats_service_name).await?;
    kube_service.run().await;

    Ok(())
}
