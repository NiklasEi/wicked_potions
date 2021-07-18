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
    mut animations: Query<(Entity, &mut Transform, &Animate)>,
    time: Res<Time>,
) {
    let delta = time.delta().as_secs_f32();
    for (entity, mut transform, animate) in animations.iter_mut() {
        let diff = animate.goal - Vec2::new(transform.translation.x, transform.translation.y);
        let movement = delta * animate.speed;
        if diff.length() < (delta * animate.speed) {
            transform.translation.x = animate.goal.x;
            transform.translation.y = animate.goal.y;
            commands.entity(entity).remove::<Animate>();
        } else {
            let movement = diff.normalize() * movement;
            transform.translation.x += movement.x;
            transform.translation.y += movement.y;
        }
    }
}
