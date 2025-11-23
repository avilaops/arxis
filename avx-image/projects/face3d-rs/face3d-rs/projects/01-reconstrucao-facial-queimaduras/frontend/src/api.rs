use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

const API_BASE: &str = "http://localhost:3000/api";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MedicalCase {
    pub id: Uuid,
    pub patient_id: String,
    pub status: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateCaseRequest {
    pub patient_id: String,
}

pub async fn create_case(patient_id: String) -> Result<MedicalCase, String> {
    let response = Request::post(&format!("{}/cases", API_BASE))
        .json(&CreateCaseRequest { patient_id })
        .map_err(|e| format!("Failed to serialize: {}", e))?
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;

    if !response.ok() {
        return Err(format!("API error: {}", response.status()));
    }

    let data: serde_json::Value = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse response: {}", e))?;

    let case: MedicalCase = serde_json::from_value(data["case"].clone())
        .map_err(|e| format!("Failed to deserialize: {}", e))?;

    Ok(case)
}

pub async fn get_case(case_id: Uuid) -> Result<MedicalCase, String> {
    let response = Request::get(&format!("{}/cases/{}", API_BASE, case_id))
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;

    if !response.ok() {
        return Err(format!("API error: {}", response.status()));
    }

    let data: serde_json::Value = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse response: {}", e))?;

    let case: MedicalCase = serde_json::from_value(data["case"].clone())
        .map_err(|e| format!("Failed to deserialize: {}", e))?;

    Ok(case)
}

pub async fn upload_photos(case_id: Uuid, files: Vec<web_sys::File>) -> Result<Vec<Uuid>, String> {
    // TODO: Implement proper multipart upload
    Err("Upload not fully implemented yet".to_string())
}
