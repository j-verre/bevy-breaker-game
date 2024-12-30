use audio::initialize_audio;
use bevy::{
    asset::AssetServer,
    prelude::{Camera2d, Commands, Res},
};

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d::default());
    initialize_audio(&mut commands, &asset_server);
}

pub mod audio {
    use bevy::{
        asset::{AssetServer, Handle},
        audio::{AudioPlayer, AudioSource, PlaybackSettings},
        prelude::{Commands, Res, Resource},
    };

    #[allow(dead_code)]
    #[derive(Resource)]
    struct CollisionSound(Handle<AudioSource>);

    #[allow(dead_code)]
    #[derive(Resource)]
    struct BackgroundMusic(Handle<AudioSource>);

    pub fn initialize_audio(commands: &mut Commands, asset_server: &Res<AssetServer>) {
        // Add any sound resources needed later the game
        let sfx = asset_server.load("audio/sfx/digital-two-tone.wav");
        commands.insert_resource(CollisionSound(sfx));

        let music = asset_server.load("audio/music/elevator_music.ogg");
        commands.insert_resource(CollisionSound(music.clone()));
        commands.spawn((AudioPlayer::new(music), PlaybackSettings::LOOP));
    }
}

pub mod ui {
    use bevy::{
        color::Color,
        math::{Vec2, Vec3},
        prelude::{Commands, Component, Transform, Visibility},
        sprite::Sprite,
        utils::default,
    };

    #[derive(Component)]
    pub struct Paddle;

    pub fn init_paddle(mut commands: Commands) {
        commands.spawn((
            Paddle,
            Sprite {
                color: Color::srgb(0.5, 0.5, 1.0),
                custom_size: Some(Vec2::new(120.0, 20.0)),
                ..default()
            },
            Transform {
                translation: Vec3::new(0.0, -300.0, 0.0),
                ..default()
            },
            Visibility::default(),
        ));
    }
}
