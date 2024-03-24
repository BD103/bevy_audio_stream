use crate::StreamChunk;
use crossbeam_queue::ArrayQueue;
use std::sync::Arc;

/// A sender that sends audio through a stream.
///
/// Create a new sender by calling [`AudioStreamSource::sender`](crate::AudioStreamSource::sender).
///
/// # Example
///
/// ```
/// # use bevy_audio_stream::AudioStreamSource;
/// #
/// // Create a new audio stream.
/// let stream = AudioStreamSource::new(4, 1, 44100);
///
/// // Create a new sender for the associated stream.
/// let sender = stream.sender();
///
/// // Send new audio through the stream like this:
/// sender.send([0.5; 512]);
/// ```
#[repr(transparent)]
pub struct AudioStreamSender {
    stream: Arc<ArrayQueue<StreamChunk>>,
}

impl AudioStreamSender {
    pub(crate) fn new(stream: Arc<ArrayQueue<StreamChunk>>) -> Self {
        AudioStreamSender { stream }
    }

    /// Sends to chunk of audio through the streaming, returning true if successful.
    ///
    /// This will fail and return false if the stream is full and does not have a large enough capacity.
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn send() {
        // Create sender with capacity of 2.
        let sender = AudioStreamSender::new(Arc::new(ArrayQueue::new(2)));

        // First two chunks should succeed.
        assert!(sender.send([1.0; 512]));
        assert!(sender.send([1.0; 512]));

        // Third chunk should fail, not enough capacity!
        assert!(!sender.send([1.0; 512]));
    }

    #[test]
    fn clone() {
        let sender = AudioStreamSender::new(Arc::new(ArrayQueue::new(1)));
        let sender_clone = sender.clone();

        assert!(
            Arc::ptr_eq(&sender.stream, &sender_clone.stream),
            "Cloned audio stream sender does not share the same underlying memory."
        );
    }
}
