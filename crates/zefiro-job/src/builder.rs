use k8s_openapi::api::{
    batch::v1::{Job, JobSpec},
    core::v1::{Container, HostPathVolumeSource, PodSpec, PodTemplateSpec, ResourceRequirements, Volume, VolumeMount},
};
use kube::api::ObjectMeta;
use crate::{priority::JobPriority, resources::JobResources};

const INPUTS_DIR_NAME: &str = "inputs";
const OUTPUTS_DIR_NAME: &str = "outputs";
const HOST_PATH_TYPE: &str = "Directory";
const RESTART_POLICY: &str = "Never";

pub struct JobBuilder {
    job_name: Option<String>,
    container: Container,
    priority: JobPriority,
    time_limit: usize,
}

impl JobBuilder {
    pub fn new(
        job_name: &str,
        container_name: &str,
        image_uri: &str,
        container_args: Vec<String>,
        min_resources: JobResources,
        max_resources: Option<JobResources>,
        priority: JobPriority,
        time_limit: usize,
    ) -> Self {
        let container = Self::create_container_template(
            container_name,
            image_uri,
            container_args,
            min_resources,
            max_resources,
        );

        Self {
            job_name: Some(job_name.to_string()),
            container,
            priority,
            time_limit,
        }
    }

    fn create_volume_mount_template(name: &str) -> VolumeMount {
        VolumeMount {
            mount_path: format!("/{name}"),
            name: name.to_string(),
            ..Default::default()
        }
    }

    fn create_container_template(
        name: &str,
        image: &str,
        args: Vec<String>,
        requests: JobResources,
        limits: Option<JobResources>,
    ) -> Container {
        Container {
            name: name.to_string(),
            image: Some(image.to_string()),
            image_pull_policy: Some("Never".to_string()),
            args: Some(args),
            resources: Some(ResourceRequirements {
                limits: limits.map(|res| res.to_dict()),
                requests: Some(requests.to_dict()),
                ..Default::default()
            }),
            volume_mounts: Some(vec![
                Self::create_volume_mount_template(INPUTS_DIR_NAME),
                Self::create_volume_mount_template(OUTPUTS_DIR_NAME),
            ]),
            ..Default::default()
        }
    }

    fn create_volume_template(name: &str) -> Volume {
        Volume {
            name: name.to_string(),
            host_path: Some(HostPathVolumeSource {
                path: format!("/{name}"),
                type_: Some(HOST_PATH_TYPE.to_string()),
            }),
            ..Default::default()
        }
    }

    fn create_pod_template(&self) -> PodTemplateSpec {
        PodTemplateSpec {
            spec: Some(PodSpec {
                containers: vec![self.container.clone()],
                volumes: Some(vec![
                    Self::create_volume_template(INPUTS_DIR_NAME),
                    Self::create_volume_template(OUTPUTS_DIR_NAME),
                ]),
                priority_class_name: Some(self.priority.to_string()),
                restart_policy: Some(RESTART_POLICY.to_string()),
                ..Default::default()
            }),
            ..Default::default()
        }
    }

    pub fn build(&self) -> Job {
        Job {
            metadata: ObjectMeta {
                name: self.job_name.clone(),
                ..Default::default()
            },
            spec: Some(JobSpec {
                template: self.create_pod_template(),
                active_deadline_seconds: Some(self.time_limit as i64),
                backoff_limit: Some(0),
                ttl_seconds_after_finished: Some(0),
                ..Default::default()
            }),
            ..Default::default()
        }
    }
}
