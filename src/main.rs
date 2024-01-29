mod model;
mod view;

use comfy::wgpu_types::Color;
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
const HEIGHT: usize = 40;
const LOOK_AHEAD: i32 = 3;

const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 800;

const SQUARE_SIZE: u32 = 12;
const GAME_BOARD_TOP_RIGHT_POSITION: (u32, u32) = (20, 20);

const BG_COLOR_R: f32 = 0.4;
const BG_COLOR_G: f32 = 0.4;
const BG_COLOR_B: f32 = 0.9;

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
        draw_rect(center, size, color, z_index)
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
        clear_background(comfy::Color { r:BG_COLOR_R, g: BG_COLOR_G, b: BG_COLOR_B, a: 0.0 });
        
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


