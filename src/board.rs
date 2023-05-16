use bevy::prelude::*;

pub struct BoardSetup {}

impl BoardSetup {
    pub fn setup(mut commands: Commands) {
        commands.spawn(Camera2dBundle::default());
    
        let square_size = 50;
    
        for x in 0..8 {
            for y in 0..8 {
                let color = if (x + y) % 2 == 0 {
                    Color::WHITE
                } else {
                    Color::BLACK 
                };
    
                commands.spawn(SpriteBundle {
                    sprite: Sprite {
                        color,
                        custom_size: Some(Vec2::new(square_size as f32, square_size as f32)),
                        ..default()
                    },
                    transform: Transform::from_translation(Vec3::new((x * square_size - 200) as f32, (y * square_size - 150) as f32, 0.0)),
                    ..Default::default()
                });
            }
        }
    }
}
