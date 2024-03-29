use std::collections::VecDeque;

use crate::constants::*;


use self::j_piece::*;
use self::l_piece::*;
use self::line_piece::*;
use self::s_piece::*;
use self::square_piece::*;
use self::z_piece::*;
use self::t_piece::*;
use self::piece_generator::*;


mod j_piece;
mod l_piece;
mod line_piece;
mod s_piece;
mod square_piece;
mod z_piece;
mod t_piece;
mod piece_generator;

#[derive(Clone, Copy)]
pub enum Color {
    Red,
    Green,
    Yellow,
    Blue,
}

#[derive(Clone)]
pub enum Square {
    None,
    Normal(Color),
    Ghost(Color),
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

pub enum PieceType {
    Square,
    L,
    J,
    S,
    Z,
    T,
    Line
}
impl PieceType {
    pub fn create(&self, start_position: (i32, i32)) -> Box<dyn Piece> {
        match self {
            Self::L => {Box::new(LPiece::new(start_position))},
            Self::J => {Box::new(JPiece::new(start_position))},
            Self::S => {Box::new(SPiece::new(start_position))},
            Self::Z => {Box::new(ZPiece::new(start_position))},
            Self::T => {Box::new(TPiece::new(start_position))},
            Self::Line => {Box::new(LinePiece::new(start_position))},
            Self::Square => {Box::new(SquarePiece::new(start_position))},
        }
    }
}

pub trait Piece {
    fn get_position(&self) -> (i32, i32);
    fn set_position(&mut self, position: (i32, i32));
    fn get_color(&self) -> Color;
    fn get_squares(&self) -> Vec<(i32, i32)>;
    fn rotate_left(&mut self) -> bool;
    fn rotate_right(&mut self) -> bool;
    fn get_type(&self) -> PieceType;
}

struct Board {
    squares: Vec<Square>,
    width: usize,
    height: usize,
    active_piece: Box<dyn Piece>,
}
impl Board {
    fn new(width: usize, height: usize, starting_piece: Box<dyn Piece>) -> Result<Board, String> {
        if width < 4 || height < 6 {
            return Err("Invalid width or height".to_string());
        }

        let mut board = Board{width, height, squares: vec![], active_piece: starting_piece};
        for _ in 0..(width*height) {
            board.squares.push(Square::None);
        }

        Ok(board)
    }

    fn get_active_piece_squares(&self) -> (Vec<(i32, i32)>, Color) {
        (self.active_piece.get_squares(), self.active_piece.get_color())
    }

    fn set_square(&mut self, position: (usize, usize), square: Square) {
        debug_assert!(position.0 < self.width && position.1 < self.height);
        self.squares[position.0 + position.1*self.width] = square
    }
    
    fn get_square(&self, position: (usize, usize)) -> Option<&Square> {
        if position.0 >= self.width || position.1 >= self.height {
            None
        }else{
            Some(&self.squares[position.0 + self.width*position.1])
        }
    }

    fn move_active_left(&mut self) -> bool {
        debug_assert!(self.is_active_piece_valid());
        let pos = self.active_piece.get_position();
        self.active_piece.set_position((pos.0-1, pos.1));
        
        if self.is_active_piece_valid() {
            true
        }else{
            self.active_piece.set_position(pos);
            false
        }
    }  

    fn move_active_right(&mut self) -> bool {
        debug_assert!(self.is_active_piece_valid());
        let pos = self.active_piece.get_position();
        self.active_piece.set_position((pos.0+1, pos.1));
        
        if self.is_active_piece_valid() {
            true
        }else{
            self.active_piece.set_position(pos);
            false
        }
    }  

    fn move_active_down(&mut self) -> bool {
        if !self.is_active_piece_valid() {
            return false;
        }

        let pos = self.active_piece.get_position();
        self.active_piece.set_position((pos.0, pos.1+1));
        
        if self.is_active_piece_valid() {
            true
        }else{
            self.active_piece.set_position(pos);
            false
        }
    }

    fn move_active_up(&mut self) -> bool {
        debug_assert!(self.is_active_piece_valid());
        let pos = self.active_piece.get_position();
        self.active_piece.set_position((pos.0, pos.1-1));
        
        if self.is_active_piece_valid() {
            true
        }else{
            self.active_piece.set_position(pos);
            false
        }
    }

    fn rotate_active_left(&mut self) -> bool {
        debug_assert!(self.is_active_piece_valid());
        if !self.active_piece.rotate_left() {
            return false;
        }

        if self.is_active_piece_valid() {
            true
        }else{
            self.active_piece.rotate_right();
            false
        }
    }

