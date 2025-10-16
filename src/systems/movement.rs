use crate::components::{Player, Velocity};
use bevy::prelude::*;

pub fn player_movement(
    keyboard: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &Velocity), With<Player>>,
) {
    for (mut transform, vel) in &mut query {
        let mut direction = Vec3::ZERO;

        if keyboard.pressed(KeyCode::W) {
            direction.y += 1.0;
        }
        if keyboard.pressed(Keycode::S) {
            direction.y -= 1.0;
        }
        if keyboard.pressed(KeyCode::A) {
            direction.x += 1.0;
        }
        if keyboard.pressed(KeyCode::D) {
            direction.x -= 1.0;
        }

        transform.translation += direction.normalize_or_zero() * 100.0 * vel.x;
    }
}
