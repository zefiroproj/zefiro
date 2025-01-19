use std::collections::BTreeMap;
use k8s_openapi::{
    api::core::v1::{
        Container, ContainerPort, HostPathVolumeSource, Pod, PodSpec, ResourceRequirements, Volume
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
        );

        let volumes = vec![Self::create_host_path_volume("inputs", "/inputs", "Directory")];

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
        if let (Some(cpu), Some(ram), Some(disk)) = (max_cpu, max_ram, max_disk) {
            Some(BTreeMap::from([
                ("memory".to_string(), Quantity(format!("{}M", ram))),
                ("cpu".to_string(), Quantity(cpu.to_string())),
                ("ephemeral-storage".to_string(), Quantity(format!("{}M", disk))),
            ]))
        } else {
            None
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
            ..Default::default()
        }
    }

    fn create_host_path_volume(name: &str, path: &str, volume_type: &str) -> Volume {
        Volume {
            name: name.to_string(),
            host_path: Some(HostPathVolumeSource {
                path: path.to_string(),
                type_: Some(volume_type.to_string()),
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