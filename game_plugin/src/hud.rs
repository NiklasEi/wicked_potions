use crate::board::{Cauldron, Ingredients, Score};
use crate::loading::{FontAssets, TextureAssets};
use crate::matcher::Collectable;
use crate::{GameState, SystemLabels};
use bevy::prelude::*;

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(SystemSet::on_enter(GameState::Playing).with_system(setup_ui.system()))
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(
                        update_recipe
                            .system()
                            .after(SystemLabels::DisplayUiForNewRecipe),
                    )
                    .with_system(
                        finished_recipe
                            .system()
                            .label(SystemLabels::DisplayUiForNewRecipe),
                    )
                    .with_system(update_score.system().after(SystemLabels::Animate)),
            )
            .add_event::<FinishedRecipe>();
    }
}

pub struct FinishedRecipe;

struct Ui;

fn setup_ui(
    mut commands: Commands,
    cauldron: Res<Cauldron>,
    fonts: Res<FontAssets>,
    textures: Res<TextureAssets>,
) {
    display_ingredients(&mut commands, &cauldron, &fonts, &textures);
    commands
        .spawn_bundle(Text2dBundle {
            text: Text {
                sections: vec![TextSection {
                    value: format!("Completed {} potions", 0),
                    style: TextStyle {
                        font: fonts.fira_sans.clone(),
                        font_size: 15.,
                        ..Default::default()
                    },
                }],
                alignment: Default::default(),
            },
            transform: Transform::from_translation(Vec3::new(800. - 76., 350., 10.)),
            ..Text2dBundle::default()
        })
        .insert(Ui)
        .insert(PotionsCount);
    commands
        .spawn_bundle(Text2dBundle {
            text: Text {
                sections: vec![TextSection {
                    value: format!("Treasury {}$", 0),
                    style: TextStyle {
                        font: fonts.fira_sans.clone(),
                        font_size: 15.,
                        ..Default::default()
                    },
                }],
                alignment: Default::default(),
            },
            transform: Transform::from_translation(Vec3::new(800. - 76., 310., 10.)),
            ..Text2dBundle::default()
        })
        .insert(Ui)
        .insert(MoneyDisplay);
}

struct MoneyDisplay;
struct PotionsCount;

fn finished_recipe(
    mut commands: Commands,
    mut events: EventReader<FinishedRecipe>,
    cauldron: Res<Cauldron>,
    fonts: Res<FontAssets>,
    mut potions_count: Query<&mut Text, (With<Ui>, With<PotionsCount>)>,
    textures: Res<TextureAssets>,
    recipe_ui_components: Query<Entity, (With<Ui>, With<Collectable>)>,
) {
    for _event in events.iter() {
        println!("finished recipe: {:?}", *cauldron);

        for entity in recipe_ui_components.iter() {
            commands.entity(entity).despawn();
        }
        display_ingredients(&mut commands, &cauldron, &fonts, &textures);
        if let Ok(mut text) = potions_count.single_mut() {
            text.sections[0].value = format!("Completed {} potions", cauldron.finished_recipes);
        }
    }
}

fn display_ingredients(
    commands: &mut Commands,
    cauldron: &Cauldron,
    fonts: &FontAssets,
    textures: &TextureAssets,
) {
    let mut index = 0;
    for ingredient in cauldron.recipe.ingredients.iter() {
        let mut text_transform = Transform::from_translation(Vec3::new(
            800. - (index + 1) as f32 * 76. - 18.,
            430.,
            10.,
        ));
        let mut icon_transform = Transform::from_translation(Vec3::new(
            800. - (index + 1) as f32 * 76. + 5.,
            430. + 7.,
            10.,
        ));
        icon_transform.scale = Vec3::new(0.5, 0.5, 0.5);
        if index == 2 {
            text_transform.translation.y -= 30.;
            text_transform.translation.x += 76.;
            icon_transform.translation.y -= 30.;
            icon_transform.translation.x += 76.;
        }
        commands
            .spawn_bundle(Text2dBundle {
                text: Text {
                    sections: vec![TextSection {
                        value: format!("{}/{}", 0, ingredient.amount),
                        style: TextStyle {
                            font: fonts.fira_sans.clone(),
                            font_size: 15.,
                            ..Default::default()
                        },
                    }],
                    alignment: Default::default(),
                },
                transform: text_transform,
                ..Text2dBundle::default()
            })
            .insert(Ui)
            .insert(ingredient.collectable.clone());
        commands
            .spawn_bundle(SpriteSheetBundle {
                texture_atlas: ingredient.collectable.get_texture(&textures),
                transform: icon_transform,
                sprite: TextureAtlasSprite {
                    index: ingredient.collectable.get_animation().frames - 1,
                    ..TextureAtlasSprite::default()
                },
                ..SpriteSheetBundle::default()
            })
            .insert(Ui)
            .insert(ingredient.collectable.clone());
        index += 1;
    }
}

fn update_score(
    score: Res<Score>,
    mut potions_count: Query<&mut Text, (With<Ui>, With<MoneyDisplay>)>,
) {
    if score.is_changed() {
        if let Ok(mut text) = potions_count.single_mut() {
            text.sections[0].value = format!("Treasury {}$", score.money);
        }
    }
}

fn update_recipe(
    cauldron: Res<Cauldron>,
    mut text: Query<(&mut Text, &Collectable), (With<Ui>, Without<TextureAtlasSprite>)>,
) {
    if cauldron.is_changed() || cauldron.is_added() {
        for (mut text, collectable) in text.iter_mut() {
            text.sections.first_mut().unwrap().value = format!(
                "{}/{}",
                cauldron.content.get(collectable).unwrap_or(&0),
                cauldron
                    .recipe
                    .ingredients
                    .iter()
                    .find(|&ingredient| &ingredient.collectable == collectable)
                    // we need to ignore None for the frame in which a new recipe gets started
                    .unwrap_or(&Ingredients {
                        amount: 0,
                        collectable: collectable.clone()
                    })
                    .amount
            )
        }
    }
}