    fn rotate_active_right(&mut self) -> bool {
        debug_assert!(self.is_active_piece_valid());
        if !self.active_piece.rotate_right() {
            return false;
        }

        if self.is_active_piece_valid() {
            true
        }else{
            self.active_piece.rotate_left();
            false
        }
    }

    fn are_squares_free(&self, active_squares: &Vec<(i32, i32)>) -> bool {
        for i in active_squares {
            if i.0 < 0 || i.0 >= self.width as i32 || i.1 < 0 || i.1 >= self.height as i32 {
                return false;
            }

            let square = self.get_square((i.0 as usize, i.1 as usize)).unwrap();
            match square {
                Square::Normal(_) => {
                    return false;
                },
                _ => {
                    continue;
                }
            }

        }

        true
    }

    fn remove_ghost_pieces(&mut self) {
        for i in 0..self.squares.len() {
            match self.squares[i] {
                Square::Ghost(_) => { self.squares[i] = Square::None}
                _ => {continue;},
            }
        }
    }

    fn position_ghost_pieces(&mut self) {
        self.remove_ghost_pieces();

        let mut active_squares = self.active_piece.get_squares();

        while self.are_squares_free(&active_squares) {
            active_squares = active_squares
                .iter()
                .map(|x| {(x.0, x.1+1)})
                .collect();
        }

        active_squares = active_squares
                .iter()
                .map(|x| {(x.0, x.1-1)})
                .collect();

        for i in active_squares {
            let pos: usize = (i.0 + i.1*self.width as i32) as usize;
            if pos > self.squares.len() {
                continue;
            }
            match self.squares[pos] {
                Square::None => { self.squares[pos] = Square::Ghost(self.active_piece.get_color())},
                _ => {panic!("Error positioning ghost pieces")},
            }
        }
    }

    fn set_active_piece(&mut self, new_active: Box<dyn Piece>) {
        let squares_to_set = self.active_piece.get_squares();
        debug_assert!(self.are_squares_free(&squares_to_set));
        for i in squares_to_set {
            self.set_square((i.0 as usize, i.1 as usize), Square::Normal(self.active_piece.get_color()));
        }

        self.active_piece = new_active;
    }

    fn find_lines_to_clear(&self) -> Vec<usize> {
        let mut lines_to_clear:  Vec<usize> = vec![];
        for y in 0..self.height {
            let mut is_line_full = true;
            for x in 0..self.width {
                match self.get_square((x,y)).unwrap() {
                    Square::Normal(_) => {},
                    _ => {
                        is_line_full = false; 
                        break;
                    },
                }
            }
            
            if is_line_full {
                lines_to_clear.push(y);
            }
        }

        lines_to_clear
    }

    fn clear_lines(&mut self) -> Vec<usize> {
        let mut lines_to_clear = self.find_lines_to_clear();
        if lines_to_clear.is_empty() {
            return lines_to_clear;
        }

        for y in lines_to_clear.iter() {
            for x in 0..self.width{
                self.set_square((x, *y), Square::None);
            }
        }

        lines_to_clear.sort();

        for l in &lines_to_clear {
            for y in (1..=*l).rev() {
                for x in 0..self.width {
                    self.set_square((x, y), self.get_square((x, y-1)).unwrap().clone())
                }
            }
        }

        for x in 0..self.width {
            self.set_square((x,0), Square::None);
        }

        debug_assert!(self.find_lines_to_clear().is_empty());

        lines_to_clear
    }

    fn is_active_piece_valid(&self) -> bool {
        self.are_squares_free(&self.active_piece.get_squares())
    }
}

pub enum GameEvent {
    LinesCleared(Vec<usize>),
    GameOver(i32),
    PieceSet((Vec<(i32, i32)>, Color))
}

pub struct Game {
    board: Board,
    score: i32,
    next: VecDeque<Box<dyn Piece>>,
    held: Option<Box<dyn Piece>>,
    has_swaped: bool,
    piece_generator: PieceGenerator,
}
impl Game {
    pub fn new(width: usize, height: usize, look_ahead: i32) -> Result<Game, String> {
        let mut piece_generator = PieceGenerator::new(((width/2) as i32, 0));
        let pieces = (0..look_ahead).map(|_| {piece_generator.generate_piece()}).collect();
        let board = Board::new(width, height, piece_generator.generate_piece());
        
        if board.is_err() {
            Err("Error constructing game".to_string())
        } else {
            let mut game = Game {board: board.unwrap(), score: 0, next: pieces, held: None, has_swaped: false, piece_generator};
            game.board.position_ghost_pieces();
            Ok(game)
        }
    }

