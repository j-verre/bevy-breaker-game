mod resources;
mod systems;

use bevy::prelude::*;
use systems::{
    audio::{play_background_music, play_sfx},
    initialize,
    ui::create_paddle,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "\"Breaker!\"".to_string(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, initialize)
        .add_systems(Startup, create_paddle)
        .add_systems(PostStartup, (play_background_music, play_sfx))
        .run();
}
