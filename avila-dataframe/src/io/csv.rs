//! CSV I/O - Streaming support for large files

use crate::core::{DataFrame, Series};
use crate::error::{AvilaError, Result};
use arrow::datatypes::Schema as ArrowSchema;
use arrow_csv::{Reader as CsvReader, ReaderBuilder};
use std::fs::File;
use std::io::{BufReader, BufWriter, Write};
use std::path::Path;
use std::sync::Arc;

/// CSV write options
pub struct CsvWriteOptions {
    /// Field delimiter (default: ',')
    pub delimiter: u8,
    /// Include header row
    pub header: bool,
    /// Date format
    pub date_format: Option<String>,
    /// Timestamp format
    pub timestamp_format: Option<String>,
}

impl Default for CsvWriteOptions {
    fn default() -> Self {
        Self {
            delimiter: b',',
            header: true,
            date_format: None,
            timestamp_format: None,
        }
    }
}

/// CSV read options
pub struct CsvReadOptions {
    /// Field delimiter (default: ',')
    pub delimiter: u8,
    /// Has header row
    pub has_header: bool,
    /// Infer schema from first N rows
    pub infer_schema_length: Option<usize>,
    /// Batch size for streaming
    pub batch_size: usize,
}

impl Default for CsvReadOptions {
    fn default() -> Self {
        Self {
            delimiter: b',',
            has_header: true,
            infer_schema_length: Some(100),
            batch_size: 8192,
        }
    }
}

impl DataFrame {
    /// Write DataFrame to CSV file
    ///
    /// # Example
    /// ```no_run
    /// # use avila_dataframe::prelude::*;
    /// let df = DataFrame::new(vec![
    ///     Series::new("name", vec![1.0, 2.0]),
    ///     Series::new("value", vec![10.0, 20.0]),
    /// ])?;
    /// df.write_csv("output.csv")?;
    /// # Ok::<(), avila_dataframe::error::AvilaError>(())
    /// ```
    pub fn write_csv(&self, path: impl AsRef<Path>) -> Result<()> {
        self.write_csv_with_options(path, CsvWriteOptions::default())
    }

    /// Write DataFrame to CSV with options
    pub fn write_csv_with_options(
        &self,
        path: impl AsRef<Path>,
        options: CsvWriteOptions,
    ) -> Result<()> {
        let file = File::create(path.as_ref())?;
        let mut writer = BufWriter::new(file);

        // Write header
        if options.header {
            let header = self
                .column_names()
                .join(&(options.delimiter as char).to_string());
            writeln!(writer, "{}", header)?;
        }

        // Write rows
        for row_idx in 0..self.height() {
            let mut row_values = Vec::new();

            for col in &self.columns {
                let value = col
                    .get_f64(row_idx)
                    .map(|v| format!("{}", v))
                    .unwrap_or_else(|_| String::from(""));
                row_values.push(value);
            }

            let row_str = row_values.join(&(options.delimiter as char).to_string());
            writeln!(writer, "{}", row_str)?;
        }

        writer.flush()?;
        Ok(())
    }

    /// Read DataFrame from CSV file
    ///
    /// # Example
    /// ```no_run
    /// # use avila_dataframe::prelude::*;
    /// let df = DataFrame::read_csv("data.csv")?;
    /// # Ok::<(), avila_dataframe::error::AvilaError>(())
    /// ```
    pub fn read_csv(path: impl AsRef<Path>) -> Result<Self> {
        Self::read_csv_with_options(path, CsvReadOptions::default())
    }

