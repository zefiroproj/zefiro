use k8s_openapi::api::core::v1::Pod;
use kube::{Api, Client, ResourceExt};
use kube::api::PostParams;
use tokio::time::{Duration, sleep};

mod pod;
use crate::pod::PodBuilder;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the Kubernetes client
    let client = Client::try_default().await?;
    let pods: Api<Pod> = Api::default_namespaced(client);

    let pod_name = "vidjil";
    let pod = PodBuilder::new(
        pod_name,
        pod_name,
        "vidjil:latest",
        80,
        vec![
            "--in-fastq=/inputs/in_R12.fastq.gz".to_string(),
            "--out-fasta=/inputs/output.fasta.gz".to_string(),
            "--vdj-ref=/inputs/vidjil.germline.only_human.tar.gz".to_string()
        ],
        1,
        4000,
        4000,
        None,
        None,
        None
    ).create();
    // Create the Pod
    let pod = pods.create(&PostParams::default(), &pod).await?;
    println!("Created pod: {}", pod.name_any());

    // Wait for the Pod to be ready
    let timeout = Duration::from_secs(60);
    let start = std::time::Instant::now();
    loop {
        let pod = pods.get(pod_name).await?;
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
    let logs = pods.logs(pod_name, &Default::default()).await?;
    println!("Pod logs:\n{}", logs);

    Ok(())
}


// use kube::{Client, api::{Api}};
// use k8s_openapi::api::core::v1::Service;
// use nats::{Options, Connection};
// use std::{error::Error};
// use tokio::time::sleep;

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn Error>> {
//     let client = Client::try_default().await?;

//     let api: Api<Service> = Api::namespaced(client, "default");

//     let svc = api.get("nats").await?;

//     if let Some(cluster_ip) = svc.spec.unwrap().cluster_ip {
//         println!("NATS Service IP: {}", cluster_ip);

//         let nats_address = "localhost:4222"; // format!("{}:4222", cluster_ip);

//         let nc = nats::connect(nats_address)?;

//         nc.publish("msg.test", b"Hello, NATS!")?;
//         println!("Messaged sent!");

//         let sub = nc.subscribe("msg.test")?;

//         let message = sub.next();
//         println!("Recieved a message: {:?}", message);

//         sleep(std::time::Duration::from_secs(5)).await;

//         Ok(())
//     } else {
//         println!("IP address of NATS not found");
//         Err("IP address not found".into())
//     }
// }