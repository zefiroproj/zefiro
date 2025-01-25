use k8s_openapi::api::batch::v1::Job;
use k8s_openapi::api::core::v1::Pod;
use kube::{Api, Client, ResourceExt};
use kube::api::PostParams;
use job::JobPriority;
use resources::Resources;
use tokio::time::{Duration, sleep};

mod resources;
mod job;
use crate::job::JobBuilder;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the Kubernetes client
    let client = Client::try_default().await?;
    let jobs: Api<Job> = Api::default_namespaced(client);

    let job_name = "vidjil";
    let min_resources = Resources::new(2.0, 1024, 1024);
    let max_resources = Some(Resources::new(8.0, 10000, 1024));
    let job = JobBuilder::new(
        job_name,
        job_name,
        "vidjil:latest",
        80,
        vec![
            "--in-fastq=/inputs/in_R12.fastq.gz".to_string(),
            "--out-fasta=/inputs/output.fasta.gz".to_string(),
            "--vdj-ref=/inputs/vidjil.germline.only_human.tar.gz".to_string()
        ],
        min_resources,
        max_resources,
        JobPriority::Lowest,
        120
    ).create();

    let job = jobs.create(&PostParams::default(), &job).await?;
    println!("Created job: {}", job.name_any());

    // Wait for the Pod to be ready
    let timeout = Duration::from_secs(60);
    let start = std::time::Instant::now();
    // loop {
    //     let job = jobs.get(job_name).await?;
    //     let status = job.status.as_ref().expect("Pod status should be available");
    //     let Some(phase) = &status.status else {
    //         if start.elapsed() > timeout {
    //             return Err("Timed out waiting for pod to be ready".into());
    //         }
    //         sleep(Duration::from_secs(1)).await;
    //         continue;
    //     };

    //     if phase == "Running" {
    //         println!("Pod is running");
    //         break;
    //     }

    //     if start.elapsed() > timeout {
    //         return Err("Timed out waiting for pod to be ready".into());
    //     }
    //     sleep(Duration::from_secs(1)).await;
    // }

    // // Fetch logs
    // let logs = jobs.logs(job_name, &Default::default()).await?;
    // println!("Pod logs:\n{}", logs);

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