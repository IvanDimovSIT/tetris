mod pieces;

enum Color {
    Red,
    Green,
    Yellow,
    Blue,
}

enum Square {
    None,
    Normal(Color),
    Ghost(Color),
}

trait Piece {
    fn get_position(&self) -> (usize, usize);
    fn set_position(&mut self, position: (usize, usize));
    fn get_color(&self) -> Color;
    fn get_squares(&self) -> Vec<(usize, usize)>;
    fn rotate_left(&mut self) -> bool;
    fn rotate_right(&mut self) -> bool;
}

struct Board {
    squares: Vec<Square>,
    width: usize,
    height: usize,
    active_piece: Box<dyn Piece>,

}
