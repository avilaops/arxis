//! Storage service - S3-compatible object storage

use crate::error::Result;
use bytes::Bytes;
use std::path::PathBuf;

pub struct StorageService {
    data_dir: PathBuf,
}

impl StorageService {
    pub fn new(data_dir: PathBuf) -> Result<Self> {
        std::fs::create_dir_all(&data_dir)?;
        Ok(Self { data_dir })
    }

    pub async fn create_bucket(&self, name: &str) -> Result<()> {
        let bucket_path = self.data_dir.join(name);
        std::fs::create_dir_all(bucket_path)?;
        Ok(())
    }

    pub async fn put_object(&self, bucket: &str, key: &str, data: Bytes) -> Result<()> {
        let path = self.object_path(bucket, key);
        std::fs::create_dir_all(path.parent().unwrap())?;
        tokio::fs::write(&path, &data).await?;
        Ok(())
    }

    pub async fn get_object(&self, bucket: &str, key: &str) -> Result<Bytes> {
        let path = self.object_path(bucket, key);
        let data = tokio::fs::read(&path).await?;
        Ok(Bytes::from(data))
    }

    pub async fn list_objects(&self, bucket: &str, prefix: &str) -> Result<Vec<String>> {
        let bucket_path = self.data_dir.join(bucket).join(prefix);
        let mut objects = Vec::new();

        if let Ok(entries) = std::fs::read_dir(bucket_path) {
            for entry in entries.flatten() {
                if let Ok(name) = entry.file_name().into_string() {
                    objects.push(name);
                }
            }
        }

        Ok(objects)
    }

    pub async fn delete_object(&self, bucket: &str, key: &str) -> Result<()> {
        let path = self.object_path(bucket, key);
        tokio::fs::remove_file(&path).await?;
        Ok(())
    }

    fn object_path(&self, bucket: &str, key: &str) -> PathBuf {
        self.data_dir.join(bucket).join(key)
    }
}
