pub struct Piece {
    pub None: u32,
    pub Pawn: u32,
    pub Knight: u32,
    pub Bishop: u32,
    pub Rook: u32,
    pub Queen: u32,
    pub King: u32,

    pub White: u32,
    pub Black: u32,
}

impl Piece {
    fn new() -> Piece {
        Piece {
            None: 0,
            King: 1,
            Pawn: 2,
            Knight: 3,
            Bishop: 4,
            Rook: 5,
            Queen: 6,

            White: 8,
            Black: 16,
        }
    }
}