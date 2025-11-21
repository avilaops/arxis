//! HDF5 I/O - Hierarchical Data Format for scientific data

#[cfg(feature = "io-hdf5")]
use crate::core::{DataFrame, Series};
#[cfg(feature = "io-hdf5")]
use crate::error::{Result, AvilaError};
#[cfg(feature = "io-hdf5")]
use std::path::Path;

#[cfg(feature = "io-hdf5")]
impl DataFrame {
    /// Write DataFrame to HDF5 file
    ///
    /// # Arguments
    /// * `path` - Path to HDF5 file
    /// * `dataset_name` - Name of dataset within file
    ///
    /// # Example
    /// ```no_run
    /// # #[cfg(feature = "io-hdf5")]
    /// # use avila_dataframe::prelude::*;
    /// # let df = DataFrame::new(vec![Series::new("a", vec![1.0])])?;
    /// df.write_hdf5("data.h5", "measurements")?;
    /// # Ok::<(), avila_dataframe::error::AvilaError>(())
    /// ```
    pub fn write_hdf5(&self, path: impl AsRef<Path>, dataset_name: &str) -> Result<()> {
        use hdf5::File;

        let file = File::create(path.as_ref())?;

        // Write each column as a separate dataset
        for series in &self.columns {
            let dataset_path = format!("{}/{}", dataset_name, series.name());

            // Get values as f64 array
            let values: Result<Vec<f64>> = (0..series.len())
                .map(|i| series.get_f64(i))
                .collect();
            let values = values?;

            // Create dataset
            file.new_dataset::<f64>()
                .shape(values.len())
                .create(&dataset_path)?
                .write(&values)?;
        }

        // Store metadata
        let metadata_path = format!("{}/metadata", dataset_name);
        let group = file.create_group(&metadata_path)?;
        group.new_attr::<i32>()
            .create("num_rows")?
            .write_scalar(&(self.height() as i32))?;
        group.new_attr::<i32>()
            .create("num_columns")?
            .write_scalar(&(self.width() as i32))?;

        Ok(())
    }

    /// Read DataFrame from HDF5 file
    ///
    /// # Example
    /// ```no_run
    /// # #[cfg(feature = "io-hdf5")]
    /// # use avila_dataframe::prelude::*;
    /// let df = DataFrame::read_hdf5("data.h5", "measurements")?;
    /// # Ok::<(), avila_dataframe::error::AvilaError>(())
    /// ```
    pub fn read_hdf5(path: impl AsRef<Path>, dataset_name: &str) -> Result<Self> {
        use hdf5::File;

        let file = File::open(path.as_ref())?;
        let group = file.group(dataset_name)?;

        let mut columns = Vec::new();

        // Read each dataset as a column
        for member_name in group.member_names()? {
            if member_name == "metadata" {
                continue;
            }

            let dataset = group.dataset(&member_name)?;
            let values: Vec<f64> = dataset.read_raw()?;

            columns.push(Series::new(member_name, values));
        }

        DataFrame::new(columns)
    }

    /// List datasets in HDF5 file
    pub fn list_hdf5_datasets(path: impl AsRef<Path>) -> Result<Vec<String>> {
        use hdf5::File;

        let file = File::open(path.as_ref())?;
        Ok(file.member_names()?)
    }
}

#[cfg(not(feature = "io-hdf5"))]
impl DataFrame {
    /// HDF5 support not enabled
    pub fn write_hdf5(&self, _path: impl AsRef<std::path::Path>, _dataset: &str) -> crate::error::Result<()> {
        Err(crate::error::AvilaError::generic(
            "HDF5 support not enabled. Enable 'io-hdf5' feature"
        ))
    }

    /// HDF5 support not enabled
    pub fn read_hdf5(_path: impl AsRef<std::path::Path>, _dataset: &str) -> crate::error::Result<Self> {
        Err(crate::error::AvilaError::generic(
            "HDF5 support not enabled. Enable 'io-hdf5' feature"
        ))
    }
}

#[cfg(test)]
#[cfg(feature = "io-hdf5")]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn test_hdf5_roundtrip() {
        let df = DataFrame::new(vec![
            Series::new("temperature", vec![20.5, 21.0, 19.8]),
            Series::new("pressure", vec![1013.25, 1012.0, 1015.5]),
        ])
        .unwrap();

        let temp_file = NamedTempFile::new().unwrap();

        df.write_hdf5(temp_file.path(), "weather_data").unwrap();
        let df2 = DataFrame::read_hdf5(temp_file.path(), "weather_data").unwrap();

        assert_eq!(df.shape(), df2.shape());
        assert_eq!(df.column_names().len(), df2.column_names().len());
    }
}
