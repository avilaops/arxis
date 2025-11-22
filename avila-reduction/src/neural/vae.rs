//! Variational Autoencoder (VAE) for dimensionality reduction

use crate::{ReductionError, Result};

/// Variational Autoencoder configuration
pub struct VAE {
    /// Latent dimensions
    pub latent_dims: usize,
}

impl VAE {
    /// Create new VAE builder
    pub fn builder() -> VAEBuilder {
        VAEBuilder::default()
    }
}

/// Builder for VAE
#[derive(Default)]
pub struct VAEBuilder {
    latent_dims: usize,
}

impl VAEBuilder {
    /// Set latent dimensions
    pub fn latent_dims(mut self, dims: usize) -> Self {
        self.latent_dims = dims;
        self
    }

    /// Build the VAE
    pub fn build(self) -> Result<VAE> {
        if self.latent_dims == 0 {
            return Err(ReductionError::ConfigurationError(
                "Latent dimensions must be greater than 0".to_string(),
            ));
        }
        Ok(VAE {
            latent_dims: self.latent_dims,
        })
    }
}
