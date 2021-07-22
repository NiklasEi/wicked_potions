mod animate;
mod audio;
mod board;
mod hud;
mod loading;
mod matcher;
mod menu;

use crate::animate::AnimatePlugin;
use crate::audio::InternalAudioPlugin;
use crate::board::{BoardPlugin, Cauldron, Score};
use crate::hud::HudPlugin;
use crate::loading::LoadingPlugin;
use crate::menu::MenuPlugin;

use bevy::app::AppBuilder;
#[cfg(debug_assertions)]
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    Loading,
    Playing,
    Menu,
    Restart,
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
            .add_plugin(HudPlugin);

        app.add_system_set(SystemSet::on_enter(GameState::Restart).with_system(restart.system()));

        #[cfg(debug_assertions)]
        {
            app.add_plugin(FrameTimeDiagnosticsPlugin::default())
                .add_plugin(LogDiagnosticsPlugin::default());
        }
    }
}

fn restart(
    mut state: ResMut<State<GameState>>,
    mut cauldron: ResMut<Cauldron>,
    mut score: ResMut<Score>,
) {
    *cauldron = Cauldron::new();
    score.money = 0;
    state.set(GameState::Playing).unwrap();
}
