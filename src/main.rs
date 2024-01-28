use model::{GameListener, Square};

use crate::model::{Color, Game};

mod model;

const WIDTH: usize = 8;
const HEIGHT: usize = 12;

struct Listener{
}
impl GameListener for Listener {
    fn on_line_cleared(&self, lines_y: Vec<usize>) {
        println!("on_line_cleared");
    }
    fn on_game_over(&self){
        println!("on_game_over");
    }
    
    fn on_piece_set(&self){
        println!("on_piece_set");
    }
    
    fn on_score_changed(&self, score: i32){
        println!("on_score_changed");
    }
}

fn print_game(squares: Vec<Square>) {
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let c;
            match squares[x + y*WIDTH] {
                Square::Normal(C) => {
                    match C {
                        Color::Blue => {c = 'B'},
                        Color::Yellow => {c = 'Y'},
                        Color::Green => {c = 'G'},
                        Color::Red => {c = 'R'},
                        _ => {c = '_'},
                    }
                },
                Square::Ghost(C) => {
                    match C {
                        Color::Blue => {c = 'b'},
                        Color::Yellow => {c = 'y'},
                        Color::Green => {c = 'g'},
                        Color::Red => {c = 'r'},
                        _ => {c = '_'},
                    }
                }
                Square::None => {
                    c = '.';
                },
                _ => {c = 'X'},
            }
            print!("{c} ");
        }
        println!();
    }
    println!();
}


fn main() {
    println!("Hello, world!");

    let mut game = Game::new(WIDTH, HEIGHT, 3, Box::new(Listener{})).unwrap();
    print_game(game.get_board_squares());

    game.next_step();
    print_game(game.get_board_squares());

    game.move_left();
    print_game(game.get_board_squares());

    for _ in 0..15 {
        game.next_step();
        print_game(game.get_board_squares());
    }

}
