#![allow(clippy::too_many_arguments, clippy::type_complexity)]

use audio::{AudioAssets, DrumsChannel, MusicChannel, SoundChannel};
use bevy::prelude::*;
use bevy::window::WindowMode;
use bevy_asset_loader::prelude::{LoadingState, LoadingStateAppExt};
use bevy_kira_audio::{AudioApp, AudioPlugin};
use state::{
    game::GamePlugin, loading::GameAssets, main_menu::MainMenuPlugin, results::ResultsPlugin,
};
use ui::constants::BACKGROUND_COLOR;

mod audio;
mod enemies;
mod gameplay;
mod grid;
mod input;
mod state;
mod tower;
mod ui;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugin(AudioPlugin)
        .add_state::<state::State>()
        .add_loading_state(
            LoadingState::new(state::State::Loading).continue_to_state(state::State::MainMenu),
        )
        .add_collection_to_loading_state::<_, GameAssets>(state::State::Loading)
        .add_collection_to_loading_state::<_, AudioAssets>(state::State::Loading)
        .add_plugin(MainMenuPlugin)
        .add_plugin(GamePlugin)
        .add_plugin(ResultsPlugin)
        .add_startup_system(state::results::create_player)
        .add_system(state::loading::setup.in_schedule(OnEnter(state::State::Loading)))
        .add_system(state::loading::cleanup.in_schedule(OnExit(state::State::Loading)))
        .add_audio_channel::<MusicChannel>()
        .add_audio_channel::<DrumsChannel>()
        .add_audio_channel::<SoundChannel>()
        .insert_resource(audio::VolumeSettings::default())
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .run();
}
