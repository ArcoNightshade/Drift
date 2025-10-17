use bevy::prelude::*;
use crate::components::{Player, Velocity};

pub fn movement_system(mut query: Query<(&Velocity, &mut Transform), With<Player>>) {
    for (vel, mut transform) in &mut query {
        transform.translation.x += vel.x;
        transform.translation.y += vel.y;
    }
}
