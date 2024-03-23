use crate::AudioStreamSource;
use bevy_app::{App, Plugin};
use bevy_audio::AddAudioSource;

/// A plugin that enables the use of [`AudioStreamSource`].
/// 
/// # Example
/// 
/// ```no_run
/// # use bevy::prelude::*;
/// # use bevy_audio_stream::AudioStreamPlugin;
/// #
/// App::new()
///     .add_plugins(DefaultPlugins)
///     .add_plugins(AudioStreamPlugin)
///     .run();
/// ```
pub struct AudioStreamPlugin;

impl Plugin for AudioStreamPlugin {
    fn build(&self, app: &mut App) {
        app.add_audio_source::<AudioStreamSource>();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adds_audio_source() {
        use crate::AudioStreamSource;
        use bevy::{asset::AssetPlugin, audio::AudioPlugin, prelude::*};

        App::new()
            // Needed for `AudioStreamPlugin`.
            .add_plugins((AssetPlugin::default(), AudioPlugin::default()))
            .add_plugins(AudioStreamPlugin)
            .add_systems(Startup, setup)
            // Runs app for one frame, calling `setup`, then exits.
            .run();

        fn setup(mut commands: Commands, mut audio_stream: ResMut<Assets<AudioStreamSource>>) {
            commands.spawn(crate::AudioStreamBundle {
                // This would fail if `AudioStreamSource` was not registered properly.
                source: audio_stream.add(AudioStreamSource::new(1, 1, 44100)),
                ..default()
            });
        }
    }
}
