//! Geocoding - Address to Coordinates
//!
//! Converte endereços em coordenadas geográficas.
//! Sistema de geocoding otimizado para Brasil com suporte global.

use crate::coords::GeoCoord;
use crate::spatial_native::RTreeIndex;
use std::collections::HashMap;

/// Resultado de geocoding
#[derive(Debug, Clone)]
pub struct GeocodingResult {
    pub coordinate: GeoCoord,
    pub address: String,
    pub confidence: f64,
    pub match_type: MatchType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MatchType {
    Exact,
    Partial,
    Interpolated,
    Approximate,
}

/// Geocoder engine
pub struct Geocoder {
    index: RTreeIndex,
    addresses: Vec<AddressRecord>,
    fuzzy_threshold: f64,
}

#[derive(Debug, Clone)]
struct AddressRecord {
    id: usize,
    coordinate: GeoCoord,
    street: String,
    number: Option<u32>,
    neighborhood: String,
    city: String,
    state: String,
    postal_code: Option<String>,
    country: String,
}

impl Geocoder {
    /// Cria novo geocoder
    pub fn new() -> Self {
        Self {
            index: RTreeIndex::new(),
            addresses: Vec::new(),
            fuzzy_threshold: 0.75,
        }
    }

    /// Adiciona endereço ao índice
    pub fn add_address(
        &mut self,
        coord: GeoCoord,
        street: String,
        number: Option<u32>,
        neighborhood: String,
        city: String,
        state: String,
        postal_code: Option<String>,
        country: String,
    ) {
        let id = self.addresses.len();
        self.index.insert(id, coord);

        self.addresses.push(AddressRecord {
            id,
            coordinate: coord,
            street,
            number,
            neighborhood,
            city,
            state,
            postal_code,
            country,
        });
    }

    /// Geocode um endereço
    pub fn geocode(&self, address: &str) -> Option<GeocodingResult> {
        let parsed = Self::parse_address(address);

        // Try exact match first
        if let Some(result) = self.exact_match(&parsed) {
            return Some(result);
        }

        // Try partial match
        if let Some(result) = self.partial_match(&parsed) {
            return Some(result);
        }

        // Try city/state match
        self.city_match(&parsed)
    }

    /// Geocode múltiplos endereços em lote
    pub fn batch_geocode(&self, addresses: &[String]) -> Vec<Option<GeocodingResult>> {
        addresses.iter()
            .map(|addr| self.geocode(addr))
            .collect()
    }

    /// Busca por CEP brasileiro
    pub fn geocode_cep(&self, cep: &str) -> Option<GeocodingResult> {
        let normalized = Self::normalize_cep(cep);

        for record in &self.addresses {
            if let Some(ref postal) = record.postal_code {
                if postal == &normalized {
                    return Some(GeocodingResult {
                        coordinate: record.coordinate,
                        address: self.format_address(record),
                        confidence: 1.0,
                        match_type: MatchType::Exact,
                    });
                }
            }
        }

        None
    }

    // === Private Methods ===

    fn parse_address(address: &str) -> HashMap<String, String> {
        let mut parts = HashMap::new();
        let normalized = address.to_lowercase();

        // Simple parser (em produção, usar regex ou NLP)
        if let Some((street, rest)) = normalized.split_once(',') {
            parts.insert("street".to_string(), street.trim().to_string());

            for part in rest.split(',') {
                let trimmed = part.trim();
                if trimmed.len() == 2 {
                    parts.insert("state".to_string(), trimmed.to_uppercase());
                } else if trimmed.chars().all(|c| c.is_numeric() || c == '-') {
                    parts.insert("cep".to_string(), trimmed.to_string());
                } else {
                    parts.insert("city".to_string(), trimmed.to_string());
                }
            }
        } else {
            parts.insert("query".to_string(), normalized);
        }

        parts
    }

    fn exact_match(&self, parsed: &HashMap<String, String>) -> Option<GeocodingResult> {
        if let Some(street) = parsed.get("street") {
            for record in &self.addresses {
                if record.street.to_lowercase() == *street {
                    if let Some(city) = parsed.get("city") {
                        if record.city.to_lowercase() == *city {
                            return Some(GeocodingResult {
                                coordinate: record.coordinate,
                                address: self.format_address(record),
                                confidence: 1.0,
                                match_type: MatchType::Exact,
                            });
                        }
                    }
                }
            }
        }
        None
    }

    fn partial_match(&self, parsed: &HashMap<String, String>) -> Option<GeocodingResult> {
        if let Some(query) = parsed.get("query") {
            let mut best_match: Option<(usize, f64)> = None;

            for (idx, record) in self.addresses.iter().enumerate() {
                let full_addr = self.format_address(record).to_lowercase();
                let similarity = Self::string_similarity(&full_addr, query);

                if similarity >= self.fuzzy_threshold {
                    if let Some((_, best_sim)) = best_match {
                        if similarity > best_sim {
                            best_match = Some((idx, similarity));
                        }
                    } else {
                        best_match = Some((idx, similarity));
                    }
                }
            }

            if let Some((idx, confidence)) = best_match {
                let record = &self.addresses[idx];
                return Some(GeocodingResult {
                    coordinate: record.coordinate,
                    address: self.format_address(record),
                    confidence,
                    match_type: MatchType::Partial,
                });
            }
        }
        None
    }

