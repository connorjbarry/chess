use std::collections::HashMap;

use crate::fen::{get_board_from_fen, STARTING_FEN};
use crate::piece::Piece;
use bevy::prelude::*;

pub struct BoardSetup {}

pub struct Board {
    pub squares: Vec<u32>,
}

impl BoardSetup {
    pub fn setup_squares(mut commands: Commands) {
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

    pub fn setup_piece(mut commands: Commands, asset_server: Res<AssetServer>) {
        let piece = Piece::default();
        let pieceToImageMap = HashMap::from([
            (piece.white | piece.pawn, "wp.png"),
            (piece.white | piece.knight, "wN.png"),
            (piece.white | piece.bishop, "wB.png"),
            (piece.white | piece.rook, "wR.png"),
            (piece.white | piece.queen, "wQ.png"),
            (piece.white | piece.king, "wK.png"),
            (piece.black | piece.pawn, "bp.png"),
            (piece.black | piece.knight, "bN.png"),
            (piece.black | piece.bishop, "bB.png"),
            (piece.black | piece.rook, "bR.png"),
            (piece.black | piece.queen, "bQ.png"),
            (piece.black | piece.king, "bK.png"),
        ]);
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
