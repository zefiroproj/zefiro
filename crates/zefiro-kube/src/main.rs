use futures::{StreamExt, TryStreamExt};
use k8s_openapi::api::core::v1::{Container, ContainerPort, Pod, PodSpec};
use kube::api::{PostParams, WatchEvent};
use kube::{Api, Client, ResourceExt};
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the Kubernetes client
    let client = Client::try_default().await?;
    let pods: Api<Pod> = Api::default_namespaced(client);

    // Define the Pod with port 80 exposed
    let pod = Pod {
        metadata: kube::api::ObjectMeta {
            name: Some("nginx-example".to_string()),
            ..Default::default()
        },
        spec: Some(PodSpec {
            containers: vec![Container {
                name: "nginx".to_string(),
                image: Some("nginx:latest".to_string()),
                ports: Some(vec![ContainerPort {
                    container_port: 80,
                    ..Default::default()
                }]),
                ..Default::default()
            }],
            ..Default::default()
        }),
        ..Default::default()
    };

    // Create the Pod
    let pod = pods.create(&PostParams::default(), &pod).await?;
    println!("Created pod: {}", pod.name_any());

    // Wait for the Pod to be ready
    let timeout = Duration::from_secs(60);
    let start = std::time::Instant::now();
    loop {
        let pod = pods.get("nginx-example").await?;
        let status = pod.status.as_ref().expect("Pod status should be available");
        let Some(phase) = &status.phase else {
            if start.elapsed() > timeout {
                return Err("Timed out waiting for pod to be ready".into());
            }
            sleep(Duration::from_secs(1)).await;
            continue;
        };

        if phase == "Running" {
            println!("Pod is running");
            break;
        }

        if start.elapsed() > timeout {
            return Err("Timed out waiting for pod to be ready".into());
        }
        sleep(Duration::from_secs(1)).await;
    }

    // Fetch logs
    let logs = pods.logs("nginx-example", &Default::default()).await?;
    println!("Pod logs:\n{}", logs);

    Ok(())
}
