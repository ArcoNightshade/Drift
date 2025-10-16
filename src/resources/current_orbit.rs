use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct CurrentOrbit {
    pub altitude: f32,
    pub decay_rate: f32,
}
