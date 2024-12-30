use bevy::{
    asset::{AssetServer, Assets, Handle},
    color::Color,
    input::ButtonInput,
    prelude::{
        Commands, Component, Deref, DerefMut, KeyCode, Res, ResMut, Resource, Single, Text, With,
    },
    sprite::ColorMaterial,
    text::{Font, TextColor, TextFont},
    utils::default,
};

const SCOREBOARD_COLOR: Color = Color::srgb(1.0, 0.5, 0.5);

#[derive(Deref, DerefMut, Resource)]
/// Resource that tracks the game's score
pub struct Score(usize);

impl Score {
    pub fn new() -> Self {
        Score(0)
    }
    pub fn increment(&mut self) {
        self.0 += 1
    }
    pub fn get(&self) -> usize {
        self.0
    }
}

#[derive(Component)]
pub struct ScoreboardUi;
const SCOREBOARD_FONT: &str = "fonts/road_rage/RoadRage-Regular.ttf";

pub fn init_scoreboard(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut _materials: ResMut<Assets<ColorMaterial>>,
) {
    let font: Handle<Font> = asset_server.load(SCOREBOARD_FONT);
    let style = TextFont {
        font,
        font_size: 26.0,
        ..default()
    };

    // commands.spawn(Text2d::new("Score: 0"));
    commands.spawn((
        ScoreboardUi,
        Text::new("Score - "),
        style,
        TextColor(SCOREBOARD_COLOR),
    ));
}

pub fn update_score(
    mut score: ResMut<Score>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut text: Single<&mut Text, With<ScoreboardUi>>,
) {
    if keyboard_input.just_pressed(KeyCode::Digit1) {
        score.increment();
        text.0 = format!("Score - {}", score.get().to_string());
    }
}
