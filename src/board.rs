use bevy::prelude::*;
use crate::fen::{get_board_from_fen, STARTING_FEN};

pub struct BoardSetup {}

pub struct Board {
    pub squares: Vec<u32>,
}

impl BoardSetup {
    pub fn setup_squares(mut commands: Commands) {
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

    pub fn setup_piece(mut commands: Commands, asset_server: Res<AssetServer>) {
        commands.spawn(Camera2dBundle::default());
    }
}


impl Board {
    pub fn new(&mut self) -> Board {
        // we will initialize the array from fen notation
        self.squares = get_board_from_fen(STARTING_FEN);

        Board {
            squares: self.squares.clone(),
        }
    }
}