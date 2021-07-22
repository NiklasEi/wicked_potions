use crate::board::{Cauldron, Score};
use crate::loading::FontAssets;
use crate::GameState;
use bevy::prelude::*;

pub struct LostPlugin;

impl Plugin for LostPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<ButtonMaterials>()
            .add_system_set(SystemSet::on_enter(GameState::Lost).with_system(setup_menu.system()))
            .add_system_set(
                SystemSet::on_update(GameState::Lost).with_system(click_play_button.system()),
            )
            .add_system_set(SystemSet::on_exit(GameState::Lost).with_system(reset.system()));
    }
}

struct ButtonMaterials {
    normal: Handle<ColorMaterial>,
    hovered: Handle<ColorMaterial>,
}

impl FromWorld for ButtonMaterials {
    fn from_world(world: &mut World) -> Self {
        let mut materials = world.get_resource_mut::<Assets<ColorMaterial>>().unwrap();
        ButtonMaterials {
            normal: materials.add(Color::rgb(0.15, 0.15, 0.15).into()),
            hovered: materials.add(Color::rgb(0.25, 0.25, 0.25).into()),
        }
    }
}

struct Lost;

struct PlayButton;

fn reset(mut cauldron: ResMut<Cauldron>, mut score: ResMut<Score>) {
    *cauldron = Cauldron::new();
    score.money = 0;
}

fn setup_menu(
    mut commands: Commands,
    font_assets: Res<FontAssets>,
    button_materials: Res<ButtonMaterials>,
    cauldron: Res<Cauldron>,
    score: Res<Score>,
) {
    commands
        .spawn_bundle(UiCameraBundle::default())
        .insert(Lost);
    commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(120.0), Val::Px(50.0)),
                margin: Rect::all(Val::Auto),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                position_type: PositionType::Absolute,
                position: Rect {
                    right: Val::Px(70.0),
                    top: Val::Px(90.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            material: button_materials.normal.clone(),
            ..Default::default()
        })
        .insert(PlayButton)
        .insert(Lost)
        .with_children(|parent| {
            parent
                .spawn_bundle(TextBundle {
                    text: Text {
                        sections: vec![TextSection {
                            value: "Again".to_string(),
                            style: TextStyle {
                                font: font_assets.fira_sans.clone(),
                                font_size: 40.0,
                                color: Color::rgb(0.9, 0.9, 0.9),
                            },
                        }],
                        alignment: Default::default(),
                    },
                    ..Default::default()
                })
                .insert(Lost);
        });
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Px(120.0), Val::Px(50.0)),
                margin: Rect::all(Val::Auto),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                position_type: PositionType::Absolute,
                position: Rect {
                    right: Val::Px(70.0),
                    top: Val::Px(220.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            visible: Visible {
                is_visible: false,
                is_transparent: true,
            },
            ..Default::default()
        })
        .insert(Lost)
        .with_children(|parent| {
            parent
                .spawn_bundle(TextBundle {
                    text: Text {
                        sections: vec![TextSection {
                            value: format!(
                                "You brew enough\nPotions to let\nevil win {} times\n\n\nCoins: {}",
                                cauldron.finished_recipes, score.money
                            ),
                            style: TextStyle {
                                font: font_assets.fira_sans.clone(),
                                font_size: 20.0,
                                color: Color::rgb(0.9, 0.9, 0.9),
                            },
                        }],
                        alignment: Default::default(),
                    },
                    ..Default::default()
                })
                .insert(Lost);
        });
}

fn click_play_button(
    mut commands: Commands,
    button_materials: Res<ButtonMaterials>,
    mut state: ResMut<State<GameState>>,
    mut interaction_query: Query<
        (&Interaction, &mut Handle<ColorMaterial>),
        (Changed<Interaction>, With<Button>),
    >,
    menu_elements: Query<Entity, With<Lost>>,
) {
    for (interaction, mut material) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                for entity in menu_elements.iter() {
                    commands.entity(entity).despawn();
                }
                state.set(GameState::Playing).unwrap();
            }
            Interaction::Hovered => {
                *material = button_materials.hovered.clone();
            }
            Interaction::None => {
                *material = button_materials.normal.clone();
            }
        }
    }
}
