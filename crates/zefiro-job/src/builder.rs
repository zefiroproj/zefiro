use crate::{priority::JobPriority, resources::JobResources};
use k8s_openapi::api::{
    batch::v1::{Job, JobSpec},
    core::v1::{
        Container, HostPathVolumeSource, PodSpec, PodTemplateSpec, ResourceRequirements, Volume,
        VolumeMount,
    },
};
use kube::api::ObjectMeta;

const HOST_PATH_TYPE: &str = "Directory";
const RESTART_POLICY: &str = "Never";
const TTL_SECONDS: usize = 300; // 5 min in seconds

pub struct JobBuilder {
    job_name: String,
    container: Container,
    priority: JobPriority,
    time_limit: usize,
    inputs_dir: String,
    outputs_dir: String,
    ttl_seconds: usize,
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
        inputs_dir: &str,
        outputs_dir: &str,
    ) -> Self {
        Self {
            job_name: job_name.to_string(),
            container: Self::create_container(
                container_name,
                image_uri,
                container_args,
                min_resources,
                max_resources,
                inputs_dir,
                outputs_dir,
            ),
            priority,
            time_limit,
            inputs_dir: inputs_dir.to_string(),
            outputs_dir: outputs_dir.to_string(),
            ttl_seconds: if time_limit > TTL_SECONDS {
                time_limit
            } else {
                TTL_SECONDS
            },
        }
    }

    fn create_volume_mount(dir: &str, name: &str) -> VolumeMount {
        VolumeMount {
            name: name.to_string(),
            mount_path: dir.to_string(),
            ..Default::default()
        }
    }

    fn create_container(
        name: &str,
        image: &str,
        args: Vec<String>,
        requests: JobResources,
        limits: Option<JobResources>,
        inputs_dir: &str,
        outputs_dir: &str,
    ) -> Container {
        Container {
            name: name.to_string(),
            image: Some(image.to_string()),
            image_pull_policy: Some(RESTART_POLICY.to_string()),
            args: Some(args),
            resources: Some(ResourceRequirements {
                limits: limits.map(|res| res.to_dict()),
                requests: Some(requests.to_dict()),
                ..Default::default()
            }),
            volume_mounts: Some(vec![
                Self::create_volume_mount(inputs_dir, "inputs"),
                Self::create_volume_mount(outputs_dir, "outputs"),
            ]),
            ..Default::default()
        }
    }

    fn create_volume(name: &str, dir: &str) -> Volume {
        Volume {
            name: name.to_string(),
            host_path: Some(HostPathVolumeSource {
                path: dir.to_string(),
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
                    Self::create_volume("inputs", &self.inputs_dir),
                    Self::create_volume("outputs", &self.outputs_dir),
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
                name: Some(self.job_name.clone()),
                ..Default::default()
            },
            spec: Some(JobSpec {
                template: self.create_pod_template(),
                active_deadline_seconds: Some(self.time_limit as i64),
                backoff_limit: Some(0),
                ttl_seconds_after_finished: Some(self.ttl_seconds as i32),
                ..Default::default()
            }),
            ..Default::default()
        }
    }
}
