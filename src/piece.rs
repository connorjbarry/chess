pub struct Piece {
    pub none: u32,
    pub pawn: u32,
    pub knight: u32,
    pub bishop: u32,
    pub rook: u32,
    pub queen: u32,
    pub king: u32,

    pub white: u32,
    pub black: u32,
}

impl Default for Piece {
    fn default() -> Piece {
        Piece {
            none: 0,
            king: 1,
            pawn: 2,
            knight: 3,
            bishop: 4,
            rook: 5,
            queen: 6,

            white: 8,
            black: 16,
        }
    }
}