use bevy::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Component)]
pub enum GameStates {
    Main,
    ConsoleLoading,
    Console,
    PauseMenu,
    TabMenuLoading,
    TabMenu,
    MainMenu,
}
