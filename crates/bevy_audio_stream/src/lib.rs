mod decoder;
mod sender;
mod source;

pub use self::{decoder::AudioStreamDecoder, sender::AudioStreamSender, source::AudioStreamSource};

pub type AudioStreamBundle = bevy_audio::AudioSourceBundle<AudioStreamSource>;

pub type StreamChunk = [f32; 512];
