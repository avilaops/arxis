// LinkedIn-specific extractors
// This module contains extractors for LinkedIn company pages and job postings

use super::*;

pub struct LinkedInJobExtractor;

impl DataExtractor for LinkedInJobExtractor {
    type Output = Vec<JobPosting>;

    fn extract(&self, html: &Html) -> ScraperResult<Vec<JobPosting>> {
        let job_selector = Selector::parse("li.jobs-search__results-list, div.job-card-container").unwrap();
        let mut jobs = Vec::new();

        for element in html.select(&job_selector) {
            if let Ok(job) = self.extract_single_job(&element) {
                jobs.push(job);
            }
        }

        Ok(jobs)
    }

    fn validate(&self, jobs: &Vec<JobPosting>) -> bool {
        !jobs.is_empty()
    }
}

impl LinkedInJobExtractor {
    fn extract_single_job(&self, element: &ElementRef) -> ScraperResult<JobPosting> {
        let title_selector = Selector::parse("h3.base-search-card__title, a.job-card-list__title").unwrap();
        let company_selector = Selector::parse("h4.base-search-card__subtitle, a.job-card-container__company-name").unwrap();
        let location_selector = Selector::parse("span.job-search-card__location").unwrap();

        let title = element.select(&title_selector)
            .next()
            .map(|el| el.text().collect::<String>().trim().to_string())
            .ok_or(ScraperError::MissingField("job title".to_string()))?;

        let company = element.select(&company_selector)
            .next()
            .map(|el| el.text().collect::<String>().trim().to_string())
            .unwrap_or_else(|| "Unknown".to_string());

        let location = element.select(&location_selector)
            .next()
            .map(|el| el.text().collect::<String>().trim().to_string())
            .unwrap_or_else(|| "Remote".to_string());

        Ok(JobPosting {
            id: uuid::Uuid::new_v4().to_string(),
            title,
            company,
            location,
            salary_range: None,
            required_skills: vec![],
            experience_years: None,
            remote: false,
            posted_date: Utc::now(),
            description: None,
            source: "LinkedIn".to_string(),
        })
    }
}
