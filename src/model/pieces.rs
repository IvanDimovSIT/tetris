use crate::model::Piece;

use super::{Color, PieceType};

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
        PieceType::SquarePiece
    }
}
impl SquarePiece {
    pub fn new(position: (i32, i32)) -> SquarePiece {
        let mut square_piece = SquarePiece{position: position, squares: vec![]};
        square_piece.squares.push((0,0));
        square_piece.squares.push((1,0));
        square_piece.squares.push((0,1));
        square_piece.squares.push((1,1));

        square_piece
    }
}


enum Rotation {
    Up,
    Right,
    Down,
    Left,
}
impl Rotation {
    fn rotate_right(&mut self) {
        match self {
            Rotation::Up => {*self = Rotation::Right},
            Rotation::Right => {*self = Rotation::Down},
            Rotation::Down => {*self = Rotation::Left},
            Rotation::Left => {*self = Rotation::Up},
        }
    }

    fn rotate_left(&mut self) {
        match self {
            Rotation::Up => {*self = Rotation::Left},
            Rotation::Right => {*self = Rotation::Up},
            Rotation::Down => {*self = Rotation::Right},
            Rotation::Left => {*self = Rotation::Down},
        }
    }
}


pub struct LPiece {
    position: (i32, i32),
    squares: Vec<Vec<(i32, i32)>>,
    rotation: Rotation
}
impl Piece for LPiece {
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
        PieceType::LPiece
    }
}
impl LPiece {
    pub fn new(position: (i32, i32)) -> LPiece {
        let mut l_piece = LPiece{position: position, squares: vec![], rotation: Rotation::Up};
        l_piece.squares.push(vec![]);
        l_piece.squares[0].push((0,0));
        l_piece.squares[0].push((1,0));
        l_piece.squares[0].push((1,1));
        l_piece.squares[0].push((1,2));

        l_piece.squares.push(vec![]);
        l_piece.squares[1].push((1,0));
        l_piece.squares[1].push((1,1));
        l_piece.squares[1].push((0,1));
        l_piece.squares[1].push((-1,1));

        l_piece.squares.push(vec![]);
        l_piece.squares[2].push((0,0));
        l_piece.squares[2].push((0,1));
        l_piece.squares[2].push((0,2));
        l_piece.squares[2].push((1,2));

        l_piece.squares.push(vec![]);
        l_piece.squares[3].push((0,0));
        l_piece.squares[3].push((1,0));
        l_piece.squares[3].push((-1,0));
        l_piece.squares[3].push((-1,1));

        l_piece
    }
}


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
        PieceType::SPiece
    }

}
impl SPiece {
    pub fn new(position: (i32, i32)) -> SPiece {
        let mut s_piece = SPiece{position: position, squares: vec![], rotation: Rotation::Up};
        s_piece.squares.push(vec![]);
        s_piece.squares[0].push((0,0));
        s_piece.squares[0].push((1,0));
        s_piece.squares[0].push((0,1));
        s_piece.squares[0].push((-1,1));

        s_piece.squares.push(vec![]);
        s_piece.squares[1].push((0,0));
        s_piece.squares[1].push((0,1));
        s_piece.squares[1].push((1,1));
        s_piece.squares[1].push((1,2));

        s_piece
    }
}



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
        PieceType::JPiece
    }

}
impl JPiece {
    pub fn new(position: (i32, i32)) -> JPiece {
        let mut j_piece = JPiece{position: position, squares: vec![], rotation: Rotation::Up};
        j_piece.squares.push(vec![]);
        j_piece.squares[0].push((0,0));
        j_piece.squares[0].push((1,0));
        j_piece.squares[0].push((0,1));
        j_piece.squares[0].push((0,2));

        j_piece.squares.push(vec![]);
        j_piece.squares[1].push((0,0));
        j_piece.squares[1].push((1,0));
        j_piece.squares[1].push((-1,0));
        j_piece.squares[1].push((1,1));

        j_piece.squares.push(vec![]);
        j_piece.squares[2].push((0,0));
        j_piece.squares[2].push((0,1));
        j_piece.squares[2].push((0,2));
        j_piece.squares[2].push((-1,2));

        j_piece.squares.push(vec![]);
        j_piece.squares[3].push((-1,0));
        j_piece.squares[3].push((-1,1));
        j_piece.squares[3].push((0,1));
        j_piece.squares[3].push((1,1));

        j_piece
    }
}


