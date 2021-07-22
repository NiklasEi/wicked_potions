use crate::GameState;
use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioChannel, AudioPlugin, AudioSource};
use crate::loading::AudioAssets;

pub struct InternalAudioPlugin;

impl Plugin for InternalAudioPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(AudioChannels {
            effects: AudioChannel::new("effects".to_owned()),
            background: AudioChannel::new("background".to_owned()),
        })
        .add_plugin(AudioPlugin)
        .add_event::<AudioEffect>()
        .add_system_set(SystemSet::on_enter(GameState::Menu).with_system(start_audio.system()))
        .add_system_set(SystemSet::on_update(GameState::Playing).with_system(play_effect.system()))
        .add_system_set(SystemSet::on_exit(GameState::Playing).with_system(stop_audio.system()));
    }
}

struct AudioChannels {
    effects: AudioChannel,
    background: AudioChannel,
}

pub struct AudioEffect {
    pub handle: Handle<AudioSource>,
}

fn start_audio(audio: Res<Audio>, channels: Res<AudioChannels>, audio_assets: Res<AudioAssets>) {
    audio.set_volume_in_channel(0.4, &channels.effects);
    audio.set_volume_in_channel(0.4, &channels.background);
    audio.play_looped_in_channel(audio_assets.background.clone(), &channels.background)
}

fn play_effect(
    mut events: EventReader<AudioEffect>,
    audio: Res<Audio>,
    channels: Res<AudioChannels>,
) {
    for effect in events.iter() {
        audio.play_in_channel(effect.handle.clone(), &channels.effects)
    }
}

fn stop_audio(audio: Res<Audio>, channels: Res<AudioChannels>) {
    audio.stop_channel(&channels.effects);
}
