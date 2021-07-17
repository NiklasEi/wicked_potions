use bevy::prelude::*;
use bevy_kira_audio::AudioPlugin;

pub struct InternalAudioPlugin;

impl Plugin for InternalAudioPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(AudioPlugin);
    }
}
