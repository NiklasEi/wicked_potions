use crate::board::Board;
use crate::matcher::Slot;
use crate::{GameState, SystemLabels};
use bevy::prelude::*;
use std::cmp::min;

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
}
pub struct Animate {
    pub frames: u32,
    pub loop_animation: bool,
}

impl Move {
    pub fn move_to_slot(slot: &Slot) -> Self {
        Move {
            goal: Vec2::new(slot.column as f32 * 64. + 32., slot.row as f32 * 64. + 32.),
            speed: 256.,
        }
    }
}

fn move_collectables(
    mut commands: Commands,
    mut board: ResMut<Board>,
    mut animations: Query<(Entity, &mut Transform, &mut Vec<Move>)>,
    time: Res<Time>,
) {
    let mut count = 0;
    let delta = time.delta().as_secs_f32();
    for (entity, mut transform, mut animations) in animations.iter_mut() {
        let animate = animations.first().unwrap();
        count += 1;
        let diff =
            animate.goal.clone() - Vec2::new(transform.translation.x, transform.translation.y);
        let movement = delta * animate.speed;
        if diff.length() < (delta * animate.speed) {
            transform.translation.x = animate.goal.x;
            transform.translation.y = animate.goal.y;
            if animations.len() == 1 {
                commands.entity(entity).remove::<Vec<Move>>();
            } else {
                animations.remove(0);
            }
        } else {
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
