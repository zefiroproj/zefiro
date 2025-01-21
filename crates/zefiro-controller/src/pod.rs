use std::collections::BTreeMap;
use k8s_openapi::{
    api::core::v1::{
        Container, ContainerPort, HostPathVolumeSource, Pod, PodSpec, ResourceRequirements, Volume, VolumeMount
    },
    apimachinery::pkg::api::resource::Quantity
};
use kube::api::ObjectMeta;

pub struct PodBuilder {
    pub pod_name: Option<String>,
    container: Container,
    volumes: Vec<Volume>,
}

impl PodBuilder {
    pub fn new(
        pod_name: &str,
        container_name: &str,
        image_uri: &str,
        container_port: i32,
        container_args: Vec<String>,
        min_cpu: u32,
        min_ram: u32,
        min_disk: u32,
        max_cpu: Option<u32>,
        max_ram: Option<u32>,
        max_disk: Option<u32>,
    ) -> Self {
        let resources_limits = Self::build_resource_limits(max_cpu, max_ram, max_disk);
        let resources_requests = Self::build_resource_requests(min_cpu, min_ram, min_disk);

        let container = Self::create_container(
            container_name,
            image_uri,
            container_port,
            container_args,
            resources_limits,
            resources_requests,
            "/inputs",
            "inputs"
        );

        let volumes = vec![Self::create_host_path_volume("inputs", "/inputs", Some("Directory"))];

        Self {
            pod_name: Some(pod_name.to_string()),
            container,
            volumes,
        }
    }

    fn build_resource_limits(
        max_cpu: Option<u32>,
        max_ram: Option<u32>,
        max_disk: Option<u32>,
    ) -> Option<BTreeMap<String, Quantity>> {
        let mut resource_limits = BTreeMap::new();
        if let Some(ram) = max_ram {
            resource_limits.insert("memory".to_string(), Quantity(ram.to_string()));
        }
        if let Some(cpu) = max_cpu {
            resource_limits.insert("cpu".to_string(), Quantity(cpu.to_string()));
        }
        if let Some(disk) = max_disk {
            resource_limits.insert("ephemeral-storage".to_string(), Quantity(disk.to_string()));
        }
        if resource_limits.is_empty() {
            None
        } else {
            Some(resource_limits)
        }
    }    

    fn build_resource_requests(
        min_cpu: u32,
        min_ram: u32,
        min_disk: u32,
    ) -> BTreeMap<String, Quantity> {
        BTreeMap::from([
            ("memory".to_string(), Quantity(format!("{}M", min_ram))),
            ("cpu".to_string(), Quantity(min_cpu.to_string())),
            ("ephemeral-storage".to_string(), Quantity(format!("{}M", min_disk))),
        ])
    }

    fn create_container(
        name: &str,
        image: &str,
        port: i32,
        args: Vec<String>,
        limits: Option<BTreeMap<String, Quantity>>,
        requests: BTreeMap<String, Quantity>,
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
                limits,
                requests: Some(requests),
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

    pub fn create(&self) -> Pod {
        Pod {
            metadata: ObjectMeta {
                name: self.pod_name.clone(),
                ..Default::default()
            },
            spec: Some(PodSpec {
                containers: vec![self.container.clone()],
                volumes: Some(self.volumes.clone()),
                ..Default::default()
            }),
            ..Default::default()
        }
    }
}