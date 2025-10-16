use crate::resources::CurrentOrbit;
use bevy::prelude::*;

pub fn update_decay(mut orbit: ResMut<CurrentOrbit>, time: Res<Time>) {
    orbit.altitude -= orbit.decay_rate * time.delta_seconds();
}
