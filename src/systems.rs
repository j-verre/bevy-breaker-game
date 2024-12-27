use bevy::prelude::{Camera2d, Commands};

pub fn initialize(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}

pub mod audio {
    use bevy::{
        asset::AssetServer,
        audio::{AudioPlayer, PlaybackSettings},
        prelude::{Commands, Res},
    };

    pub fn play_background_music(asset_server: Res<AssetServer>, mut commands: Commands) {
        commands.spawn((
            AudioPlayer::new(asset_server.load("audio/music_loop.ogg")),
            PlaybackSettings::LOOP,
        ));
    }

    #[allow(dead_code)]
    pub fn play_sfx(_asset_server: Res<AssetServer>, mut _commands: Commands) {
        todo!()
    }
}

pub mod ui {
    use bevy::{
        color::Color,
        math::{Vec2, Vec3},
        prelude::{Commands, Transform, Visibility},
        sprite::Sprite,
    };

    pub fn create_paddle(mut commands: Commands) {
        commands.spawn((
            Sprite {
                color: Color::srgb(0.5, 0.5, 1.0),
                custom_size: Some(Vec2::new(120.0, 20.0)),
                ..Default::default()
            },
            Transform {
                translation: Vec3::new(0.0, -300.0, 0.0),
                ..Default::default()
            },
            Visibility::default(),
        ));
    }
}