    pub fn get_look_ahead(&self) -> Vec<&dyn Piece> {
        self.next.iter().map(|x| -> &dyn Piece {x.as_ref()}).collect()
    }

    pub fn next_step(&mut self) -> Vec<GameEvent> {
        let mut events: Vec<GameEvent> = vec![];

        if !self.board.is_active_piece_valid() {
            events.push(GameEvent::GameOver(self.score));
            return events;
        }

        if !self.board.move_active_down() {
            self.has_swaped = false;
            events.push(GameEvent::PieceSet(self.board.get_active_piece_squares()));
            self.board.set_active_piece(self.next.pop_back().unwrap());
            self.next.push_front(self.piece_generator.generate_piece());
            let cleared_lines = self.board.clear_lines();
            self.update_score(&cleared_lines);                     
            if !cleared_lines.is_empty() {
                events.push(GameEvent::LinesCleared(cleared_lines.clone()));
            }
            if !self.board.is_active_piece_valid() {
                events.push(GameEvent::GameOver(self.score));
                return events;
            }
        }

        self.board.position_ghost_pieces();        
        
        events 
    }

    pub fn get_held(&self) -> Option<(Vec<(i32, i32)>, Color)> {
        if let Some(piece) = &self.held {
            Some((piece.get_squares(), piece.get_color()))
        }else{
            None
        }
    }

    pub fn swap_held(&mut self) -> bool {
        if self.has_swaped {
            return false;
        }
        self.has_swaped = true;

        let start_position = (self.board.width as i32/2,0);
        if self.held.is_none() {
            self.held = Some(self.board.active_piece.get_type().create(start_position));
            self.board.active_piece = self.next.pop_back().unwrap();
            self.next.push_front(self.piece_generator.generate_piece());
        }else {
            let held_type = self.held.as_ref().unwrap().get_type();
            self.held = Some(self.board.active_piece.get_type().create(start_position));
            self.board.active_piece = held_type.create(start_position);
        }
        self.board.position_ghost_pieces();

        true
    }

    pub fn can_move_down(&mut self) -> bool {
        if self.board.move_active_down() {
            let result = self.board.move_active_up();
            debug_assert!(result);
            true
        }else{
            false
        }
    }

    pub fn set_piece_down(&mut self) {
        let mut squares_moved = 0;
        while self.board.move_active_down() {
            squares_moved += 1;
        }
        self.board.position_ghost_pieces();
        
        self.score += squares_moved*SCORE_REWARD_QUICK_PLACE;
    }

    pub fn move_left(&mut self) -> bool {
        let result = self.board.move_active_left();
        self.board.position_ghost_pieces();

        result
    }

    pub fn move_right(&mut self) -> bool {
        let result = self.board.move_active_right();
        self.board.position_ghost_pieces();

        result
    }

    pub fn move_up(&mut self) -> bool {
        let result = self.board.move_active_up();
        self.board.position_ghost_pieces();

        result
    }

    pub fn move_down(&mut self) -> bool {
        if !self.can_move_down() {
            return false;
        } 
        self.board.move_active_down();
        self.board.position_ghost_pieces();

        true
    }

    pub fn rotate_left(&mut self) -> bool {
        let result = self.board.rotate_active_left();
        if result {
            self.board.position_ghost_pieces();
        }

        result
    }

    pub fn rotate_right(&mut self) -> bool {
        let result = self.board.rotate_active_right();
        if result {
            self.board.position_ghost_pieces();
        }
        
        result
    }

    pub fn get_score(&self) -> i32 {
        self.score
    }

    pub fn get_board_squares(&self) -> Vec<Square> {
        let mut squares = self.board.squares.clone();

        let active = self.board.get_active_piece_squares();
        for i in active.0 {
            squares[(i.0 + i.1*self.board.width as i32) as usize] = Square::Normal(active.1); 
        }

        squares
    }

    fn update_score(&mut self, cleared_lines: &Vec<usize>) {
        self.score += match cleared_lines.len() {
            1 => {SCORE_REWARD_LINES1},
            2 => {SCORE_REWARD_LINES2},
            3 => {SCORE_REWARD_LINES3},
            4 => {SCORE_REWARD_LINES4},
            _ => {0},
        }
    }
}