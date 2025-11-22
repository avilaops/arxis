//! Autoencoder neural network for dimensionality reduction

use crate::{ReductionError, Result};

/// Autoencoder configuration
pub struct Autoencoder {
    /// Encoding dimensions
    pub encoding_dims: Vec<usize>,
}

impl Autoencoder {
    /// Create new autoencoder builder
    pub fn builder() -> AutoencoderBuilder {
        AutoencoderBuilder::default()
    }
}

/// Builder for Autoencoder
#[derive(Default)]
pub struct AutoencoderBuilder {
    encoding_dims: Vec<usize>,
}

impl AutoencoderBuilder {
    /// Set encoding dimensions
    pub fn encoding_dims(mut self, dims: Vec<usize>) -> Self {
        self.encoding_dims = dims;
        self
    }

    /// Build the autoencoder
    pub fn build(self) -> Result<Autoencoder> {
        if self.encoding_dims.is_empty() {
            return Err(ReductionError::ConfigurationError(
                "Encoding dimensions cannot be empty".to_string(),
            ));
        }
        Ok(Autoencoder {
            encoding_dims: self.encoding_dims,
        })
    }
}