    fn city_match(&self, parsed: &HashMap<String, String>) -> Option<GeocodingResult> {
        if let Some(city) = parsed.get("city") {
            for record in &self.addresses {
                if record.city.to_lowercase() == *city {
                    return Some(GeocodingResult {
                        coordinate: record.coordinate,
                        address: format!("{}, {}", record.city, record.state),
                        confidence: 0.6,
                        match_type: MatchType::Approximate,
                    });
                }
            }
        }
        None
    }

    fn format_address(&self, record: &AddressRecord) -> String {
        let mut parts = vec![record.street.clone()];

        if let Some(num) = record.number {
            parts.push(num.to_string());
        }

        if !record.neighborhood.is_empty() {
            parts.push(record.neighborhood.clone());
        }

        parts.push(format!("{}, {}", record.city, record.state));

        if let Some(ref cep) = record.postal_code {
            parts.push(cep.clone());
        }

        parts.join(", ")
    }

    fn normalize_cep(cep: &str) -> String {
        cep.chars()
            .filter(|c| c.is_numeric())
            .collect()
    }

    fn string_similarity(s1: &str, s2: &str) -> f64 {
        // Levenshtein distance simplificado
        let len1 = s1.len();
        let len2 = s2.len();

        if len1 == 0 {
            return if len2 == 0 { 1.0 } else { 0.0 };
        }
        if len2 == 0 {
            return 0.0;
        }

        let max_dist = len1.max(len2);
        let distance = Self::levenshtein_distance(s1, s2);

        1.0 - (distance as f64 / max_dist as f64)
    }

    fn levenshtein_distance(s1: &str, s2: &str) -> usize {
        let len1 = s1.len();
        let len2 = s2.len();
        let mut matrix = vec![vec![0; len2 + 1]; len1 + 1];

        for i in 0..=len1 {
            matrix[i][0] = i;
        }
        for j in 0..=len2 {
            matrix[0][j] = j;
        }

        let s1_chars: Vec<char> = s1.chars().collect();
        let s2_chars: Vec<char> = s2.chars().collect();

        for i in 1..=len1 {
            for j in 1..=len2 {
                let cost = if s1_chars[i - 1] == s2_chars[j - 1] { 0 } else { 1 };
                matrix[i][j] = (matrix[i - 1][j] + 1)
                    .min(matrix[i][j - 1] + 1)
                    .min(matrix[i - 1][j - 1] + cost);
            }
        }

        matrix[len1][len2]
    }
}

impl Default for Geocoder {
    fn default() -> Self {
        Self::new()
    }
}

/// Carrega endereços brasileiros (exemplo com capitais)
pub fn load_brazilian_capitals() -> Geocoder {
    let mut geocoder = Geocoder::new();

    let capitals = vec![
        ("São Paulo", "SP", -23.55, -46.63),
        ("Rio de Janeiro", "RJ", -22.91, -43.17),
        ("Brasília", "DF", -15.79, -47.88),
        ("Salvador", "BA", -12.97, -38.51),
        ("Fortaleza", "CE", -3.71, -38.54),
        ("Belo Horizonte", "MG", -19.92, -43.94),
        ("Manaus", "AM", -3.11, -60.02),
        ("Curitiba", "PR", -25.43, -49.27),
        ("Recife", "PE", -8.05, -34.88),
        ("Porto Alegre", "RS", -30.03, -51.23),
    ];

    for (city, state, lat, lon) in capitals {
        geocoder.add_address(
            GeoCoord::new(lat, lon),
            "Centro".to_string(),
            None,
            "Centro".to_string(),
            city.to_string(),
            state.to_string(),
            None,
            "Brasil".to_string(),
        );
    }

    geocoder
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_geocode_capital() {
        let geocoder = load_brazilian_capitals();
        let result = geocoder.geocode("São Paulo, SP").unwrap();

        assert_eq!(result.match_type, MatchType::Approximate);
        assert!((result.coordinate.lat + 23.55).abs() < 0.1);
    }

    #[test]
    fn test_batch_geocode() {
        let geocoder = load_brazilian_capitals();
        let addresses = vec![
            "São Paulo, SP".to_string(),
            "Rio de Janeiro, RJ".to_string(),
            "Endereço Inexistente".to_string(),
        ];

        let results = geocoder.batch_geocode(&addresses);
        assert_eq!(results.len(), 3);
        assert!(results[0].is_some());
        assert!(results[1].is_some());
        assert!(results[2].is_none());
    }
}
