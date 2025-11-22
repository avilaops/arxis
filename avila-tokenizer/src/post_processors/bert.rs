use super::PostProcessor;
use crate::error::Result;

/// BERT-style post-processing
/// Format: [CLS] + A + [SEP] + B + [SEP]
#[derive(Debug, Clone)]
pub struct BertProcessing {
    cls_id: u32,
    sep_id: u32,
}

impl BertProcessing {
    pub fn new(cls_id: u32, sep_id: u32) -> Self {
        Self { cls_id, sep_id }
    }
}

impl PostProcessor for BertProcessing {
    fn process(&self, ids: Vec<u32>, pair_ids: Option<Vec<u32>>) -> Result<Vec<u32>> {
        let mut result = vec![self.cls_id];
        result.extend(ids);
        result.push(self.sep_id);

        if let Some(pair) = pair_ids {
            result.extend(pair);
            result.push(self.sep_id);
        }

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bert_processing_single() {
        let processor = BertProcessing::new(101, 102);
        let result = processor.process(vec![1, 2, 3], None).unwrap();
        assert_eq!(result, vec![101, 1, 2, 3, 102]);
    }

    #[test]
    fn test_bert_processing_pair() {
        let processor = BertProcessing::new(101, 102);
        let result = processor.process(vec![1, 2], Some(vec![3, 4])).unwrap();
        assert_eq!(result, vec![101, 1, 2, 102, 3, 4, 102]);
    }
}
