use bevy::prelude::States;

pub mod game;
pub mod loading;
pub mod main_menu;
pub mod results;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum State {
    #[default]
    Loading,
    MainMenu,
    Game,
    Results,
}