    /// Read DataFrame from CSV with options
    pub fn read_csv_with_options(path: impl AsRef<Path>, options: CsvReadOptions) -> Result<Self> {
        let file1 = File::open(path.as_ref())?;
        let (schema, _) = arrow_csv::reader::infer_reader_schema(
            BufReader::new(file1),
            options.delimiter,
            options.infer_schema_length,
            options.has_header,
        )?;

        let file2 = File::open(path.as_ref())?;
        let reader = BufReader::new(file2);

        // Build CSV reader
        let builder = ReaderBuilder::new(Arc::new(schema))
            .with_delimiter(options.delimiter)
            .with_header(options.has_header)
            .with_batch_size(options.batch_size);

        let mut csv_reader = builder.build(reader)?;

        // Read first batch
        let batch = csv_reader
            .next()
            .ok_or_else(|| AvilaError::generic("CSV file is empty"))??;

        // Convert to DataFrame
        Self::from_record_batch(batch)
    }

    /// Read CSV in chunks (streaming)
    pub fn read_csv_chunked(path: impl AsRef<Path>, chunk_size: usize) -> Result<CsvChunkedReader> {
        let file1 = File::open(path.as_ref())?;
        let (schema, _) =
            arrow_csv::reader::infer_reader_schema(BufReader::new(file1), b',', Some(100), true)?;

        let file2 = File::open(path.as_ref())?;
        let reader = BufReader::new(file2);

        let csv_reader = ReaderBuilder::new(Arc::new(schema))
            .with_batch_size(chunk_size)
            .with_header(true)
            .build(reader)?;

        Ok(CsvChunkedReader { reader: csv_reader })
    }
}

/// Chunked CSV reader for streaming large files
pub struct CsvChunkedReader {
    reader: CsvReader<BufReader<File>>,
}

impl CsvChunkedReader {
    /// Read next chunk
    pub fn next_chunk(&mut self) -> Result<Option<DataFrame>> {
        match self.reader.next() {
            Some(Ok(batch)) => Ok(Some(DataFrame::from_record_batch(batch)?)),
            Some(Err(e)) => Err(e.into()),
            None => Ok(None),
        }
    }

    /// Process all chunks with a callback
    pub fn process_chunks<F, T>(&mut self, mut f: F) -> Result<Vec<T>>
    where
        F: FnMut(DataFrame) -> Result<T>,
    {
        let mut results = Vec::new();

        while let Some(chunk) = self.next_chunk()? {
            results.push(f(chunk)?);
        }

        Ok(results)
    }
}

impl Iterator for CsvChunkedReader {
    type Item = Result<DataFrame>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.next_chunk() {
            Ok(Some(df)) => Some(Ok(df)),
            Ok(None) => None,
            Err(e) => Some(Err(e)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_csv_roundtrip() {
        let df = DataFrame::new(vec![
            Series::new("a", vec![1.0, 2.0, 3.0]),
            Series::new("b", vec![4.0, 5.0, 6.0]),
        ])
        .unwrap();

        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path();

        // Write
        df.write_csv(path).unwrap();

        // Read
        let df2 = DataFrame::read_csv(path).unwrap();

        assert_eq!(df.shape(), df2.shape());
    }

    #[test]
    fn test_csv_custom_delimiter() {
        let df = DataFrame::new(vec![Series::new("x", vec![1.0, 2.0])]).unwrap();

        let temp_file = NamedTempFile::new().unwrap();

        let options = CsvWriteOptions {
            delimiter: b';',
            ..Default::default()
        };

        df.write_csv_with_options(temp_file.path(), options)
            .unwrap();

        // Verify file contains semicolons
        let content = std::fs::read_to_string(temp_file.path()).unwrap();
        assert!(content.contains(';'));
    }

    #[test]
    fn test_csv_chunked_reading() {
        // Create test CSV file
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "value").unwrap();
        for i in 0..100 {
            writeln!(temp_file, "{}", i).unwrap();
        }
        temp_file.flush().unwrap();

        let mut reader = DataFrame::read_csv_chunked(temp_file.path(), 10).unwrap();

        let mut total_rows = 0;
        while let Some(chunk) = reader.next_chunk().unwrap() {
            total_rows += chunk.height();
        }

        assert_eq!(total_rows, 100);
    }
}
