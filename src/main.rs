mod scoring;
mod state;
mod systems;

use bevy::prelude::*;
use scoring::{init_scoreboard, update_score, Score};
use systems::{
    setup,
    ui::{init_paddle, Paddle},
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
        .insert_resource(Score::new())
        .add_systems(Startup, setup)
        .add_systems(Startup, (init_paddle, init_scoreboard))
        .add_systems(Update, (move_paddle, update_score))
        .run();
}

fn move_paddle(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut transform: Single<&mut Transform, With<Paddle>>,
) {
    if keyboard_input.pressed(KeyCode::ArrowRight) {
        println!("-> pressed!");
        transform.translation.x += 10.0;
    } else if keyboard_input.pressed(KeyCode::ArrowLeft) {
        println!("<- pressed!");
        transform.translation.x -= 10.0;
    }
}
