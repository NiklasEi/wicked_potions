mod animate;
mod audio;
mod board;
mod hud;
mod loading;
mod lost;
mod matcher;
mod menu;

use crate::animate::AnimatePlugin;
use crate::audio::InternalAudioPlugin;
use crate::board::BoardPlugin;
use crate::hud::HudPlugin;
use crate::loading::LoadingPlugin;
use crate::menu::MenuPlugin;

use crate::lost::LostPlugin;
use bevy::app::AppBuilder;
#[cfg(debug_assertions)]
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    Loading,
    Playing,
    Menu,
    Lost,
}

#[derive(SystemLabel, Clone, Hash, Debug, Eq, PartialEq)]
pub enum SystemLabels {
    Animate,
    DisplayUiForNewRecipe,
    MatchPatterns,
    UserInput,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_state(GameState::Loading)
            .add_plugin(LoadingPlugin)
            .add_plugin(MenuPlugin)
            .add_plugin(InternalAudioPlugin)
            .add_plugin(BoardPlugin)
            .add_plugin(AnimatePlugin)
            .add_plugin(HudPlugin)
            .add_plugin(LostPlugin);

        #[cfg(debug_assertions)]
        {
            app.add_plugin(FrameTimeDiagnosticsPlugin::default())
                .add_plugin(LogDiagnosticsPlugin::default());
        }
    }
}
