use crate::{AudioStreamDecoder, AudioStreamSender, StreamChunk};
use bevy_asset::Asset;
use bevy_audio::Decodable;
use bevy_reflect::TypePath;
use crossbeam_queue::ArrayQueue;
use std::sync::Arc;

/// A lossy audio stream.
///
/// You can create a new stream with [`AudioStreamSource::new`], and send samples to it using [`AudioStreamSource::sender`].
///
/// This type is parallel to [`AudioSource`](bevy_audio::AudioSource), in the sense that you can use it in an [`AudioSourceBundle`](crate::AudioStreamBundle) to play audio.
///
/// This stream is lossy. If it runs out of capacity for new [`StreamChunk`]s, it will discard them.
#[derive(Asset, TypePath)]
pub struct AudioStreamSource {
    stream: Arc<ArrayQueue<StreamChunk>>,

    channels: u16,
    sample_rate: u32,
}

impl AudioStreamSource {
    /// Creates a new [`AudioStreamSource`].
    pub fn new(capacity: usize, channels: u16, sample_rate: u32) -> Self {
        Self {
            stream: Arc::new(ArrayQueue::new(capacity)),
            channels,
            sample_rate,
        }
    }

    /// Returns an [`AudioStreamSender`] which will send data through this stream.
    pub fn sender(&self) -> AudioStreamSender {
        AudioStreamSender::new(Arc::clone(&self.stream))
    }
}

impl Decodable for AudioStreamSource {
    type Decoder = AudioStreamDecoder;
    type DecoderItem = f32;

    fn decoder(&self) -> Self::Decoder {
        AudioStreamDecoder::new(Arc::clone(&self.stream), self.channels, self.sample_rate)
    }
}
