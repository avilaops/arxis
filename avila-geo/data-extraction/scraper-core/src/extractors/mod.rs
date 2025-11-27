use crate::types::{CompanyInfo, JobPosting, ScraperError, ScraperResult};
use scraper::{Html, Selector, ElementRef};
use chrono::Utc;

pub trait DataExtractor {
    type Output;

    fn extract(&self, html: &Html) -> ScraperResult<Self::Output>;
    fn validate(&self, data: &Self::Output) -> bool;
}

pub mod linkedin;
pub mod itjobs;
pub mod google_maps;

pub use linkedin::LinkedInCompanyExtractor;
pub use itjobs::ITJobsExtractor;
pub use google_maps::GoogleMapsExtractor;

/// Extract company info from LinkedIn
pub struct LinkedInCompanyExtractor;

impl DataExtractor for LinkedInCompanyExtractor {
    type Output = CompanyInfo;

    fn extract(&self, html: &Html) -> ScraperResult<CompanyInfo> {
        let name = self.extract_name(html)?;
        let size = self.extract_size(html)?;
        let industry = self.extract_industry(html)?;
        let website = self.extract_website(html);
        let description = self.extract_description(html);

        Ok(CompanyInfo {
            id: uuid::Uuid::new_v4().to_string(),
            name: name.clone(),
            industry,
            size: crate::types::CompanySize::from_text(&size),
            employees: crate::types::CompanySize::from_text(&size).mid_point(),
            location: self.extract_location(html)?,
            website,
            description,
            founded_year: None,
            source: "LinkedIn".to_string(),
            scraped_at: Utc::now(),
        })
    }

    fn validate(&self, data: &CompanyInfo) -> bool {
        !data.name.is_empty() && data.employees > 0
    }
}

impl LinkedInCompanyExtractor {
    fn extract_name(&self, html: &Html) -> ScraperResult<String> {
        let selector = Selector::parse("h1.org-top-card-summary__title, h1.top-card-layout__title").unwrap();

        html.select(&selector)
            .next()
            .map(|el| el.text().collect::<String>().trim().to_string())
            .ok_or(ScraperError::MissingField("company name".to_string()))
    }

    fn extract_size(&self, html: &Html) -> ScraperResult<String> {
        let selector = Selector::parse(
            "dd.org-about-company-module__company-size-definition-text, \
             div.org-top-card-summary-info-list__info-item:contains('employees')"
        ).unwrap();

        html.select(&selector)
            .next()
            .map(|el| el.text().collect::<String>().trim().to_string())
            .ok_or(ScraperError::MissingField("company size".to_string()))
    }

    fn extract_industry(&self, html: &Html) -> ScraperResult<String> {
        let selector = Selector::parse("dd.org-about-company-module__industry").unwrap();

        Ok(html.select(&selector)
            .next()
            .map(|el| el.text().collect::<String>().trim().to_string())
            .unwrap_or_default())
    }

    fn extract_location(&self, html: &Html) -> ScraperResult<String> {
        let selector = Selector::parse("div.org-top-card-summary-info-list__info-item:contains('headquarters')").unwrap();

        Ok(html.select(&selector)
            .next()
            .map(|el| el.text().collect::<String>().trim().to_string())
            .unwrap_or_else(|| "Unknown".to_string()))
    }

    fn extract_website(&self, html: &Html) -> Option<String> {
        let selector = Selector::parse("a.org-about-us-company-module__website").unwrap();

        html.select(&selector)
            .next()
            .and_then(|el| el.value().attr("href"))
            .map(|s| s.to_string())
    }

    fn extract_description(&self, html: &Html) -> Option<String> {
        let selector = Selector::parse("p.org-about-us-organization-description__text").unwrap();

        html.select(&selector)
            .next()
            .map(|el| el.text().collect::<String>().trim().to_string())
    }
}

/// Extract job postings
pub struct JobPostingExtractor {
    keywords: Vec<String>,
}

impl JobPostingExtractor {
    pub fn new(keywords: Vec<String>) -> Self {
        Self { keywords }
    }

    fn matches_keywords(&self, job: &JobPosting) -> bool {
        if self.keywords.is_empty() {
            return true;
        }

        let text = format!(
            "{} {} {}",
            job.title.to_lowercase(),
            job.description.as_ref().unwrap_or(&String::new()).to_lowercase(),
            job.required_skills.join(" ").to_lowercase()
        );

        self.keywords.iter().any(|kw| text.contains(&kw.to_lowercase()))
    }
}

impl DataExtractor for JobPostingExtractor {
    type Output = Vec<JobPosting>;

    fn extract(&self, html: &Html) -> ScraperResult<Vec<JobPosting>> {
        let job_selector = Selector::parse(".job-card, .job-listing, article").unwrap();
        let mut jobs = Vec::new();

        for element in html.select(&job_selector) {
            if let Ok(job) = self.extract_single_job(&element) {
                if self.matches_keywords(&job) {
                    jobs.push(job);
                }
            }
        }

        Ok(jobs)
    }

    fn validate(&self, jobs: &Vec<JobPosting>) -> bool {
        !jobs.is_empty()
    }
}

impl JobPostingExtractor {
    fn extract_single_job(&self, element: &ElementRef) -> ScraperResult<JobPosting> {
        // Extract job details from element
        // This is a simplified version - real implementation would be more robust

        Ok(JobPosting {
            id: uuid::Uuid::new_v4().to_string(),
            title: "Example Job".to_string(),
            company: "Example Company".to_string(),
            location: "Remote".to_string(),
            salary_range: None,
            required_skills: vec![],
            experience_years: None,
            remote: true,
            posted_date: Utc::now(),
            description: None,
            source: "Generic".to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_company_size_parsing() {
        use crate::types::CompanySize;

        let size = CompanySize::from_text("51-200 employees");
        assert!(matches!(size, CompanySize::Medium));

        let size2 = CompanySize::from_text("1000+ employees");
        assert!(matches!(size2, CompanySize::Enterprise));
    }
}
