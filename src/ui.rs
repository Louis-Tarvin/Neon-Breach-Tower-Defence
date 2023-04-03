use bevy::prelude::*;

#[derive(Resource, Debug, Default)]
pub struct UIState {
    pub hovered_pos: Option<(i8, i8)>,
}
