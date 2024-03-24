#![doc = include_str!("../README.md")]

mod decoder;
mod plugin;
mod sender;
mod source;

pub use self::{
    decoder::AudioStreamDecoder, plugin::AudioStreamPlugin, sender::AudioStreamSender,
    source::AudioStreamSource,
};

/// A chunk of samples sent in a stream.
pub type StreamChunk = [f32; 512];

/// Bundle for playing streamed audio.
pub type AudioStreamBundle = bevy_audio::AudioSourceBundle<AudioStreamSource>;
