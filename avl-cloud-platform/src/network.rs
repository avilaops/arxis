//! Network service - VPC, Load Balancers, Firewalls

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct NetworkManager {
    vpcs: HashMap<String, VirtualPrivateCloud>,
    load_balancers: HashMap<String, LoadBalancer>,
}

impl NetworkManager {
    pub fn new() -> Self {
        Self {
            vpcs: HashMap::new(),
            load_balancers: HashMap::new(),
        }
    }

    pub fn create_vpc(&mut self, name: String, cidr: String) -> String {
        let vpc = VirtualPrivateCloud {
            id: format!("vpc-{}", uuid::Uuid::new_v4()),
            name,
            cidr,
            subnets: Vec::new(),
        };

        let id = vpc.id.clone();
        self.vpcs.insert(id.clone(), vpc);
        id
    }

    pub fn list_vpcs(&self) -> Vec<&VirtualPrivateCloud> {
        self.vpcs.values().collect()
    }

    pub fn create_load_balancer(&mut self, name: String, vpc_id: String) -> String {
        let lb = LoadBalancer {
            id: format!("lb-{}", uuid::Uuid::new_v4()),
            name,
            vpc_id,
            targets: Vec::new(),
        };

        let id = lb.id.clone();
        self.load_balancers.insert(id.clone(), lb);
        id
    }

    pub fn list_load_balancers(&self) -> Vec<&LoadBalancer> {
        self.load_balancers.values().collect()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VirtualPrivateCloud {
    pub id: String,
    pub name: String,
    pub cidr: String,
    pub subnets: Vec<Subnet>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subnet {
    pub id: String,
    pub cidr: String,
    pub availability_zone: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadBalancer {
    pub id: String,
    pub name: String,
    pub vpc_id: String,
    pub targets: Vec<String>,
}
