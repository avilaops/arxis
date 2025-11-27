use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MedicalCase {
    pub id: Uuid,
    pub patient_id: String,
    pub created_at: DateTime<Utc>,
    pub status: CaseStatus,
    pub photos: Vec<Uuid>,
    pub reconstruction: Option<Reconstruction3D>,
    pub surgical_plan: Option<SurgicalPlan>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CaseStatus {
    Created,
    PhotosUploaded,
    Reconstructed,
    SurgerySimulated,
    Completed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reconstruction3D {
    pub model_id: Uuid,
    pub vertices: Vec<[f32; 3]>,
    pub faces: Vec<[usize; 3]>,
    pub texture_coords: Option<Vec<[f32; 2]>>,
    pub shape_params: Vec<f32>,     // 3DMM alpha
    pub expression_params: Vec<f32>, // 3DMM beta
    pub quality_score: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SurgicalPlan {
    pub plan_id: Uuid,
    pub procedure_type: ProcedureType,
    pub affected_area: BurnArea,
    pub graft_source: Option<String>,
    pub estimated_operations: u32,
    pub notes: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProcedureType {
    SkinGraft,
    TissueExpansion,
    FlapReconstruction,
    LaserResurfacing,
    ScarRevision,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BurnArea {
    pub region: FacialRegion,
    pub severity: BurnSeverity,
    pub area_percentage: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FacialRegion {
    Forehead,
    Cheeks,
    Nose,
    Chin,
    Neck,
    LeftEye,
    RightEye,
    Mouth,
    Ears,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BurnSeverity {
    FirstDegree,
    SecondDegree,
    ThirdDegree,
    FourthDegree,
}

// API Request/Response types

#[derive(Debug, Deserialize)]
pub struct CreateCaseRequest {
    pub patient_id: String,
}

#[derive(Debug, Serialize)]
pub struct CaseResponse {
    pub case: MedicalCase,
}

#[derive(Debug, Serialize)]
pub struct UploadResponse {
    pub photo_ids: Vec<Uuid>,
}

#[derive(Debug, Serialize)]
pub struct ReconstructionResponse {
    pub reconstruction: Reconstruction3D,
}

#[derive(Debug, Deserialize)]
pub struct SurgicalPlanRequest {
    pub procedure_type: ProcedureType,
    pub affected_area: BurnArea,
    #[allow(dead_code)]
    pub graft_source: Option<String>,
    #[allow(dead_code)]
    pub notes: String,
}

#[derive(Debug, Serialize)]
pub struct SimulationResponse {
    pub simulation: SurgicalSimulation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SurgicalSimulation {
    pub simulation_id: Uuid,
    pub predicted_model: Reconstruction3D,
    pub success_probability: f32,
    pub estimated_recovery_weeks: u32,
    pub material_requirements: Vec<MaterialRequirement>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaterialRequirement {
    pub material_type: String,
    pub quantity: String,
    pub notes: String,
}
