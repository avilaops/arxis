// ITJobs.pt extractor - Portugal's leading tech job board

use super::*;

pub struct ITJobsExtractor {
    keywords: Vec<String>,
}

impl ITJobsExtractor {
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

    fn extract_single_job(&self, element: &ElementRef) -> ScraperResult<JobPosting> {
        let title_selector = Selector::parse("h2.title a, .job-title a").unwrap();
        let company_selector = Selector::parse(".company, .job-company").unwrap();
        let location_selector = Selector::parse(".location, .job-location").unwrap();
        let description_selector = Selector::parse(".description, .job-description").unwrap();

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
            .unwrap_or_else(|| "Portugal".to_string());

        let description = element.select(&description_selector)
            .next()
            .map(|el| el.text().collect::<String>().trim().to_string());

        // Extract skills from description
        let required_skills = self.extract_skills(description.as_deref().unwrap_or(""));

        // Detect remote work
        let remote = location.to_lowercase().contains("remoto")
            || description.as_ref().map(|d| d.to_lowercase().contains("remoto")).unwrap_or(false);

        Ok(JobPosting {
            id: uuid::Uuid::new_v4().to_string(),
            title,
            company,
            location,
            salary_range: None,
            required_skills,
            experience_years: None,
            remote,
            posted_date: Utc::now(),
            description,
            source: "ITJobs.pt".to_string(),
        })
    }

    fn extract_skills(&self, text: &str) -> Vec<String> {
        let common_skills = vec![
            "Rust", "Python", "Java", "JavaScript", "TypeScript", "Go", "C++", "C#",
            "React", "Vue", "Angular", "Node.js", "Django", "Flask", "Spring",
            "Docker", "Kubernetes", "AWS", "Azure", "GCP",
            "PostgreSQL", "MySQL", "MongoDB", "Redis",
            "Git", "CI/CD", "Agile", "Scrum",
        ];

        let text_lower = text.to_lowercase();
        common_skills
            .into_iter()
            .filter(|skill| text_lower.contains(&skill.to_lowercase()))
            .map(|s| s.to_string())
            .collect()
    }
}

impl DataExtractor for ITJobsExtractor {
    type Output = Vec<JobPosting>;

    fn extract(&self, html: &Html) -> ScraperResult<Vec<JobPosting>> {
        let job_selector = Selector::parse("article.job, .job-item, .offer").unwrap();
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skill_extraction() {
        let extractor = ITJobsExtractor::new(vec![]);
        let text = "We're looking for a Rust developer with experience in Docker and AWS";
        let skills = extractor.extract_skills(text);

        assert!(skills.contains(&"Rust".to_string()));
        assert!(skills.contains(&"Docker".to_string()));
        assert!(skills.contains(&"AWS".to_string()));
    }
}
