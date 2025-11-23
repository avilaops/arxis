//! Streaming support for HTTP requests and responses
//!
//! This module provides streaming capabilities for handling large files
//! and real-time data streams efficiently.

use crate::error::{Error, Result};
use bytes::Bytes;
use futures::stream::Stream;
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::io::AsyncRead;

/// A streaming body that can be read in chunks
pub struct StreamingBody {
    inner: Pin<Box<dyn Stream<Item = Result<Bytes>> + Send>>,
}

impl StreamingBody {
    /// Create a new streaming body from a stream
    pub fn new<S>(stream: S) -> Self
    where
        S: Stream<Item = Result<Bytes>> + Send + 'static,
    {
        Self {
            inner: Box::pin(stream),
        }
    }

    /// Create a streaming body from an AsyncRead
    pub fn from_reader<R>(reader: R, chunk_size: usize) -> Self
    where
        R: AsyncRead + Send + Unpin + 'static,
    {
        let stream = ReaderStream::new(reader, chunk_size);
        Self::new(stream)
    }

    /// Get the next chunk from the stream
    pub async fn next_chunk(&mut self) -> Option<Result<Bytes>> {
        use futures::StreamExt;
        self.inner.next().await
    }

    /// Collect all chunks into a single Bytes
    pub async fn collect(mut self) -> Result<Bytes> {
        let mut chunks = Vec::new();
        while let Some(chunk) = self.next_chunk().await {
            chunks.push(chunk?);
        }

        let total_len: usize = chunks.iter().map(|c| c.len()).sum();
        let mut result = Vec::with_capacity(total_len);
        for chunk in chunks {
            result.extend_from_slice(&chunk);
        }

        Ok(Bytes::from(result))
    }
}

/// Stream adapter for AsyncRead
struct ReaderStream<R> {
    reader: Option<R>,
    chunk_size: usize,
}

impl<R> ReaderStream<R> {
    fn new(reader: R, chunk_size: usize) -> Self {
        Self {
            reader: Some(reader),
            chunk_size,
        }
    }
}

impl<R: AsyncRead + Unpin + Send> Stream for ReaderStream<R> {
    type Item = Result<Bytes>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let chunk_size = self.chunk_size;

        if let Some(reader) = &mut self.reader {
            let mut buf = vec![0u8; chunk_size];
            let mut read_buf = tokio::io::ReadBuf::new(&mut buf);

            match Pin::new(reader).poll_read(cx, &mut read_buf) {
                Poll::Ready(Ok(())) => {
                    let filled = read_buf.filled().len();
                    if filled == 0 {
                        self.reader = None;
                        Poll::Ready(None)
                    } else {
                        buf.truncate(filled);
                        Poll::Ready(Some(Ok(Bytes::from(buf))))
                    }
                }
                Poll::Ready(Err(e)) => {
                    self.reader = None;
                    Poll::Ready(Some(Err(Error::BodyReadError { source: e })))
                }
                Poll::Pending => Poll::Pending,
            }
        } else {
            Poll::Ready(None)
        }
    }
}

/// Chunked transfer encoding support
pub struct ChunkedEncoder {
    inner: StreamingBody,
}

impl ChunkedEncoder {
    /// Create a new chunked encoder
    pub fn new(body: StreamingBody) -> Self {
        Self { inner: body }
    }

    /// Encode the next chunk in chunked transfer encoding format
    pub async fn next_encoded_chunk(&mut self) -> Option<Result<Bytes>> {
        match self.inner.next_chunk().await {
            Some(Ok(chunk)) => {
                if chunk.is_empty() {
                    // Last chunk
                    Some(Ok(Bytes::from("0\r\n\r\n")))
                } else {
                    // Size in hex + \r\n + data + \r\n
                    let size_hex = format!("{:x}\r\n", chunk.len());
                    let mut encoded = Vec::with_capacity(size_hex.len() + chunk.len() + 2);
                    encoded.extend_from_slice(size_hex.as_bytes());
                    encoded.extend_from_slice(&chunk);
                    encoded.extend_from_slice(b"\r\n");
                    Some(Ok(Bytes::from(encoded)))
                }
            }
            Some(Err(e)) => Some(Err(e)),
            None => Some(Ok(Bytes::from("0\r\n\r\n"))), // Terminating chunk
        }
    }
}

/// Server-Sent Events (SSE) stream
pub struct SseStream {
    inner: StreamingBody,
}

impl SseStream {
    /// Create a new SSE stream
    pub fn new(body: StreamingBody) -> Self {
        Self { inner: body }
    }

    /// Create an SSE event
    pub fn event(event_type: &str, data: &str) -> Bytes {
        let mut event = String::new();
        if !event_type.is_empty() {
            event.push_str(&format!("event: {}\n", event_type));
        }
        for line in data.lines() {
            event.push_str(&format!("data: {}\n", line));
        }
        event.push('\n');
        Bytes::from(event)
    }

    /// Create an SSE comment
    pub fn comment(text: &str) -> Bytes {
        Bytes::from(format!(": {}\n\n", text))
    }

    /// Create an SSE retry instruction
    pub fn retry(milliseconds: u64) -> Bytes {
        Bytes::from(format!("retry: {}\n\n", milliseconds))
    }

    /// Get the next SSE message
    pub async fn next_message(&mut self) -> Option<Result<Bytes>> {
        self.inner.next_chunk().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::stream;

    #[tokio::test]
    async fn test_streaming_body_from_vec() {
        let data = vec![
            Ok(Bytes::from("Hello, ")),
            Ok(Bytes::from("World!")),
        ];
        let stream = stream::iter(data);
        let mut body = StreamingBody::new(stream);

        let chunk1 = body.next_chunk().await.unwrap().unwrap();
        assert_eq!(chunk1, Bytes::from("Hello, "));

        let chunk2 = body.next_chunk().await.unwrap().unwrap();
        assert_eq!(chunk2, Bytes::from("World!"));

        assert!(body.next_chunk().await.is_none());
    }

    #[tokio::test]
    async fn test_streaming_body_collect() {
        let data = vec![
            Ok(Bytes::from("Hello, ")),
            Ok(Bytes::from("World!")),
        ];
        let stream = stream::iter(data);
        let body = StreamingBody::new(stream);

        let collected = body.collect().await.unwrap();
        assert_eq!(collected, Bytes::from("Hello, World!"));
    }

    #[test]
    fn test_sse_event() {
        let event = SseStream::event("message", "Hello, World!");
        let expected = "event: message\ndata: Hello, World!\n\n";
        assert_eq!(event, Bytes::from(expected));
    }

    #[test]
    fn test_sse_event_multiline() {
        let event = SseStream::event("update", "Line 1\nLine 2\nLine 3");
        let expected = "event: update\ndata: Line 1\ndata: Line 2\ndata: Line 3\n\n";
        assert_eq!(event, Bytes::from(expected));
    }

    #[test]
    fn test_sse_comment() {
        let comment = SseStream::comment("Keep alive");
        assert_eq!(comment, Bytes::from(": Keep alive\n\n"));
    }

    #[test]
    fn test_sse_retry() {
        let retry = SseStream::retry(3000);
        assert_eq!(retry, Bytes::from("retry: 3000\n\n"));
    }
}
