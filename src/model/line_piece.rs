use super::{Rotation, Piece, Color, PieceType};

pub struct LinePiece {
    position: (i32, i32),
    squares: Vec<Vec<(i32, i32)>>,
    rotation: Rotation
}
impl Piece for LinePiece {
    fn get_position(&self) -> (i32, i32) {
        self.position
    }

    fn set_position(&mut self, position: (i32, i32)) {
        self.position.0 = position.0;
        self.position.1 = position.1;
        
    }

    fn get_color(&self) -> Color {
        Color::Blue
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
        PieceType::Line
    }
}
impl LinePiece {
    pub fn new(position: (i32, i32)) -> LinePiece {
        let mut line_piece = LinePiece{position: (position.0, position.1+3), squares: vec![], rotation: Rotation::Up};
        line_piece.squares.push(vec![]);
        line_piece.squares[0].push((0,0));
        line_piece.squares[0].push((0,-1));
        line_piece.squares[0].push((0,-2));
        line_piece.squares[0].push((0,-3));

        line_piece.squares.push(vec![]);
        line_piece.squares[1].push((0,0));
        line_piece.squares[1].push((-1,0));
        line_piece.squares[1].push((1,0));
        line_piece.squares[1].push((2,0));

        line_piece
    }
}
