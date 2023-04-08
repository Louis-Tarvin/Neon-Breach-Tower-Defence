use bevy::{input::common_conditions::input_toggle_active, prelude::*};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use crate::state::State;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(WorldInspectorPlugin::new().run_if(input_toggle_active(false, KeyCode::F1)))
            .add_system(trigger_game_over);
    }
}

pub fn trigger_game_over(mut next_state: ResMut<NextState<State>>, input: Res<Input<KeyCode>>) {
    if input.just_pressed(KeyCode::F2) {
        next_state.set(State::MainMenu);
    }
}
