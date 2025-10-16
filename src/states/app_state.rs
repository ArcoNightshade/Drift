use bevy::prelude::States;
use bevy::prelude::*;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default, States)]
pub enum AppState {
    #[default]
    MainMenu,
    InGame,
}
