// Google Maps / Google Places API extractor

use super::*;
use crate::types::Place;

pub struct GoogleMapsExtractor;

impl DataExtractor for GoogleMapsExtractor {
    type Output = Vec<Place>;

    fn extract(&self, html: &Html) -> ScraperResult<Vec<Place>> {
        let place_selector = Selector::parse("div.place-card, div[role='article']").unwrap();
        let mut places = Vec::new();

        for element in html.select(&place_selector) {
            if let Ok(place) = self.extract_single_place(&element) {
                places.push(place);
            }
        }

        Ok(places)
    }

    fn validate(&self, places: &Vec<Place>) -> bool {
        !places.is_empty()
    }
}

impl GoogleMapsExtractor {
    fn extract_single_place(&self, element: &ElementRef) -> ScraperResult<Place> {
        let name_selector = Selector::parse("h2.fontHeadlineSmall, div.fontHeadlineSmall").unwrap();
        let address_selector = Selector::parse("div[data-item-id='address'], button[data-item-id='address']").unwrap();
        let rating_selector = Selector::parse("span.ceNzKf, div.fontDisplayLarge").unwrap();

        let name = element.select(&name_selector)
            .next()
            .map(|el| el.text().collect::<String>().trim().to_string())
            .ok_or(ScraperError::MissingField("place name".to_string()))?;

        let address = element.select(&address_selector)
            .next()
            .map(|el| el.text().collect::<String>().trim().to_string())
            .unwrap_or_else(|| "Unknown".to_string());

        let rating_text = element.select(&rating_selector)
            .next()
            .map(|el| el.text().collect::<String>().trim().to_string());

        let rating = rating_text.and_then(|r| r.parse::<f32>().ok());

        Ok(Place {
            place_id: uuid::Uuid::new_v4().to_string(),
            name,
            address,
            phone: None,
            website: None,
            rating,
            reviews_count: 0,
            business_type: vec![],
            coordinates: (0.0, 0.0), // Would need to extract from data attributes
            source: "Google Maps".to_string(),
        })
    }

    /// Extract place details from Google Places API JSON response
    pub fn from_api_response(&self, json: &str) -> ScraperResult<Vec<Place>> {
        use serde_json::Value;

        let data: Value = serde_json::from_str(json)
            .map_err(|e| ScraperError::ParseError(e.to_string()))?;

        let results = data["results"].as_array()
            .ok_or(ScraperError::ParseError("No results array".to_string()))?;

        let mut places = Vec::new();

        for result in results {
            let place = Place {
                place_id: result["place_id"].as_str().unwrap_or("").to_string(),
                name: result["name"].as_str().unwrap_or("").to_string(),
                address: result["formatted_address"].as_str().unwrap_or("").to_string(),
                phone: result["formatted_phone_number"].as_str().map(|s| s.to_string()),
                website: result["website"].as_str().map(|s| s.to_string()),
                rating: result["rating"].as_f64().map(|r| r as f32),
                reviews_count: result["user_ratings_total"].as_u64().unwrap_or(0) as u32,
                business_type: result["types"]
                    .as_array()
                    .map(|arr| arr.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect())
                    .unwrap_or_default(),
                coordinates: (
                    result["geometry"]["location"]["lat"].as_f64().unwrap_or(0.0),
                    result["geometry"]["location"]["lng"].as_f64().unwrap_or(0.0),
                ),
                source: "Google Places API".to_string(),
            };

            places.push(place);
        }

        Ok(places)
    }
}
