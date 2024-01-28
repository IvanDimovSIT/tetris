use crate::model::Piece;

use super::Color;

struct SquarePiece {
    position: (usize, usize),
    squares: Vec<(usize, usize)>,
}
impl Piece for SquarePiece {
    fn get_position(&self) -> (usize, usize) {
        self.position
    }

    fn set_position(&mut self, position: (usize, usize)) {
        self.position.0 = position.0;
        self.position.1 = position.1;
        
    }

    fn get_color(&self) -> Color {
        Color::Yellow
    }

    fn get_squares(&self) -> Vec<(usize, usize)> {
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
}

enum Rotation {
    Up,
    Right,
    Down,
    Left,
}

struct LPiece {
    position: (usize, usize),
    squares: Vec<Vec<(usize, usize)>>,
    rotation: Rotation
}
impl Piece for LPiece {
    fn get_position(&self) -> (usize, usize) {
        self.position
    }

    fn set_position(&mut self, position: (usize, usize)) {
        self.position.0 = position.0;
        self.position.1 = position.1;
        
    }

    fn get_color(&self) -> Color {
        Color::Yellow
    }

    fn get_squares(&self) -> Vec<(usize, usize)> {
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
    
    fn rotate_left(&mut self) -> bool {
        todo!();
        true
    }

    fn rotate_right(&mut self) -> bool {
        todo!();
        true
    }
}