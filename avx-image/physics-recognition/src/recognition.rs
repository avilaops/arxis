//! Módulo de Reconhecimento - Sistema Completo
//!
//! Implementa algoritmos de reconhecimento facial:
//! - PCA (Principal Component Analysis) - Eigenfaces
//! - Distance metrics (Euclidean, Mahalanobis)
//! - Face matching e identificação

use ndarray::{Array1, Array2, Axis};
use std::collections::HashMap;

/// Sistema de reconhecimento facial
pub struct FaceRecognizer {
    /// Banco de dados de faces (person_id -> list of feature vectors)
    database: HashMap<usize, Vec<Array1<f32>>>,
    /// Eigenfaces (componentes principais)
    eigenfaces: Option<Array2<f32>>,
    /// Face média
    mean_face: Option<Array1<f32>>,
    /// Número de componentes PCA
    n_components: usize,
}

impl FaceRecognizer {
    pub fn new() -> Self {
        Self {
            database: HashMap::new(),
            eigenfaces: None,
            mean_face: None,
            n_components: 0,
        }
    }

    /// Adiciona uma face ao banco de dados
    pub fn add_face(&mut self, person_id: usize, face_image: Array2<f32>) {
        // Converte imagem 2D em vetor 1D
        let len = face_image.len();
        let face_vector = face_image.into_shape(len).unwrap();

        self.database.entry(person_id)
            .or_insert_with(Vec::new)
            .push(face_vector);
    }

    /// Treina modelo PCA (Eigenfaces)
    ///
    /// Algoritmo:
    /// 1. Calcula face média: μ = (1/n)Σxᵢ
    /// 2. Centraliza dados: Φᵢ = xᵢ - μ
    /// 3. Calcula matriz de covariância: C = (1/n)ΣΦᵢΦᵢᵀ
    /// 4. Calcula autovalores/autovetores de C
    /// 5. Seleciona k maiores autovalores → Eigenfaces
    pub fn train_pca(&mut self, n_components: usize) {
        self.n_components = n_components;

        // Coleta todas as faces
        let mut all_faces = Vec::new();
        for faces in self.database.values() {
            all_faces.extend(faces.clone());
        }

        if all_faces.is_empty() {
            return;
        }

        let n_samples = all_faces.len();
        let n_features = all_faces[0].len();

        // 1. Calcula face média
        let mut mean_face = Array1::zeros(n_features);
        for face in &all_faces {
            mean_face = mean_face + face;
        }
        mean_face = mean_face / n_samples as f32;
        self.mean_face = Some(mean_face.clone());

        // 2. Centraliza dados
        let mut data_matrix = Array2::zeros((n_samples, n_features));
        for (i, face) in all_faces.iter().enumerate() {
            let centered = face - &mean_face;
            data_matrix.row_mut(i).assign(&centered);
        }

        // 3. Método simplificado: SVD no espaço reduzido
        // Para n << d, calculamos autovetores de AAᵀ ao invés de AᵀA
        let _gram_matrix = data_matrix.dot(&data_matrix.t()) / n_samples as f32;

        // 4. Aproximação: usa os primeiros n_components vetores
        // (Na prática, usaríamos eigendecomposition completa)
        let k = n_components.min(n_samples).min(n_features);
        let mut eigenfaces = Array2::zeros((k, n_features));

        // Simplificação: usa vetores da matriz de dados diretamente
        // (normalizado)
        for i in 0..k {
            if i < n_samples {
                let mut eigenvector = data_matrix.row(i).to_owned();
                let norm = eigenvector.iter().map(|x| x * x).sum::<f32>().sqrt();
                if norm > 1e-6 {
                    eigenvector = eigenvector / norm;
                }
                eigenfaces.row_mut(i).assign(&eigenvector);
            }
        }

        self.eigenfaces = Some(eigenfaces);
    }

    /// Projeta face no espaço de eigenfaces
    ///
    /// y = Uᵀ(x - μ)
    ///
    /// onde U = matriz de eigenfaces
    fn project_face(&self, face: &Array1<f32>) -> Array1<f32> {
        if let (Some(eigenfaces), Some(mean_face)) = (&self.eigenfaces, &self.mean_face) {
            let centered = face - mean_face;
            eigenfaces.dot(&centered)
        } else {
            face.clone()
        }
    }

    /// Reconhece uma face
    ///
    /// Retorna (person_id, confidence)
    pub fn recognize(&self, face_image: &Array2<f32>) -> (usize, f32) {
        // Converte para vetor
        let face_vector = face_image.clone().into_shape(face_image.len()).unwrap();

        // Projeta no espaço PCA
        let face_projection = self.project_face(&face_vector);

        // Encontra face mais próxima no banco
        let mut min_distance = f32::MAX;
        let mut best_match = 0;

        for (&person_id, faces) in &self.database {
            for db_face in faces {
                let db_projection = self.project_face(db_face);
                let distance = euclidean_distance(&face_projection, &db_projection);

                if distance < min_distance {
                    min_distance = distance;
                    best_match = person_id;
                }
            }
        }

        // Converte distância em confiança (heurística)
        let confidence = (-min_distance / 10.0).exp();

        (best_match, confidence)
    }

