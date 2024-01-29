mod model;
mod view;

use comfy::Color;
use comfy::*;
use comfy::EngineState;
use model::{GameListener, Square};

use crate::model::{Game};
use comfy::RED;
use comfy::simple_game;
use comfy::vec2;
use comfy::draw_circle;
use comfy::GameLoop;


const WIDTH: usize = 10;
const HEIGHT: usize = 20;
const LOOK_AHEAD: i32 = 3;

const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 800;

const SQUARE_SIZE: f32 = 1.3;
const GAME_BOARD_TOP_LEFT_POSITION: (f32, f32) = (-SQUARE_SIZE*(WIDTH as f32/2.0)-7.0, SQUARE_SIZE*(HEIGHT as f32/2.0)+0.0);

const BG_COLOR_R: f32 = 0.4;
const BG_COLOR_G: f32 = 0.4;
const BG_COLOR_B: f32 = 0.9;

const BOARD_Z: i32 = 0;
const SQUARES_Z: i32 = 1;

/* 
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
*/
/* 
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
*/


struct GameLoopImpl{
    game_state: Game,
}
impl GameLoopImpl {
    fn draw_game_board_bg(&self) {
        let center = vec2(
            GAME_BOARD_TOP_LEFT_POSITION.0 + (WIDTH/2) as f32 * SQUARE_SIZE - SQUARE_SIZE/2.0,
            GAME_BOARD_TOP_LEFT_POSITION.1 - (HEIGHT/2) as f32 * SQUARE_SIZE + SQUARE_SIZE/2.0
        );

        let size = vec2(
            WIDTH as f32 * SQUARE_SIZE,
            HEIGHT as f32 * SQUARE_SIZE 
        );

        draw_rect(center, size, BLACK, BOARD_Z);
    }

    fn draw_square(&self, position: (u32, u32), square: &Square) {
        if let Square::None = square {
            return;
        }
        
        let center = vec2(
            GAME_BOARD_TOP_LEFT_POSITION.0 + position.0 as f32 * SQUARE_SIZE, 
            GAME_BOARD_TOP_LEFT_POSITION.1 - position.1 as f32 * SQUARE_SIZE,
        );
        
        
        match square {
            Square::Normal(s) => {
                match s {
                    model::Color::Red => {draw_rect(center,  splat(SQUARE_SIZE), comfy::Color { r: 1.0, g: 0.5, b: 0.5, a: 1.0 }, SQUARES_Z)},
                    model::Color::Green => {draw_rect(center,  splat(SQUARE_SIZE), comfy::Color { r: 0.5, g: 1.0, b: 0.5, a: 1.0 }, SQUARES_Z)},
                    model::Color::Blue => {draw_rect(center,  splat(SQUARE_SIZE), comfy::Color { r: 0.5, g: 0.5, b: 1.0, a: 1.0 }, SQUARES_Z)},
                    model::Color::Yellow => {draw_rect(center,  splat(SQUARE_SIZE), comfy::Color { r: 1.0, g: 1.0, b: 0.5, a: 1.0 }, SQUARES_Z)},
                    _ => {},
                }
            },
            Square::Ghost(s) => {
                match s {
                    model::Color::Red => {draw_rect(center,  splat(SQUARE_SIZE), comfy::Color { r: 0.5, g: 0.1, b: 0.1, a: 1.0 }, SQUARES_Z)},
                    model::Color::Green => {draw_rect(center,  splat(SQUARE_SIZE), comfy::Color { r: 0.1, g: 0.5, b: 0.1, a: 1.0 }, SQUARES_Z)},
                    model::Color::Blue => {draw_rect(center,  splat(SQUARE_SIZE), comfy::Color { r: 0.1, g: 0.1, b: 0.5, a: 1.0 }, SQUARES_Z)},
                    model::Color::Yellow => {draw_rect(center,  splat(SQUARE_SIZE), comfy::Color { r: 0.5, g: 0.5, b: 0.1, a: 1.0 }, SQUARES_Z)},

                    _ => {},
                }
            },
            _ => {},
        }
        

        
    }
}
impl GameLoop for GameLoopImpl{
    fn new(c: &mut EngineState) -> Self {
        let mut game_state = Game::new(WIDTH, HEIGHT, LOOK_AHEAD)
            .expect("Error starting game");
        
        let mut game_loop = GameLoopImpl{game_state: game_state};     
        
        game_loop
    }



    fn update(&mut self, _c: &mut EngineContext) {
        clear_background(comfy::Color { r:BG_COLOR_R, g: BG_COLOR_G, b: BG_COLOR_B, a: 1.0 });
        self.draw_game_board_bg();
        self.draw_square((2,1), &Square::Normal(model::Color::Yellow));

        if is_key_pressed(KeyCode::Space) {
            //self.y += 1;
        }
    }
}
impl GameListener for GameLoopImpl {
    fn on_line_cleared(&self, lines_y: Vec<usize>) {
        println!("on_line_cleared: {}", lines_y.len());
    }
    fn on_game_over(&self){
        println!("on_game_over");
    }
    
    fn on_piece_set(&self){
        println!("on_piece_set");
    }
    
    fn on_score_changed(&self, score: i32){
        println!("on_score_changed: {score}");
    }
}

pub fn _comfy_default_config(config: GameConfig) -> GameConfig {
    let mut new_config = config.clone();
    new_config.resolution = ResolutionConfig::Logical(WINDOW_WIDTH, WINDOW_HEIGHT);
    new_config.min_resolution = ResolutionConfig::Logical(WINDOW_WIDTH, WINDOW_HEIGHT);

    new_config
}

pub async fn run() {
    init_game_config(
        "Tetris Game".to_string(),
        "v0.0.1",
        _comfy_default_config,
    );

    let mut engine = EngineState::new();

    let game = GameLoopImpl::new(&mut engine);

    run_comfy_main_async(game, engine).await;
}

fn main() {
    #[cfg(feature = "color-backtrace")]
    color_backtrace::install();

    #[cfg(not(target_arch = "wasm32"))]
    {
        pollster::block_on(run());
    }

    #[cfg(target_arch = "wasm32")]
    {
        wasm_bindgen_futures::spawn_local(run());
    }
}


