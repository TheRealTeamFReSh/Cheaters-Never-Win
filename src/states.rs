use bevy::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Component)]
pub enum GameStates {
    Main,
    ConsoleLoading,
    Console,
    TabMenuLoading,
    TabMenu,
}