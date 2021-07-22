use crate::loading::AudioAssets;
use crate::GameState;
use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioChannel, AudioPlugin, AudioSource};

pub struct InternalAudioPlugin;

impl Plugin for InternalAudioPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(AudioChannels {
            effects: AudioChannel::new("effects".to_owned()),
            cooking: AudioChannel::new("cooking".to_owned()),
            background: AudioChannel::new("background".to_owned()),
        })
        .add_plugin(AudioPlugin)
        .add_event::<AudioEffect>()
        .add_system_set(SystemSet::on_enter(GameState::Menu).with_system(start_audio.system()))
        .add_system(play_effect.system());
    }
}

struct AudioChannels {
    effects: AudioChannel,
    background: AudioChannel,
    cooking: AudioChannel,
}

pub struct AudioEffect {
    pub handle: Handle<AudioSource>,
}

fn start_audio(audio: Res<Audio>, channels: Res<AudioChannels>, audio_assets: Res<AudioAssets>) {
    audio.set_volume_in_channel(0.4, &channels.effects);
    audio.set_volume_in_channel(0.3, &channels.background);
    audio.set_volume_in_channel(1.0, &channels.cooking);
    audio.play_looped_in_channel(audio_assets.background.clone(), &channels.background);
    audio.play_looped_in_channel(audio_assets.cooking.clone(), &channels.cooking);
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
