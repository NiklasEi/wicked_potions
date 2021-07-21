use crate::board::{Board, Cauldron};
use crate::matcher::{Collectable, Slot};
use crate::{GameState, SystemLabels};
use bevy::prelude::*;
use std::cmp::min;
use std::f32::consts::PI;

pub struct AnimatePlugin;

impl Plugin for AnimatePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_update(GameState::Playing)
                .label(SystemLabels::Animate)
                .with_system(move_collectables.system())
                .with_system(animate_collectables.system())
                .with_system(reset_animations.system()),
        );
    }
}

pub struct Move {
    pub goal: Vec2,
    pub speed: f32,
    pub process_for_cauldron: bool,
    pub throw_in_cauldron: bool,
    origin: Option<Vec2>,
}

pub struct Animate {
    pub frames: u32,
    pub loop_animation: bool,
}

impl Move {
    pub fn move_to(goal: Vec2) -> Self {
        Move {
            goal,
            speed: 256.,
            process_for_cauldron: false,
            throw_in_cauldron: false,
            origin: None,
        }
    }

    pub fn move_to_slot(slot: &Slot) -> Self {
        Move {
            goal: Vec2::new(
                slot.column as f32 * 64. + 32. + 12.,
                slot.row as f32 * 64. + 32. + 12.,
            ),
            speed: 256.,
            process_for_cauldron: false,
            throw_in_cauldron: false,
            origin: None,
        }
    }

    pub fn process() -> Self {
        Move {
            goal: Vec2::new(800. - 132., 300.),
            speed: 256.,
            process_for_cauldron: true,
            throw_in_cauldron: false,
            origin: None,
        }
    }

    pub fn throw_in_cauldron() -> Self {
        Move {
            goal: Vec2::new(800. - 132., 128. + 8.),
            speed: 256.0,
            process_for_cauldron: false,
            throw_in_cauldron: true,
            origin: Some(Vec2::new(700., 300.)),
        }
    }
}

fn move_collectables(
    mut commands: Commands,
    mut board: ResMut<Board>,
    mut animations: Query<(Entity, &Collectable, &mut Transform, &mut Vec<Move>)>,
    mut cauldron: ResMut<Cauldron>,
    time: Res<Time>,
) {
    let mut count = 0;
    let delta = time.delta().as_secs_f32();
    for (entity, collectable, mut transform, mut animations) in animations.iter_mut() {
        let animate = animations.first().unwrap();
        count += 1;
        let diff =
            animate.goal.clone() - Vec2::new(transform.translation.x, transform.translation.y);
        let movement = delta * animate.speed;
        if diff.length() < (delta * animate.speed) {
            transform.translation.x = animate.goal.x;
            transform.translation.y = animate.goal.y;
            if animate.process_for_cauldron {
                animations.remove(0);
                animations.insert(0, Move::throw_in_cauldron());
            } else if animate.throw_in_cauldron {
                // Todo send event with collectable to add to UI / cauldron
                commands.entity(entity).despawn();
                let current = cauldron.content.get(collectable).unwrap_or(&0);
                if !cauldron.recipe.ingredients.iter().any(|ingredient| {
                    &ingredient.collectable == collectable && &ingredient.amount > current
                }) {
                    // ToDo extra points?
                    continue;
                }
                *cauldron.content.entry(collectable.clone()).or_insert(0) += 1;
            } else {
                if animations.len() == 1 {
                    commands.entity(entity).remove::<Vec<Move>>();
                } else {
                    animations.remove(0);
                }
            }
        } else {
            if animate.throw_in_cauldron {
                let process =
                    1. - (diff.length() / (animate.goal - animate.origin.unwrap()).length());
                transform.rotation = Quat::from_rotation_z(process * 2. * PI);
                transform.scale = Vec3::new(1. - process * 0.75, 1. - process * 0.75, 1.);
            }
            let movement = diff.normalize() * movement;
            transform.translation.x += movement.x;
            transform.translation.y += movement.y;
        }
    }
    board.animating = count > 0;
}

struct AnimationTimer {
    timer: Timer,
}

impl Default for AnimationTimer {
    fn default() -> Self {
        AnimationTimer {
            timer: Timer::from_seconds(0.2, true),
        }
    }
}

fn animate_collectables(
    time: Res<Time>,
    mut timer: Local<AnimationTimer>,
    mut animations: Query<(&mut TextureAtlasSprite, &Animate)>,
) {
    timer.timer.tick(time.delta());
    if timer.timer.just_finished() {
        for (mut sprite, animation) in animations.iter_mut() {
            let Animate {
                frames,
                loop_animation,
            } = animation;
            if *loop_animation {
                sprite.index = (sprite.index + 1) % frames;
            } else {
                sprite.index = min(sprite.index + 1, frames - 1);
            }
        }
    }
}

fn reset_animations(
    mut animations: Query<&mut TextureAtlasSprite, (Without<Animate>, Changed<Animate>)>,
) {
    for mut sprite in animations.iter_mut() {
        sprite.index = 0;
        println!("reset");
    }
}
