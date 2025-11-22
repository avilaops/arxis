use super::PostProcessor;
use crate::error::{Result, TokenizerError};

/// Template-based post-processing
/// Supports custom formatting with $A and $B placeholders
#[derive(Debug, Clone)]
pub struct TemplateProcessing {
    single_template: Vec<TemplateToken>,
    pair_template: Vec<TemplateToken>,
}

#[derive(Debug, Clone)]
enum TemplateToken {
    SpecialToken(u32),
    SequenceA,
    SequenceB,
}

impl TemplateProcessing {
    /// Create from template strings
    /// Example: "[CLS]:101 $A [SEP]:102" for single, "[CLS]:101 $A [SEP]:102 $B [SEP]:102" for pair
    pub fn new(single: &str, pair: &str) -> Result<Self> {
        let single_template = Self::parse_template(single)?;
        let pair_template = Self::parse_template(pair)?;

        Ok(Self {
            single_template,
            pair_template,
        })
    }

    fn parse_template(template: &str) -> Result<Vec<TemplateToken>> {
        let mut tokens = Vec::new();

        for part in template.split_whitespace() {
            if part == "$A" {
                tokens.push(TemplateToken::SequenceA);
            } else if part == "$B" {
                tokens.push(TemplateToken::SequenceB);
            } else if part.contains(':') {
                let parts: Vec<&str> = part.split(':').collect();
                if parts.len() == 2 {
                    let id: u32 = parts[1].parse().map_err(|_| {
                        TokenizerError::InvalidConfig(format!("Invalid token ID: {}", parts[1]))
                    })?;
                    tokens.push(TemplateToken::SpecialToken(id));
                }
            }
        }

        Ok(tokens)
    }

    fn apply_template(
        &self,
        template: &[TemplateToken],
        ids: &[u32],
        pair_ids: Option<&[u32]>,
    ) -> Vec<u32> {
        let mut result = Vec::new();

        for token in template {
            match token {
                TemplateToken::SpecialToken(id) => result.push(*id),
                TemplateToken::SequenceA => result.extend_from_slice(ids),
                TemplateToken::SequenceB => {
                    if let Some(pair) = pair_ids {
                        result.extend_from_slice(pair);
                    }
                }
            }
        }

        result
    }
}

impl PostProcessor for TemplateProcessing {
    fn process(&self, ids: Vec<u32>, pair_ids: Option<Vec<u32>>) -> Result<Vec<u32>> {
        let template = if pair_ids.is_some() {
            &self.pair_template
        } else {
            &self.single_template
        };

        Ok(self.apply_template(template, &ids, pair_ids.as_deref()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_template_processing() {
        let processor = TemplateProcessing::new(
            "[CLS]:101 $A [SEP]:102",
            "[CLS]:101 $A [SEP]:102 $B [SEP]:102",
        )
        .unwrap();

        let result = processor.process(vec![1, 2, 3], None).unwrap();
        assert_eq!(result, vec![101, 1, 2, 3, 102]);

        let result = processor.process(vec![1, 2], Some(vec![3, 4])).unwrap();
        assert_eq!(result, vec![101, 1, 2, 102, 3, 4, 102]);
    }
}
