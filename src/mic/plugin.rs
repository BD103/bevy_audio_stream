use super::MicConfig;
use bevy_app::prelude::*;

/// Plugin for recording audio from the microphone.
/// 
/// You can configure this plugin by inserting a [`MicConfig`] resource.
pub struct MicPlugin;

impl Plugin for MicPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MicConfig>();
    }
}
