use crate::board::Board;
use crate::GameState;
use bevy::prelude::*;

pub struct AnimatePlugin;

impl Plugin for AnimatePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(Animation::Running)
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(animate.system()));
    }
}

pub enum Animation {
    None,
    Running,
}

pub struct Animate {
    pub goal: Vec2,
    pub speed: f32,
}

fn animate(
    mut commands: Commands,
    mut board: ResMut<Board>,
    mut animations: Query<(Entity, &mut Transform, &mut Vec<Animate>)>,
    time: Res<Time>,
) {
    let mut count = 0;
    let delta = time.delta().as_secs_f32();
    for (entity, mut transform, mut animations) in animations.iter_mut() {
        count += 1;
        let animate = animations.first().unwrap();
        let diff = animate.goal - Vec2::new(transform.translation.x, transform.translation.y);
        let movement = delta * animate.speed;
        if diff.length() < (delta * animate.speed) {
            transform.translation.x = animate.goal.x;
            transform.translation.y = animate.goal.y;
            if animations.len() == 1 {
                commands.entity(entity).remove::<Vec<Animate>>();
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
