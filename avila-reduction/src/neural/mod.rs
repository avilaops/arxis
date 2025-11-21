//! Neural network-based reduction methods

#[cfg(feature = "neural")]
pub mod autoencoder;
#[cfg(feature = "neural")]
pub mod vae;
#[cfg(feature = "neural")]
pub mod contractive;

#[cfg(not(feature = "neural"))]
pub mod autoencoder {
    use crate::{Result, ReductionError};

    pub struct Autoencoder;

    impl Autoencoder {
        pub fn builder() -> Result<Self> {
            Err(ReductionError::FeatureNotEnabled(
                "Neural network support requires --features neural".to_string()
            ))
        }
    }
}
