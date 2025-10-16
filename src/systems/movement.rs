use crate::components::{Player, Velocity};
use bevy::prelude::*;

pub fn player_movement(
    keyboard: Res<ButtonInput<Key>>,
    mut query: Query<(&mut Transform, &Velocity), With<Player>>,
) {
    for (mut transform, vel) in &mut query {
        let mut direction = Vec3::ZERO;

        if keyboard.pressed(Key::W) {
            direction.y += 1.0;
        }
        if keyboard.pressed(Key::S) {
            direction.y -= 1.0;
        }
        if keyboard.pressed(Key::A) {
            direction.x += 1.0;
        }
        if keyboard.pressed(Key::D) {
            direction.x -= 1.0;
        }

        transform.translation += direction.normalize_or_zero() * 100.0 * vel.x;
    }
}
