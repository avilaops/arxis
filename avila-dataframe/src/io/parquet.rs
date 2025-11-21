//! Parquet I/O - High-performance columnar storage

use crate::core::{DataFrame, Series};
use crate::error::{Result, AvilaError};
use std::path::Path;
use std::fs::File;
use std::sync::Arc;
use parquet::file::properties::WriterProperties;
use parquet::arrow::arrow_reader::ParquetRecordBatchReaderBuilder;
use parquet::arrow::ArrowWriter;
use arrow::datatypes::Schema as ArrowSchema;

/// Parquet compression codec
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Compression {
    /// No compression
    Uncompressed,
    /// Snappy compression (fast)
    Snappy,
    /// Gzip compression (good ratio)
    Gzip,
    /// LZ4 compression (very fast)
    Lz4,
    /// Zstd compression (best ratio)
    Zstd,
}

impl From<Compression> for parquet::basic::Compression {
    fn from(compression: Compression) -> Self {
        match compression {
            Compression::Uncompressed => parquet::basic::Compression::UNCOMPRESSED,
            Compression::Snappy => parquet::basic::Compression::SNAPPY,
            Compression::Gzip => parquet::basic::Compression::GZIP(Default::default()),
            Compression::Lz4 => parquet::basic::Compression::LZ4,
            Compression::Zstd => parquet::basic::Compression::ZSTD(Default::default()),
        }
    }
}

/// Parquet write options
pub struct ParquetWriteOptions {
    /// Compression codec
    pub compression: Compression,
    /// Row group size (number of rows per group)
    pub row_group_size: Option<usize>,
    /// Enable statistics
    pub statistics: bool,
}

impl Default for ParquetWriteOptions {
    fn default() -> Self {
        Self {
            compression: Compression::Zstd,
            row_group_size: Some(100_000),
            statistics: true,
        }
    }
}

impl DataFrame {
    /// Write DataFrame to Parquet file
    ///
    /// # Arguments
    /// * `path` - Path to output file
    ///
    /// # Example
    /// ```no_run
    /// # use avila_dataframe::prelude::*;
    /// let df = DataFrame::new(vec![
    ///     Series::new("a", vec![1.0, 2.0, 3.0]),
    /// ])?;
    /// df.write_parquet("data.parquet")?;
    /// # Ok::<(), avila_dataframe::error::AvilaError>(())
    /// ```
    pub fn write_parquet(&self, path: impl AsRef<Path>) -> Result<()> {
        self.write_parquet_with_options(path, ParquetWriteOptions::default())
    }

    /// Write DataFrame to Parquet file with options
    pub fn write_parquet_with_options(
        &self,
        path: impl AsRef<Path>,
        options: ParquetWriteOptions,
    ) -> Result<()> {
        let file = File::create(path.as_ref())?;

        // Convert to RecordBatch
        let batch = self.to_record_batch()?;
        let schema = batch.schema();

        // Build writer properties
        let mut props_builder = WriterProperties::builder()
            .set_compression(options.compression.into());

        if let Some(row_group_size) = options.row_group_size {
            props_builder = props_builder.set_max_row_group_size(row_group_size);
        }

        if options.statistics {
            props_builder = props_builder.set_statistics_enabled(
                parquet::file::properties::EnabledStatistics::Page
            );
        }

        let props = props_builder.build();

        // Create writer
        let mut writer = ArrowWriter::try_new(file, schema, Some(props))?;

        // Write batch
        writer.write(&batch)?;
        writer.close()?;

        Ok(())
    }

    /// Read DataFrame from Parquet file
    ///
    /// # Example
    /// ```no_run
    /// # use avila_dataframe::prelude::*;
    /// let df = DataFrame::read_parquet("data.parquet")?;
    /// # Ok::<(), avila_dataframe::error::AvilaError>(())
    /// ```
    pub fn read_parquet(path: impl AsRef<Path>) -> Result<Self> {
        let file = File::open(path.as_ref())?;

        // Create reader
        let builder = ParquetRecordBatchReaderBuilder::try_new(file)?;
        let mut reader = builder.build()?;

        // Read first batch (for now, we'll support single batch)
        // TODO: Support multiple batches/chunked reading
        let batch = reader.next()
            .ok_or_else(|| AvilaError::generic("Parquet file is empty"))??;

        // Convert RecordBatch to DataFrame
        Self::from_record_batch(batch)
    }

    /// Read Parquet file with column selection
    pub fn read_parquet_columns(
        path: impl AsRef<Path>,
        columns: &[&str],
    ) -> Result<Self> {
        let file = File::open(path.as_ref())?;

        // Create reader with projection
        let builder = ParquetRecordBatchReaderBuilder::try_new(file)?;

        // Build projection mask
        let schema = builder.schema();
        let mut projection = Vec::new();

        for col_name in columns {
            if let Ok(index) = schema.index_of(col_name) {
                projection.push(index);
            } else {
                return Err(AvilaError::column_not_found(*col_name));
            }
        }

        let projection_mask = parquet::arrow::ProjectionMask::roots(
            builder.metadata().file_metadata().schema_descr(),
            projection
        );
        let reader = builder.with_projection(projection_mask).build()?;

        // Read batches
        let batches: std::result::Result<Vec<_>, _> = reader.collect();
        let batches = batches?;

        if batches.is_empty() {
            return Err(AvilaError::generic("Parquet file is empty"));
        }

        // For now, convert first batch
        Self::from_record_batch(batches[0].clone())
    }

    /// Get Parquet file metadata without reading data
    pub fn parquet_metadata(path: impl AsRef<Path>) -> Result<ParquetMetadata> {
        let file = File::open(path.as_ref())?;
        let builder = ParquetRecordBatchReaderBuilder::try_new(file)?;
        let metadata = builder.metadata();

        let schema_descriptor = metadata.file_metadata().schema_descr();
        Ok(ParquetMetadata {
            num_rows: metadata.file_metadata().num_rows() as usize,
            num_row_groups: metadata.num_row_groups(),
            num_columns: schema_descriptor.num_columns(),
            created_by: metadata.file_metadata().created_by().map(|s| s.to_string()),
        })
    }

}

/// Parquet file metadata
#[derive(Debug, Clone)]
pub struct ParquetMetadata {
    /// Number of rows
    pub num_rows: usize,
    /// Number of row groups
    pub num_row_groups: usize,
    /// Number of columns
    pub num_columns: usize,
    /// Created by information
    pub created_by: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn test_parquet_roundtrip() {
        let df = DataFrame::new(vec![
            Series::new("a", vec![1.0, 2.0, 3.0]),
            Series::new("b", vec![4.0, 5.0, 6.0]),
        ])
        .unwrap();

        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path();

        // Write
        df.write_parquet(path).unwrap();

        // Read
        let df2 = DataFrame::read_parquet(path).unwrap();

        assert_eq!(df.shape(), df2.shape());
        assert_eq!(df.column_names(), df2.column_names());
    }

    #[test]
    fn test_parquet_compression() {
        let df = DataFrame::new(vec![
            Series::new("data", vec![1.0, 2.0, 3.0, 4.0, 5.0]),
        ])
        .unwrap();

        let temp_file = NamedTempFile::new().unwrap();

        let options = ParquetWriteOptions {
            compression: Compression::Zstd,
            ..Default::default()
        };

        df.write_parquet_with_options(temp_file.path(), options).unwrap();
        let df2 = DataFrame::read_parquet(temp_file.path()).unwrap();

        assert_eq!(df.height(), df2.height());
    }
}
