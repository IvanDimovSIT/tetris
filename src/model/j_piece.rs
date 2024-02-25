use super::{Rotation, Piece, Color, PieceType};

pub struct JPiece {
    position: (i32, i32),
    squares: Vec<Vec<(i32, i32)>>,
    rotation: Rotation
}
impl Piece for JPiece {
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
        match self.rotation {
            Rotation::Up => {
                &self.squares[0]
            }
            Rotation::Right => {
                &self.squares[1]
            }
            Rotation::Down => {
                &self.squares[2]
            }
            Rotation::Left => {
                &self.squares[3]
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
        PieceType::J
    }

}
impl JPiece {
    pub fn new(position: (i32, i32)) -> JPiece {
        let mut j_piece = JPiece{position: (position.0, position.1+2), squares: vec![], rotation: Rotation::Up};
        j_piece.squares.push(vec![]);
        j_piece.squares[0].push((0,0));
        j_piece.squares[0].push((0,-1));
        j_piece.squares[0].push((0,-2));
        j_piece.squares[0].push((1,-2));

        j_piece.squares.push(vec![]);
        j_piece.squares[1].push((1,0));
        j_piece.squares[1].push((1,-1));
        j_piece.squares[1].push((0,-1));
        j_piece.squares[1].push((-1,-1));

        j_piece.squares.push(vec![]);
        j_piece.squares[2].push((0,0));
        j_piece.squares[2].push((-1,0));
        j_piece.squares[2].push((0,-1));
        j_piece.squares[2].push((0,-2));

        j_piece.squares.push(vec![]);
        j_piece.squares[3].push((0,0));
        j_piece.squares[3].push((1,0));
        j_piece.squares[3].push((-1,0));
        j_piece.squares[3].push((-1,-1));

        j_piece
    }
}