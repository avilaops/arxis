//! Contractive Autoencoder for dimensionality reduction

use crate::{ReductionError, Result};

/// Contractive Autoencoder configuration
pub struct ContractiveAutoencoder {
    /// Encoding dimensions
    pub encoding_dims: Vec<usize>,
    /// Contraction coefficient
    pub lambda: f64,
}

impl ContractiveAutoencoder {
    /// Create new contractive autoencoder builder
    pub fn builder() -> ContractiveBuilder {
        ContractiveBuilder::default()
    }
}

/// Builder for ContractiveAutoencoder
#[derive(Default)]
pub struct ContractiveBuilder {
    encoding_dims: Vec<usize>,
    lambda: f64,
}

impl ContractiveBuilder {
    /// Set encoding dimensions
    pub fn encoding_dims(mut self, dims: Vec<usize>) -> Self {
        self.encoding_dims = dims;
        self
    }

    /// Set contraction coefficient
    pub fn lambda(mut self, lambda: f64) -> Self {
        self.lambda = lambda;
        self
    }

    /// Build the contractive autoencoder
    pub fn build(self) -> Result<ContractiveAutoencoder> {
        if self.encoding_dims.is_empty() {
            return Err(ReductionError::ConfigurationError(
                "Encoding dimensions cannot be empty".to_string(),
            ));
        }
        if self.lambda <= 0.0 {
            return Err(ReductionError::ConfigurationError(
                "Lambda must be positive".to_string(),
            ));
        }
        Ok(ContractiveAutoencoder {
            encoding_dims: self.encoding_dims,
            lambda: self.lambda,
        })
    }
}
