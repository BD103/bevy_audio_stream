use crate::{AudioStreamDecoder, AudioStreamSender, StreamChunk};
use bevy_asset::Asset;
use bevy_audio::Decodable;
use bevy_reflect::TypePath;
use crossbeam_queue::ArrayQueue;
use std::sync::Arc;

/// A lossy audio stream.
#[derive(Asset, TypePath)]
pub struct AudioStreamSource {
    stream: Arc<ArrayQueue<StreamChunk>>,

    channels: u16,
    sample_rate: u32,
}

impl AudioStreamSource {
    pub fn new(capacity: usize, channels: u16, sample_rate: u32) -> Self {
        Self {
            stream: Arc::new(ArrayQueue::new(capacity)),
            channels,
            sample_rate,
        }
    }

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
