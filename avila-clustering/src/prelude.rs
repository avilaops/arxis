//! Convenient re-exports for common clustering operations

pub use crate::{ClusteringError, Result};

// Algorithms
pub use crate::algorithms::dbscan::*;
pub use crate::algorithms::fuzzy_cmeans::*;
pub use crate::algorithms::gmm::*;
pub use crate::algorithms::hdbscan::*;
pub use crate::algorithms::hierarchical::*;
pub use crate::algorithms::kmeans::*;
pub use crate::algorithms::kmedoids::*;
pub use crate::algorithms::mean_shift::*;
pub use crate::algorithms::optics::*;
pub use crate::algorithms::spectral::*;
pub use crate::algorithms::streaming::*;

// Metrics
pub use crate::metrics::distance::*;
pub use crate::metrics::manifold::*;
pub use crate::metrics::validation::*;

// Scientific
pub use crate::scientific::curved::*;
pub use crate::scientific::physics::*;
pub use crate::scientific::spacetime::*;

// Common types
pub use ndarray::{Array1, Array2, ArrayView1, ArrayView2};
