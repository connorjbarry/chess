use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Chess".to_string(),
                resolution: (600., 600.).into(),
                ..Default::default()
            }),
            ..default()
        }))
        .add_startup_system(chess_board_setup_system)
        //  .add_startup_system(piece_setup_system)
        .run();
}

fn chess_board_setup_system(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    for i in 0..8 {
        for j in 0..8 {
            let color = if (i + j) % 2 == 0 {
                Color::WHITE
            } else {
                Color::BLACK
            };
            commands.spawn(SpriteBundle {
                sprite: Sprite {
                    color,
                    custom_size: Some(Vec2::new(75.0, 75.0)),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(
                    -300. + (j as f32) * 75., // -300
                    300. - (i as f32) * 75.,  // 300
                    0.,
                )),
                ..default()
            });
        }
    }
}
