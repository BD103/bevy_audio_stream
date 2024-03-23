use crate::AudioStreamSource;
use bevy_app::{App, Plugin};
use bevy_audio::AddAudioSource;

/// A plugin that registers [`AudioStreamSource`].
pub struct AudioStreamPlugin;

impl Plugin for AudioStreamPlugin {
    fn build(&self, app: &mut App) {
        app.add_audio_source::<AudioStreamSource>();
    }
}
