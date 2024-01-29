mod model;
mod constants;

use comfy::Color;
use comfy::*;
use comfy::EngineState;
use model::{GameListener, Square};

use crate::model::{Game};
use crate::constants::*;

use comfy::RED;
use comfy::simple_game;
use comfy::vec2;
use comfy::draw_circle;
use comfy::GameLoop;

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
        
        
        let mut color: comfy::Color = match square {
            Square::Normal(s) | Square::Ghost(s) => {
                match s {
                    model::Color::Red => { comfy::Color { r: SQUARE_RED_R, g: SQUARE_RED_G, b: SQUARE_RED_B, a: 1.0 } },
                    model::Color::Green => { comfy::Color { r: SQUARE_RED_R, g: SQUARE_RED_G, b: SQUARE_RED_B, a: 1.0 } },
                    model::Color::Blue => { comfy::Color { r: SQUARE_RED_R, g: SQUARE_RED_G, b: SQUARE_RED_B, a: 1.0 } },
                    model::Color::Yellow => { comfy::Color { r: SQUARE_RED_R, g: SQUARE_RED_G, b: SQUARE_RED_B, a: 1.0 } },
                }
            },
            Square::None => panic!("Unrecognised square"),
        };
        
        if let Square::Ghost(_) = square {
            color.r *= SQUARE_GHOST_COLOR_COEF;
            color.g *= SQUARE_GHOST_COLOR_COEF;
            color.b *= SQUARE_GHOST_COLOR_COEF;
        }
        draw_rect(center,  splat(SQUARE_SIZE), color, SQUARES_Z);

        color.r *= SQUARE_INNER_COLOR_COEF;
        color.g *= SQUARE_INNER_COLOR_COEF;
        color.b *= SQUARE_INNER_COLOR_COEF;

        draw_rect(center,  splat(SQUARE_SIZE_INNER), color, SQUARES_Z + 1);

        //draw_rect(center,  splat(SQUARE_SIZE), comfy::Color { r: SQUARE_RED_R, g: SQUARE_RED_G, b: SQUARE_RED_B, a: 1.0 }, SQUARES_Z)

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


