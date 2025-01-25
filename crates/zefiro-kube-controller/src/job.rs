use std::collections::BTreeMap;
use k8s_openapi::{
    api::{batch::v1::{Job, JobSpec}, core::v1::{
        Container, ContainerPort, HostPathVolumeSource, Pod, PodSpec, PodTemplateSpec, ResourceRequirements, Volume, VolumeMount
    }, scheduling::v1::PriorityClass},
    apimachinery::pkg::api::resource::Quantity
};
use kube::api::{Object, ObjectMeta};
use kube::Client;
use std::sync::{Arc, Mutex};
use tokio::sync::Mutex as AsyncMutex;
use log::{info, warn, error};

use crate::resources::Resources;

enum JobStatus {
    Queued,
    Running,
    Stopping,
    Failing,
    Stopped,
    Failed,
    Done
}

pub enum JobPriority {
    Lowest,
    Low,
    Medium,
    High,
    Highest,
}

impl JobPriority {
    pub fn to_string(&self) -> String {
        let priority = match self {
            Self::Lowest => "lowest",
            Self::Low => "low",
            Self::Medium => "medium",
            Self::High => "high",
            Self::Highest => "highest",
        };
        priority.to_string()
    }
}


pub struct JobBuilder {
    pub pod_name: Option<String>,
    container: Container,
    volumes: Vec<Volume>,
    priority: JobPriority,
    time_limit: usize,
    retries: usize
}

impl JobBuilder {
    pub fn new(
        pod_name: &str,
        container_name: &str,
        image_uri: &str,
        container_port: i32,
        container_args: Vec<String>,
        min_resources: Resources,
        max_resources: Option<Resources>,
        priority: JobPriority,
        time_limit: usize
    ) -> Self {
        let container = Self::create_container(
            container_name,
            image_uri,
            container_port,
            container_args,
            max_resources,
            min_resources,
            "/inputs",
            "inputs"
        );

        let volumes = vec![Self::create_host_path_volume("inputs", "/inputs", Some("Directory"))];

        Self {
            pod_name: Some(pod_name.to_string()),
            container,
            volumes,
            priority,
            time_limit,
            retries: 0
        }
    }

    fn create_container(
        name: &str,
        image: &str,
        port: i32,
        args: Vec<String>,
        limits: Option<Resources>,
        requests: Resources,
        mount_path: &str,
        mount_name: &str
    ) -> Container {
        Container {
            name: name.to_string(),
            image: Some(image.to_string()),
            ports: Some(vec![ContainerPort {
                container_port: port,
                ..Default::default()
            }]),
            image_pull_policy: Some("Never".to_string()),
            args: Some(args),
            resources: Some(ResourceRequirements {
                limits: Some(limits.map_or(BTreeMap::new(), |resources| resources.to_dict())),
                requests: Some(requests.to_dict()),
                ..Default::default()
            }),
            volume_mounts: Some(vec![VolumeMount {
                mount_path: mount_path.to_string(),
                name: mount_name.to_string(),
                ..Default::default()
            }]),
            ..Default::default()
        }
    }

    fn create_host_path_volume(name: &str, path: &str, volume_type: Option<&str>) -> Volume {
        Volume {
            name: name.to_string(),
            host_path: Some(HostPathVolumeSource {
                path: path.to_string(),
                type_: volume_type.map(|t| t.to_string()),
            }),
            ..Default::default()
        }
    }

    fn create_pod_template(&self) -> PodTemplateSpec {
        PodTemplateSpec {
            spec: Some(PodSpec {
                containers: vec![self.container.clone()],
                volumes: Some(self.volumes.clone()),
                priority_class_name: Some(self.priority.to_string()),
                restart_policy: Some("Never".to_string()),
                ..Default::default()
            }),
            ..Default::default()
        }
    }

    pub fn create(&self) -> Job {
        Job {
            metadata: ObjectMeta {
                name: self.pod_name.clone(),
                ..Default::default()
            },
            spec: Some(JobSpec {
                template: self.create_pod_template(),
                active_deadline_seconds: Some(self.time_limit as i64),
                backoff_limit: Some(self.retries as i32),
                ttl_seconds_after_finished: Some(0),
                ..Default::default()
                
            }),
            ..Default::default()
        }
    }
}

#[derive(Clone, Default)]
pub struct JobMonitor {
    pod_names: Arc<Mutex<Vec<String>>>,
    lock: Arc<AsyncMutex<()>>,
}

impl JobMonitor {
    pub fn new() -> Self {
        Self {
            pod_names: Arc::new(Mutex::new(Vec::new())),
            lock: Arc::new(AsyncMutex::new(())),
        }
    }

    pub async fn add(&self, pod_name: String) {
        let lock = self.lock.lock().await; // Acquire lock
        info!("JobMonitor adding {}", pod_name);
        let mut pod_names = self.pod_names.lock().unwrap();
        pod_names.push(pod_name);
    }

    pub async fn remove(&self, pod_name: &str) {
        let lock = self.lock.lock().await; // Acquire lock
        let mut pod_names = self.pod_names.lock().unwrap();
        if let Some(index) = pod_names.iter().position(|name| name == pod_name) {
            info!("JobMonitor removing {}", pod_name);
            pod_names.remove(index);
        } else {
            warn!("JobMonitor {} has already been removed", pod_name);
        }
    }

    pub async fn cleanup(&self) {
        info!("Starting Cleanup");
        let _lock = self.lock.lock().await; // Acquire lock

        let client = match Client::try_default().await {
            Ok(c) => c,
            Err(err) => {
                error!("Failed to create Kubernetes client: {:?}", err);
                return;
            }
        };

        let mut pod_names = self.pod_names.lock().unwrap();
        for pod_name in pod_names.iter() {
            info!("JobMonitor deleting pod {}", pod_name);
            // Replace this with actual Kubernetes deletion logic
            if let Err(err) = delete_pod(&client, pod_name).await {
                error!("Error deleting pod named {}, ignoring: {:?}", pod_name, err);
            }
        }
        pod_names.clear();
        info!("Finishing Cleanup");
    }
}

// Dummy function to simulate pod deletion
async fn delete_pod(client: &Client, pod_name: &str) -> Result<(), kube::Error> {
    // Implement your Kubernetes pod deletion logic here
    // For example:
    // let api: Api<Pod> = Api::namespaced(client.clone(), "default");
    // api.delete(pod_name, &DeleteParams::default()).await?;
    info!("Simulating deletion of pod {}", pod_name);
    Ok(())
}