mod board;
use crate::board::Board;

use bevy::prelude::*;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Chess".to_string(),
            resolution: (800., 800.).into(),
            ..Default::default()
        }),
        ..default()
    }))
    .add_plugin(BoardPlugin)
    .run();
}

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(Board::board_setup);
    }
}