pub struct ZPiece {
    position: (i32, i32),
    squares: Vec<Vec<(i32, i32)>>,
    rotation: Rotation
}
impl Piece for ZPiece {
    fn get_position(&self) -> (i32, i32) {
        self.position
    }

    fn set_position(&mut self, position: (i32, i32)) {
        self.position.0 = position.0;
        self.position.1 = position.1;
        
    }

    fn get_color(&self) -> Color {
        Color::Green
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
        PieceType::ZPiece
    }
}
impl ZPiece {
    pub fn new(position: (i32, i32)) -> ZPiece {
        let mut z_piece = ZPiece{position: position, squares: vec![], rotation: Rotation::Up};
        z_piece.squares.push(vec![]);
        z_piece.squares[0].push((0,0));
        z_piece.squares[0].push((-1,0));
        z_piece.squares[0].push((0,1));
        z_piece.squares[0].push((1,1));

        z_piece.squares.push(vec![]);
        z_piece.squares[1].push((0,0));
        z_piece.squares[1].push((0,1));
        z_piece.squares[1].push((-1,1));
        z_piece.squares[1].push((-1,2));

        z_piece
    }
}


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
        PieceType::LinePiece
    }
}
impl LinePiece {
    pub fn new(position: (i32, i32)) -> LinePiece {
        let mut line_piece = LinePiece{position: position, squares: vec![], rotation: Rotation::Up};
        line_piece.squares.push(vec![]);
        line_piece.squares[0].push((0,0));
        line_piece.squares[0].push((0,1));
        line_piece.squares[0].push((0,2));
        line_piece.squares[0].push((0,3));

        line_piece.squares.push(vec![]);
        line_piece.squares[1].push((0,0));
        line_piece.squares[1].push((-1,0));
        line_piece.squares[1].push((1,0));
        line_piece.squares[1].push((2,0));

        line_piece
    }
}


pub struct TPiece {
    position: (i32, i32),
    squares: Vec<Vec<(i32, i32)>>,
    rotation: Rotation
}
impl Piece for TPiece {
    fn get_position(&self) -> (i32, i32) {
        self.position
    }

    fn set_position(&mut self, position: (i32, i32)) {
        self.position.0 = position.0;
        self.position.1 = position.1;
        
    }

    fn get_color(&self) -> Color {
        Color::Green
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
        PieceType::TPiece
    }
}
impl TPiece {
    pub fn new(position: (i32, i32)) -> TPiece {
        let mut t_piece = TPiece{position: position, squares: vec![], rotation: Rotation::Up};
        t_piece.squares.push(vec![]);
        t_piece.squares[0].push((0,0));
        t_piece.squares[0].push((0,1));
        t_piece.squares[0].push((-1,1));
        t_piece.squares[0].push((1,1));

        t_piece.squares.push(vec![]);
        t_piece.squares[1].push((0,0));
        t_piece.squares[1].push((0,1));
        t_piece.squares[1].push((1,1));
        t_piece.squares[1].push((0,2));

        t_piece.squares.push(vec![]);
        t_piece.squares[2].push((0,0));
        t_piece.squares[2].push((1,0));
        t_piece.squares[2].push((-1,0));
        t_piece.squares[2].push((0,1));

        t_piece.squares.push(vec![]);
        t_piece.squares[3].push((0,0));
        t_piece.squares[3].push((0,1));
        t_piece.squares[3].push((-1,1));
        t_piece.squares[3].push((0,2));

        t_piece
    }
}
