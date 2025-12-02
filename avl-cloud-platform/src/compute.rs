//! Compute service - Virtual machines and containers

use crate::error::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct ComputeManager {
    instances: HashMap<String, Instance>,
}

impl ComputeManager {
    pub fn new() -> Result<Self> {
        Ok(Self {
            instances: HashMap::new(),
        })
    }

    pub async fn create_instance(&mut self, spec: InstanceSpec) -> Result<Instance> {
        let instance = Instance {
            id: format!("i-{}", uuid::Uuid::new_v4()),
            name: spec.name,
            status: InstanceStatus::Pending,
            instance_type: spec.instance_type,
            vcpus: spec.vcpus,
            memory_mb: spec.memory_mb,
            disk_gb: spec.disk_gb,
            ip_address: None,
            created_at: chrono::Utc::now(),
        };

        self.instances.insert(instance.id.clone(), instance.clone());
        Ok(instance)
    }

    pub fn list_instances(&self) -> Vec<&Instance> {
        self.instances.values().collect()
    }

    pub async fn delete_instance(&mut self, id: &str) -> Result<()> {
        self.instances.remove(id);
        Ok(())
    }

    pub fn get_instance(&self, id: &str) -> Option<&Instance> {
        self.instances.get(id)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstanceSpec {
    pub name: String,
    pub instance_type: String,
    pub vcpus: u32,
    pub memory_mb: u64,
    pub disk_gb: u64,
    pub image: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Instance {
    pub id: String,
    pub name: String,
    pub status: InstanceStatus,
    pub instance_type: String,
    pub vcpus: u32,
    pub memory_mb: u64,
    pub disk_gb: u64,
    pub ip_address: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InstanceStatus {
    Pending,
    Running,
    Stopped,
    Terminated,
}