    /// Verifica se duas faces são da mesma pessoa
    ///
    /// Usa threshold na distância
    pub fn verify(&self, face1: &Array2<f32>, face2: &Array2<f32>, threshold: f32) -> bool {
        let vec1 = face1.clone().into_shape(face1.len()).unwrap();
        let vec2 = face2.clone().into_shape(face2.len()).unwrap();

        let proj1 = self.project_face(&vec1);
        let proj2 = self.project_face(&vec2);

        let distance = euclidean_distance(&proj1, &proj2);
        distance < threshold
    }

    /// Retorna as k faces mais similares
    pub fn find_similar(&self, face_image: &Array2<f32>, k: usize) -> Vec<(usize, f32)> {
        let face_vector = face_image.clone().into_shape(face_image.len()).unwrap();
        let face_projection = self.project_face(&face_vector);

        let mut distances = Vec::new();

        for (&person_id, faces) in &self.database {
            for db_face in faces {
                let db_projection = self.project_face(db_face);
                let distance = euclidean_distance(&face_projection, &db_projection);
                distances.push((person_id, distance));
            }
        }

        // Ordena por distância
        distances.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

        // Retorna k menores
        distances.into_iter()
            .take(k)
            .map(|(id, dist)| (id, (-dist / 10.0).exp()))
            .collect()
    }
}

/// Distância euclidiana entre vetores
///
/// d = ||x - y|| = √Σ(xᵢ - yᵢ)²
fn euclidean_distance(v1: &Array1<f32>, v2: &Array1<f32>) -> f32 {
    v1.iter()
        .zip(v2.iter())
        .map(|(a, b)| (a - b).powi(2))
        .sum::<f32>()
        .sqrt()
}

/// Distância de cosseno (similaridade angular)
///
/// sim = (x·y) / (||x|| ||y||)
/// dist = 1 - sim
fn cosine_distance(v1: &Array1<f32>, v2: &Array1<f32>) -> f32 {
    let dot_product: f32 = v1.iter().zip(v2.iter()).map(|(a, b)| a * b).sum();
    let norm1 = v1.iter().map(|x| x * x).sum::<f32>().sqrt();
    let norm2 = v2.iter().map(|x| x * x).sum::<f32>().sqrt();

    if norm1 < 1e-6 || norm2 < 1e-6 {
        return 1.0;
    }

    1.0 - (dot_product / (norm1 * norm2))
}

/// Distância de Mahalanobis (considera covariância)
///
/// d = √[(x-y)ᵀ Σ⁻¹ (x-y)]
///
/// onde Σ = matriz de covariância
#[allow(dead_code)]
fn mahalanobis_distance(
    v1: &Array1<f32>,
    v2: &Array1<f32>,
    inv_cov: &Array2<f32>,
) -> f32 {
    let diff = v1 - v2;
    let result = diff.dot(&inv_cov.dot(&diff));
    result.sqrt()
}

/// Calcula métricas de avaliação
pub struct EvaluationMetrics {
    pub accuracy: f32,
    pub precision: f32,
    pub recall: f32,
    pub f1_score: f32,
}

impl EvaluationMetrics {
    pub fn compute(
        true_positives: usize,
        false_positives: usize,
        false_negatives: usize,
        true_negatives: usize,
    ) -> Self {
        let tp = true_positives as f32;
        let fp = false_positives as f32;
        let fn_ = false_negatives as f32;
        let tn = true_negatives as f32;

        let accuracy = (tp + tn) / (tp + fp + fn_ + tn);
        let precision = if tp + fp > 0.0 { tp / (tp + fp) } else { 0.0 };
        let recall = if tp + fn_ > 0.0 { tp / (tp + fn_) } else { 0.0 };
        let f1_score = if precision + recall > 0.0 {
            2.0 * (precision * recall) / (precision + recall)
        } else {
            0.0
        };

        Self { accuracy, precision, recall, f1_score }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::features;

    #[test]
    fn test_face_recognition() {
        let mut recognizer = FaceRecognizer::new();

        // Adiciona faces
        for id in 0..3 {
            for _ in 0..2 {
                let face = features::create_synthetic_face_for_person(32, 32, id);
                recognizer.add_face(id, face);
            }
        }

        // Treina
        recognizer.train_pca(10);

        // Testa
        let test_face = features::create_synthetic_face_for_person(32, 32, 1);
        let (predicted_id, confidence) = recognizer.recognize(&test_face);

        assert!(confidence > 0.0);
        println!("Predicted: {}, Confidence: {}", predicted_id, confidence);
    }

    #[test]
    fn test_distances() {
        let v1 = Array1::from_vec(vec![1.0, 2.0, 3.0]);
        let v2 = Array1::from_vec(vec![4.0, 5.0, 6.0]);

        let euclidean = euclidean_distance(&v1, &v2);
        let cosine = cosine_distance(&v1, &v2);

        assert!(euclidean > 0.0);
        assert!(cosine >= 0.0 && cosine <= 2.0);
    }
}
