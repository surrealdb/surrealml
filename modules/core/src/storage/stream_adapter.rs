//! Stream adapter for file system
use std::fs::File;
use std::io::Read;
use bytes::Bytes;

use futures_core::stream::Stream;
use futures_core::task::{Context, Poll};
use std::pin::Pin;
use std::error::Error;
use crate::{
    safe_eject,
    errors::error::{
        SurrealError,
        SurrealErrorStatus
    }
};


/// Stream adapter for file system.
/// 
/// # Arguments
/// * `chunk_size` - The size of the chunks to read from the file.
/// * `file_pointer` - The pointer to the file to be streamed
pub struct StreamAdapter {
    chunk_size: usize,
    file_pointer: File
}

impl StreamAdapter {

    /// Creates a new `StreamAdapter` struct.
    /// 
    /// # Arguments
    /// * `chunk_size` - The size of the chunks to read from the file.
    /// * `file_path` - The path to the file to be streamed
    /// 
    /// # Returns
    /// A new `StreamAdapter` struct.
    pub fn new(chunk_size: usize, file_path: String) -> Result<Self, SurrealError> {
        let file_pointer = safe_eject!(File::open(file_path), SurrealErrorStatus::NotFound);
        Ok(StreamAdapter {
            chunk_size,
            file_pointer
        })
    }

}

impl Stream for StreamAdapter {

    type Item = Result<Bytes, Box<dyn Error + Send + Sync>>;

    /// Polls the next chunk from the file.
    /// 
    /// # Arguments
    /// * `self` - The `StreamAdapter` struct.
    /// * `cx` - The context of the task to enable the task to be woken up and polled again using the waker.
    /// 
    /// # Returns
    /// A poll containing the next chunk from the file.
    fn poll_next(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let mut buffer = vec![0u8; self.chunk_size];
        let bytes_read = self.file_pointer.read(&mut buffer)?;

        buffer.truncate(bytes_read);
        if buffer.is_empty() {
            return Poll::Ready(None);
        }
        Poll::Ready(Some(Ok(buffer.into())))
    }
}
