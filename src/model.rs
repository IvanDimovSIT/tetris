use self::pieces::SquarePiece;

mod pieces;

#[derive(Clone)]
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

pub trait Piece {
    fn get_position(&self) -> (i32, i32);
    fn set_position(&mut self, position: (i32, i32));
    fn get_color(&self) -> Color;
    fn get_squares(&self) -> Vec<(i32, i32)>;
    fn rotate_left(&mut self) -> bool;
    fn rotate_right(&mut self) -> bool;
}


pub trait GameListenerListener {
    fn on_line_cleared(&self, lines_y: Vec<usize>);
    fn on_game_over(&self);
    fn on_piece_set(&self);
    fn request_new_pice(&self) -> Box<dyn Piece>; 
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

        let mut board = Board{width: width, height: height, squares: vec![], active_piece: starting_piece};
        for _ in (0..(width*height)) {
            board.squares.push(Square::None);
        }

        Ok(board)
    }

    fn get_active_piece_squares(&self) -> (Vec<(i32, i32)>, Color) {
        (self.active_piece.get_squares(), self.active_piece.get_color())
    }

    fn get_width(&self) -> usize {
        self.width
    }

    fn get_height(&self) -> usize {
        self.height
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
        debug_assert!(self.is_active_piece_position_valid(&self.active_piece.get_squares()));
        let pos = self.active_piece.get_position();
        self.active_piece.set_position((pos.0-1, pos.1));
        
        if self.is_active_piece_position_valid(&self.active_piece.get_squares()) {
            true
        }else{
            self.active_piece.set_position(pos);
            false
        }
    }  

    fn move_active_right(&mut self) -> bool {
        debug_assert!(self.is_active_piece_position_valid(&self.active_piece.get_squares()));
        let pos = self.active_piece.get_position();
        self.active_piece.set_position((pos.0+1, pos.1));
        
        if self.is_active_piece_position_valid(&self.active_piece.get_squares()) {
            true
        }else{
            self.active_piece.set_position(pos);
            false
        }
    }  

    fn move_active_down(&mut self) -> bool {
        debug_assert!(self.is_active_piece_position_valid(&self.active_piece.get_squares()));
        let pos = self.active_piece.get_position();
        self.active_piece.set_position((pos.0, pos.1+1));
        
        if self.is_active_piece_position_valid(&self.active_piece.get_squares()) {
            true
        }else{
            self.active_piece.set_position(pos);
            false
        }
    }

    fn rotate_active_left(&mut self) -> bool {
        debug_assert!(self.is_active_piece_position_valid(&self.active_piece.get_squares()));
        if !self.active_piece.rotate_left() {
            return false;
        }

        if self.is_active_piece_position_valid(&self.active_piece.get_squares()) {
            true
        }else{
            self.active_piece.rotate_right();
            false
        }
    }

    fn rotate_active_right(&mut self) -> bool {
        debug_assert!(self.is_active_piece_position_valid(&self.active_piece.get_squares()));
        if !self.active_piece.rotate_right() {
            return false;
        }

        if self.is_active_piece_position_valid(&self.active_piece.get_squares()) {
            true
        }else{
            self.active_piece.rotate_left();
            false
        }
    }

    fn is_active_piece_position_valid(&self, active_squares: &Vec<(i32, i32)>) -> bool {
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

        while self.is_active_piece_position_valid(&active_squares) {
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
            match self.squares[pos] {
                Square::None => { self.squares[pos] = Square::Ghost(self.active_piece.get_color())},
                _ => {panic!("Error positioning ghost pieces")},
            }
        }
    }

    fn set_active_piece(&mut self, new_active: Box<dyn Piece>) {
        let squares_to_set = self.active_piece.get_squares();
        debug_assert!(self.is_active_piece_position_valid(&squares_to_set));
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

    fn clear_line(&mut self, row: usize) {
        for x in 0..self.width {
            self.set_square((x, row), Square::None);
        }

        for y in (1..row).rev() {
            for x in 0..self.width {
                let square_to_add = self.get_square((x, y-1));
                debug_assert!(square_to_add.is_some());
                self.set_square((x, y), square_to_add.unwrap().clone());
            }
        }

        for x in 0..self.width {
            self.set_square((x, 0), Square::None);
        }
    }

    fn clear_lines(&mut self) -> Vec<usize> {
        let lines_to_clear = self.find_lines_to_clear();
        if lines_to_clear.is_empty() {
            return  lines_to_clear;
        }

        for i in &lines_to_clear {
            self.clear_line(*i);
        }

        lines_to_clear
    }
}
