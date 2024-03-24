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
/// This type is parallel to [`AudioSource`](bevy_audio::AudioSource) in the sense that you can use it in an [`AudioSourceBundle`](crate::AudioStreamBundle) to play audio.
///
/// This stream is lossy. If it runs out of capacity for new [`StreamChunk`]s, it will discard them.
///
/// # Example
///
/// ```
/// # use bevy::prelude::*;
/// # use bevy_audio_stream::{AudioStreamBundle, AudioStreamSender, AudioStreamSource};
/// #
/// #[derive(Resource, Deref)]
/// struct MyAudioSender(AudioStreamSender);
///
/// fn setup(mut commands: Commands, mut audio: ResMut<Assets<AudioStreamSource>>) {
///     // Create a new audio stream.
///     let stream = AudioStreamSource::new(4, 1, 44100);
///
///     // Add the stream sender as a resource, for later access.
///     commands.insert_resource(MyAudioSender(stream.sender()));
///
///     // Spawn a new entity that will play the audio stream.
///     commands.spawn(AudioStreamBundle {
///         source: audio.add(stream),
///         ..default()
///     });
/// }
/// #
/// # fn check_valid_system<M>(_system: impl IntoSystemConfigs<M>) {}
/// #
/// # check_valid_system(setup);
/// ```
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

#[cfg(test)]
mod tests {
    use bevy_audio::Source;

    use super::*;

    #[test]
    fn receives_audio() {
        const CHANNELS: u16 = 1;
        const SAMPLE_RATE: u32 = 44100;

        let stream = AudioStreamSource::new(1, CHANNELS, SAMPLE_RATE);
        let sender = stream.sender();
        let decoder = stream.decoder();

        // Assert settings persist from source to decoder.
        assert_eq!(decoder.channels(), CHANNELS);
        assert_eq!(decoder.sample_rate(), SAMPLE_RATE);

        // Assert sent audio is received by decoder.
        assert!(
            sender.send([0.8; 512]),
            "Failed to send audio through stream, even with sufficient capacity."
        );
        assert!(decoder.take(512).all(|x| x == 0.8));
    }
}
