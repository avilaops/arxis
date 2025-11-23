use crate::models::*;
use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
use tokio::fs;
use uuid::Uuid;

/// Storage for medical cases and associated files
pub struct CaseStorage {
    base_path: PathBuf,
}

impl CaseStorage {
    pub fn new<P: AsRef<Path>>(base_path: P) -> Result<Self> {
        let base_path = base_path.as_ref().to_path_buf();
        std::fs::create_dir_all(&base_path)?;
        Ok(Self { base_path })
    }

    pub async fn save_case(&mut self, case: &MedicalCase) -> Result<()> {
        let case_dir = self.base_path.join(case.id.to_string());
        fs::create_dir_all(&case_dir).await?;

        let case_file = case_dir.join("case.json");
        let json = serde_json::to_string_pretty(case)?;
        fs::write(case_file, json).await?;

        Ok(())
    }

    pub async fn load_case(&self, case_id: Uuid) -> Result<MedicalCase> {
        let case_file = self.base_path.join(case_id.to_string()).join("case.json");
        let json = fs::read_to_string(case_file)
            .await
            .context("Case not found")?;
        let case = serde_json::from_str(&json)?;
        Ok(case)
    }

    pub async fn save_photo(&mut self, case_id: Uuid, photo_id: Uuid, data: &[u8]) -> Result<()> {
        let photos_dir = self.base_path.join(case_id.to_string()).join("photos");
        fs::create_dir_all(&photos_dir).await?;

        let photo_file = photos_dir.join(format!("{}.jpg", photo_id));
        fs::write(photo_file, data).await?;

        // Update case with new photo ID
        let mut case = self.load_case(case_id).await?;
        case.photos.push(photo_id);
        case.status = CaseStatus::PhotosUploaded;
        self.save_case(&case).await?;

        Ok(())
    }

    pub async fn load_photo(&self, case_id: Uuid, photo_id: Uuid) -> Result<Vec<u8>> {
        let photo_file = self
            .base_path
            .join(case_id.to_string())
            .join("photos")
            .join(format!("{}.jpg", photo_id));

        let data = fs::read(photo_file).await?;
        Ok(data)
    }

    pub async fn update_case_reconstruction(
        &mut self,
        case_id: Uuid,
        reconstruction: Reconstruction3D,
    ) -> Result<()> {
        let mut case = self.load_case(case_id).await?;
        case.reconstruction = Some(reconstruction);
        case.status = CaseStatus::Reconstructed;
        self.save_case(&case).await?;
        Ok(())
    }
}
