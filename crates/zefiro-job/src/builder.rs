use std::collections::BTreeMap;
use k8s_openapi::api::{batch::v1::{Job, JobSpec}, core::v1::{
        Container, HostPathVolumeSource, PodSpec, PodTemplateSpec, ResourceRequirements, Volume, VolumeMount
    }
};
use kube::api::ObjectMeta;

use crate::{priority::JobPriority, resources::Resources};


pub struct JobBuilder {
    pub pod_name: Option<String>,
    container: Container,
    volumes: Vec<Volume>,
    priority: JobPriority,
    time_limit: usize,
}

impl JobBuilder {
    pub fn new(
        pod_name: &str,
        container_name: &str,
        image_uri: &str,
        container_args: Vec<String>,
        min_resources: Resources,
        max_resources: Option<Resources>,
        priority: JobPriority,
        time_limit: usize
    ) -> Self {
        let container = Self::create_container(
            container_name,
            image_uri,
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
            time_limit
        }
    }

    fn create_container(
        name: &str,
        image: &str,
        args: Vec<String>,
        limits: Option<Resources>,
        requests: Resources,
        mount_path: &str,
        mount_name: &str
    ) -> Container {
        Container {
            name: name.to_string(),
            image: Some(image.to_string()),
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
                backoff_limit: Some(0),
                ttl_seconds_after_finished: Some(120),
                ..Default::default()
                
            }),
            ..Default::default()
        }
    }
}
