use crate::StreamChunk;
use crossbeam_queue::ArrayQueue;
use std::sync::Arc;

#[repr(transparent)]
pub struct AudioStreamSender {
    stream: Arc<ArrayQueue<StreamChunk>>,
}

impl AudioStreamSender {
    pub(crate) fn new(stream: Arc<ArrayQueue<StreamChunk>>) -> Self {
        AudioStreamSender { stream }
    }

    pub fn send(&self, chunk: StreamChunk) -> bool {
        self.stream.push(chunk).is_ok()
    }
}

impl Clone for AudioStreamSender {
    fn clone(&self) -> Self {
        AudioStreamSender {
            stream: Arc::clone(&self.stream),
        }
    }
}
