use rand::seq::SliceRandom;
use rand::rngs::ThreadRng;

use super::*;
use super::PieceType;

pub struct PieceGenerator{
    rng: ThreadRng,
    bag: Vec<PieceType>,
    start_position: (i32, i32),
}
impl PieceGenerator {
    pub fn new(start_position: (i32, i32)) -> PieceGenerator {
        PieceGenerator{ rng: rand::thread_rng(), bag: vec![], start_position }
    }

    pub fn generate_piece(&mut self) -> Box<dyn Piece> {
        if self.bag.is_empty() {
            self.refill_bag();
        }
        
        self.bag.pop().unwrap().create(self.start_position)
    }

    fn refill_bag(&mut self) {
        let mut bag: Vec<PieceType> = vec![
            PieceType::LPiece,
            PieceType::SPiece,
            PieceType::ZPiece,
            PieceType::JPiece,
            PieceType::TPiece,
            PieceType::LinePiece,
            PieceType::SquarePiece
        ];
        bag.shuffle(&mut self.rng);
        self.bag = bag;
    }
}
