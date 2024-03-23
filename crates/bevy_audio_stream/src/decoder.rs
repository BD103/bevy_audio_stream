use crate::StreamChunk;
use bevy_audio::Source;
use crossbeam_queue::ArrayQueue;
use std::sync::Arc;

/// A [`Source`] decoder that receives samples from [`AudioStreamSender`](crate::AudioStreamSender).
///
/// This type should never be created manually. You are likely looking for [`AudioStreamSource`](crate::AudioStreamSource) instead.
pub struct AudioStreamDecoder {
    /// Stream for popping new sample chunks.
    stream: Arc<ArrayQueue<StreamChunk>>,

    /// The chunk currently being iterated.
    chunk: Box<StreamChunk>,
    /// Iterator index.
    i: usize,

    channels: u16,
    sample_rate: u32,
}

impl AudioStreamDecoder {
    /// Creates a new [`AudioStreamDecoder`] from a stream.
    pub(crate) fn new(
        stream: Arc<ArrayQueue<StreamChunk>>,
        channels: u16,
        sample_rate: u32,
    ) -> Self {
        Self {
            stream,

            chunk: Box::new([0.0; 512]),
            i: 512, // First time Self::next is called it will pop a new chunk from the stream.

            channels,
            sample_rate,
        }
    }
}

impl Iterator for AudioStreamDecoder {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        // If the entire chunk has been yielded...
        if self.i >= self.chunk.len() {
            if let Some(chunk) = self.stream.pop() {
                // Nice! There's some new audio for us.
                *self.chunk = chunk;
                self.i = 0;
            } else {
                // There's no new audio, yield silence.
                return Some(0.0);
            }
        }

        let res = self.chunk[self.i];

        self.i += 1;

        Some(res)
    }
}

impl Source for AudioStreamDecoder {
    fn current_frame_len(&self) -> Option<usize> {
        // `AudioStreamDecoder` currently does not support changing at runtime.
        None
    }

    fn channels(&self) -> u16 {
        self.channels
    }

    fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    fn total_duration(&self) -> Option<std::time::Duration> {
        // Audio is a stream, so technically infinite.
        None
    }
}
