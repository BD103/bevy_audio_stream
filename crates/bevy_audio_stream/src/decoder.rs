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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn receives_stream() {
        // Create a new stream and associated decoder.
        let stream = Arc::new(ArrayQueue::new(2));
        let decoder = AudioStreamDecoder::new(Arc::clone(&stream), 1, 44100);

        // Push a chunk of all ones, then all halves.
        stream.push([1.0; 512]).unwrap();
        stream.push([0.5; 512]).unwrap();

        // Iterate over the decoder and collect both chunks into a `Vec`.
        let samples: Vec<_> = decoder.take(512 * 2).collect();
        let (all_ones, all_halves) = samples.split_at(512);

        // Check that data is intact.
        assert!(all_ones.iter().all(|&x| x == 1.0));
        assert!(all_halves.iter().all(|&x| x == 0.5));
    }

    #[test]
    fn empty_stream() {
        // Create a new stream and associated decoder.
        let stream = Arc::new(ArrayQueue::new(1));
        let mut decoder = AudioStreamDecoder::new(Arc::clone(&stream), 1, 44100);

        // If there is nothing in the stream, it should return silence.
        assert_eq!(
            decoder.next(),
            Some(0.0),
            "Empty decoder stream did not return silence."
        );

        // Send something in the stream.
        stream.push([1.0; 512]).unwrap();

        assert_eq!(
            decoder.next(),
            Some(1.0),
            "Decoder stream did not receiver new audio."
        );
    }

    #[test]
    fn index() {
        // Create a new stream and associated decoder.
        let stream = Arc::new(ArrayQueue::new(1));
        let mut decoder = AudioStreamDecoder::new(Arc::clone(&stream), 1, 44100);

        let original_index = decoder.i;

        // Original index needs to be >= the length of a chunk, so the decoder pops a new chunk from the stream next time `.next()` is called.
        assert_eq!(
            original_index, 512,
            "Decoder stream original index is not the length of `StreamChunk`."
        );

        decoder.next();

        // If i increments with an empty stream, then it could wrap around to 0 and replace audio.
        assert_eq!(
            decoder.i, original_index,
            "Decoder stream index incremented even while empty."
        );

        stream.push([0.0; 512]).unwrap();
        decoder.next();

        assert_eq!(
            decoder.i, 1,
            "Decoder stream index did not change after receiving audio."
        );
    }
}
