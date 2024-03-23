use bevy_ecs::prelude::*;

/// Used to configure [`MicPlugin`](crate::MicPlugin).
///
/// Insert this into your app to override the defaults.
#[derive(Resource, Default)]
pub struct MicConfig {}
