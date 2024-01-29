use rand::seq::SliceRandom;
use rand::rngs::ThreadRng;

use super::*;

pub struct PieceGenerator{
    rng: ThreadRng,
    bag: Vec<i32>,
    start_position: (i32, i32),
}
impl PieceGenerator {
    pub fn new(start_position: (i32, i32)) -> PieceGenerator {
        PieceGenerator{ rng: rand::thread_rng(), bag: vec![], start_position }
    }

    pub fn generate_piece(&mut self) -> Box<dyn Piece> {
        let piece: Box<dyn Piece>;
        if self.bag.is_empty() {
            self.refill_bag();
        }

        match self.bag.pop().unwrap() {
            0 => {piece = Box::new(SquarePiece::new(self.start_position))},
            1 => {piece = Box::new(LPiece::new(self.start_position))},
            2 => {piece = Box::new(SPiece::new(self.start_position))},
            3 => {piece = Box::new(JPiece::new(self.start_position))},
            4 => {piece = Box::new(ZPiece::new(self.start_position))},
            5 => {piece = Box::new(LinePiece::new(self.start_position))},
            _ => {panic!("Invalid random number")},
        };

        piece
    }

    fn refill_bag(&mut self) {
        let mut bag: Vec<i32> = (0..=5).collect();
        bag.shuffle(&mut self.rng);
        self.bag = bag;
    }
}