//! FITS I/O - Flexible Image Transport System for astronomy data

use crate::core::DataFrame;
use crate::error::{AvilaError, Result};
use std::path::Path;

impl DataFrame {
    /// Write DataFrame to FITS file
    ///
    /// FITS is the standard format for astronomical data
    ///
    /// # Example
    /// ```no_run
    /// # use avila_dataframe::prelude::*;
    /// let df = DataFrame::new(vec![
    ///     Series::new("ra", vec![10.0, 20.0]),  // Right Ascension
    ///     Series::new("dec", vec![45.0, 46.0]), // Declination
    /// ])?;
    /// df.write_fits("catalog.fits")?;
    /// # Ok::<(), avila_dataframe::error::AvilaError>(())
    /// ```
    pub fn write_fits(&self, _path: impl AsRef<Path>) -> Result<()> {
        #[cfg(feature = "io-fits")]
        {
            // TODO: Implement FITS writing with fits-rs
            Err(AvilaError::not_implemented("write_fits"))
        }
        #[cfg(not(feature = "io-fits"))]
        {
            Err(AvilaError::generic(
                "FITS support not enabled. Enable 'io-fits' feature",
            ))
        }
    }

    /// Read DataFrame from FITS file
    ///
    /// # Example
    /// ```no_run
    /// # use avila_dataframe::prelude::*;
    /// let df = DataFrame::read_fits("catalog.fits")?;
    /// # Ok::<(), avila_dataframe::error::AvilaError>(())
    /// ```
    pub fn read_fits(_path: impl AsRef<Path>) -> Result<Self> {
        #[cfg(feature = "io-fits")]
        {
            // TODO: Implement FITS reading with fits-rs
            Err(AvilaError::not_implemented("read_fits"))
        }
        #[cfg(not(feature = "io-fits"))]
        {
            Err(AvilaError::generic(
                "FITS support not enabled. Enable 'io-fits' feature",
            ))
        }
    }
}
