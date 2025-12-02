// Advanced storage features

use std::collections::HashMap;

/// Model versioning system
pub struct ModelVersioning {
    versions: HashMap<String, Vec<Version>>,
}

impl ModelVersioning {
    pub fn new() -> Self {
        Self {
            versions: HashMap::new(),
        }
    }

    pub fn add_version(&mut self, model_name: &str, version: Version) {
        self.versions
            .entry(model_name.to_string())
            .or_insert_with(Vec::new)
            .push(version);
    }

    pub fn get_latest(&self, model_name: &str) -> Option<&Version> {
        self.versions
            .get(model_name)
            .and_then(|versions| versions.last())
    }

    pub fn get_version(&self, model_name: &str, version_id: &str) -> Option<&Version> {
        self.versions.get(model_name).and_then(|versions| {
            versions.iter().find(|v| v.id == version_id)
        })
    }
}

#[derive(Debug, Clone)]
pub struct Version {
    pub id: String,
    pub timestamp: u64,
    pub checksum: Vec<u8>,
    pub size: u64,
}

impl Default for ModelVersioning {
    fn default() -> Self {
        Self::new()
    }
}
