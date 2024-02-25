use super::{Rotation, Piece, Color, PieceType};

pub struct SPiece {
    position: (i32, i32),
    squares: Vec<Vec<(i32, i32)>>,
    rotation: Rotation
}
impl Piece for SPiece {
    fn get_position(&self) -> (i32, i32) {
        self.position
    }

    fn set_position(&mut self, position: (i32, i32)) {
        self.position.0 = position.0;
        self.position.1 = position.1;
        
    }

    fn get_color(&self) -> Color {
        Color::Red
    }

    fn get_squares(&self) -> Vec<(i32, i32)> {
        match self.rotation {
            Rotation::Up | Rotation::Down => {
                &self.squares[0]
            }
            Rotation::Right | Rotation::Left => {
                &self.squares[1]
            }
        }
            .iter()
            .map(|x| {(x.0+self.position.0, x.1+self.position.1)})
            .collect()
    }
    
    fn rotate_right(&mut self) -> bool {
        self.rotation.rotate_right();
        
        true
    }

    fn rotate_left(&mut self) -> bool {
        self.rotation.rotate_left();

        true
    }

    fn get_type(&self) -> PieceType {
        PieceType::S
    }

}
impl SPiece {
    pub fn new(position: (i32, i32)) -> SPiece {
        let mut s_piece = SPiece{position: (position.0, position.1+2), squares: vec![], rotation: Rotation::Up};
        s_piece.squares.push(vec![]);
        s_piece.squares[0].push((0,0));
        s_piece.squares[0].push((-1,0));
        s_piece.squares[0].push((0,-1));
        s_piece.squares[0].push((1,-1));

        s_piece.squares.push(vec![]);
        s_piece.squares[1].push((0,0));
        s_piece.squares[1].push((0,-1));
        s_piece.squares[1].push((-1,-1));
        s_piece.squares[1].push((-1,-2));

        s_piece
    }
}