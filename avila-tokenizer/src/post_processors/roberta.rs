use super::PostProcessor;
use crate::error::Result;

/// RoBERTa-style post-processing
/// Format: <s> + A + </s> + </s> + B + </s>
#[derive(Debug, Clone)]
pub struct RobertaProcessing {
    bos_id: u32,
    eos_id: u32,
    add_prefix_space: bool,
}

impl RobertaProcessing {
    pub fn new(bos_id: u32, eos_id: u32) -> Self {
        Self {
            bos_id,
            eos_id,
            add_prefix_space: true,
        }
    }

    pub fn with_prefix_space(mut self, add_prefix_space: bool) -> Self {
        self.add_prefix_space = add_prefix_space;
        self
    }
}

impl PostProcessor for RobertaProcessing {
    fn process(&self, ids: Vec<u32>, pair_ids: Option<Vec<u32>>) -> Result<Vec<u32>> {
        let mut result = vec![self.bos_id];
        result.extend(ids);
        result.push(self.eos_id);

        if let Some(pair) = pair_ids {
            result.push(self.eos_id); // Extra EOS for pair
            result.extend(pair);
            result.push(self.eos_id);
        }

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roberta_processing_single() {
        let processor = RobertaProcessing::new(0, 2);
        let result = processor.process(vec![1, 2, 3], None).unwrap();
        assert_eq!(result, vec![0, 1, 2, 3, 2]);
    }

    #[test]
    fn test_roberta_processing_pair() {
        let processor = RobertaProcessing::new(0, 2);
        let result = processor.process(vec![1, 2], Some(vec![3, 4])).unwrap();
        assert_eq!(result, vec![0, 1, 2, 2, 2, 3, 4, 2]);
    }
}
