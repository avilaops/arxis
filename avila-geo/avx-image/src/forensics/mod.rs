//! Forensics module (fingerprints, documents, analysis)

pub mod document;
pub mod fingerprint;

use crate::core::ImageBuffer;
use crate::Result;

/// Analyze fingerprint
pub fn analyze_fingerprint(img: &ImageBuffer) -> Result<FingerprintAnalysis> {
    fingerprint::analyze(img)
}

/// Verify document authenticity
pub fn verify_document(img: &ImageBuffer) -> Result<DocumentVerification> {
    document::verify(img)
}

#[derive(Debug, Clone)]
pub struct FingerprintAnalysis {
    pub minutiae: Vec<Minutia>,
    pub pattern_type: PatternType,
    pub quality_score: f32,
}

#[derive(Debug, Clone)]
pub struct Minutia {
    pub x: f32,
    pub y: f32,
    pub angle: f32,
    pub minutia_type: MinutiaType,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MinutiaType {
    Ending,
    Bifurcation,
    Lake,
    Ridge,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PatternType {
    Arch,
    Loop,
    Whorl,
    Composite,
}

#[derive(Debug, Clone)]
pub struct DocumentVerification {
    pub is_authentic: bool,
    pub confidence: f32,
    pub forgery_detected: Vec<ForgeryType>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ForgeryType {
    Copied,
    Spliced,
    Retouched,
    AIGenerated,
}
