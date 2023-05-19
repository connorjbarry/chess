use std::vec;

use crate::{piece::Piece};

pub static STARTING_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

pub fn get_board_from_fen(fen: &str) -> Vec<u32> {
    let fen_board = fen.split_whitespace().collect::<Vec<&str>>()[0];

    let mut board: Vec<u32> = vec![0; 64];

    let mut rank: u32 = 0;
    let mut file: u32 = 0;

    for c in fen_board.chars() {
        if c == '/' {
            rank += 1;
            file = 0;
        } else {
            if c.is_digit(10) {
                file += c.to_digit(10).unwrap();
            } else {
                let piece = get_piece(c);
                board[(rank * 8 + file) as usize] = piece;
                file += 1;
            }
        }
    }

    board

}

fn get_piece(letter: char) -> u32 {
    let piece = Piece::default();

    let piece_color = if letter.is_uppercase() {
        piece.white
    } else {
        piece.black
    };

    let piece_type = match letter.to_ascii_lowercase() {
        'p' => piece.pawn,
        'n' => piece.knight,
        'b' => piece.bishop,
        'r' => piece.rook,
        'q' => piece.queen,
        'k' => piece.king,
        _ => piece.none,
    };

    piece_color | piece_type
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pawn_conversion() {
        let piece_from_char = get_piece("p".chars().next().unwrap());
        assert_eq!(piece_from_char, 18);
    } 
    #[test] 
    fn test_rook_conversion() {
        let piece_from_char = get_piece("r".chars().next().unwrap());
        assert_eq!(piece_from_char, 21);
    }

    #[test]
    fn test_board_from_fen() {
        let board_from_fen = get_board_from_fen(STARTING_FEN);
        assert_eq!(board_from_fen.len(), 64);
        assert_eq!(board_from_fen[0], 21);
        assert_eq!(board_from_fen[63], 13);
    }
}