use std::collections::BTreeMap;

use k8s_openapi::apimachinery::pkg::api::resource::Quantity;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct JobResources {
    cpus: f64,
    ram: u32,
    disk: u32
}

impl JobResources {
    pub fn new(cpus: f64, ram: u32, disk: u32) -> Self {
        JobResources { cpus, ram, disk }
    }
    
    pub fn to_dict(&self) -> BTreeMap<String, Quantity> {
        BTreeMap::from([
            ("memory".to_string(), Quantity(format!("{}M", self.ram))),
            ("cpu".to_string(), Quantity(self.cpus.to_string())),
            ("ephemeral-storage".to_string(), Quantity(format!("{}M", self.disk))),
        ])
    }
}