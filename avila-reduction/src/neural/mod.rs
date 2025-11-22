//! Neural network-based reduction methods

#[cfg(feature = "neural")]
pub mod autoencoder;
#[cfg(feature = "neural")]
pub mod contractive;
#[cfg(feature = "neural")]
pub mod vae;

#[cfg(not(feature = "neural"))]
pub mod autoencoder {
    use crate::{ReductionError, Result};

    pub struct Autoencoder;

    impl Autoencoder {
        pub fn builder() -> Result<Self> {
            Err(ReductionError::FeatureNotEnabled(
                "Neural network support requires --features neural".to_string(),
            ))
        }
    }
}
