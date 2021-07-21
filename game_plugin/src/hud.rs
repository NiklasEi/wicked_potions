use crate::board::Cauldron;
use crate::loading::{FontAssets, TextureAssets};
use crate::matcher::Collectable;
use crate::GameState;
use bevy::prelude::*;

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(SystemSet::on_enter(GameState::Playing).with_system(setup_ui.system()))
            .add_system_set(
                SystemSet::on_update(GameState::Playing).with_system(update_recipe.system()),
            );
    }
}

struct Ui;

fn setup_ui(
    mut commands: Commands,
    cauldron: Res<Cauldron>,
    fonts: Res<FontAssets>,
    textures: Res<TextureAssets>,
) {
    let mut index = 0;
    for ingredient in cauldron.recipe.ingredients.iter() {
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
                transform: Transform::from_translation(Vec3::new(
                    800. - (index + 1) as f32 * 76. + 18.,
                    550.,
                    10.,
                )),
                ..Text2dBundle::default()
            })
            .insert(Ui)
            .insert(ingredient.collectable.clone());
        let mut icon_transform = Transform::from_translation(Vec3::new(
            800. - (index + 1) as f32 * 76. + 38.,
            550. + 7.,
            10.,
        ));
        icon_transform.scale = Vec3::new(0.5, 0.5, 0.5);
        commands.spawn_bundle(SpriteSheetBundle {
            texture_atlas: ingredient.collectable.get_texture(&textures),
            transform: icon_transform,
            sprite: TextureAtlasSprite {
                index: ingredient.collectable.get_animation().frames - 1,
                ..TextureAtlasSprite::default()
            },
            ..SpriteSheetBundle::default()
        });
        index += 1;
    }
}

fn update_recipe(cauldron: Res<Cauldron>, mut text: Query<(&mut Text, &Collectable), With<Ui>>) {
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
                    .unwrap()
                    .amount
            )
        }
    }
}
