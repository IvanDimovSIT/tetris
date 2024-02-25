use super::{Piece, Color, PieceType};

pub struct SquarePiece {
    position: (i32, i32),
    squares: Vec<(i32, i32)>,
}
impl Piece for SquarePiece {
    fn get_position(&self) -> (i32, i32) {
        self.position
    }

    fn set_position(&mut self, position: (i32, i32)) {
        self.position.0 = position.0;
        self.position.1 = position.1;
        
    }

    fn get_color(&self) -> Color {
        Color::Yellow
    }

    fn get_squares(&self) -> Vec<(i32, i32)> {
        self.squares
            .iter()
            .map(|x| {(x.0+self.position.0, x.1+self.position.1)})
            .collect()
    }
    
    fn rotate_left(&mut self) -> bool {
        false
    }

    fn rotate_right(&mut self) -> bool {
        false
    }

    fn get_type(&self) -> PieceType {
        PieceType::Square
    }
}
impl SquarePiece {
    pub fn new(position: (i32, i32)) -> SquarePiece {
        let mut square_piece = SquarePiece{position, squares: vec![]};
        square_piece.squares.push((0,0));
        square_piece.squares.push((1,0));
        square_piece.squares.push((0,1));
        square_piece.squares.push((1,1));

        square_piece
    }
}