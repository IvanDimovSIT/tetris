mod model;
mod constants;


use comfy::epaint::FontId;

use comfy::*;
use comfy::EngineState;
use model::Square;

use crate::model::Game;
use crate::constants::*;
use crate::model::GameEvent;

use comfy::vec2;

use comfy::GameLoop;

struct GameLoopImpl{
    game_state: Game,
    time_passed: f32,
    difficulty: f32,
    is_game_over: bool,
    score: i32,
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

    fn put_square(&self, center: Vec2, square: &Square, square_size: f32) {
        let mut color: comfy::Color = match square {
            Square::Normal(s) | Square::Ghost(s) => {
                match s {
                    model::Color::Red => { comfy::Color { r: SQUARE_RED_R, g: SQUARE_RED_G, b: SQUARE_RED_B, a: 1.0 } },
                    model::Color::Green => { comfy::Color { r: SQUARE_GREEN_R, g: SQUARE_GREEN_G, b: SQUARE_GREEN_B, a: 1.0 } },
                    model::Color::Blue => { comfy::Color { r: SQUARE_BLUE_R, g: SQUARE_BLUE_G, b: SQUARE_BLUE_B, a: 1.0 } },
                    model::Color::Yellow => { comfy::Color { r: SQUARE_YELLOW_R, g: SQUARE_YELLOW_G, b: SQUARE_YELLOW_B, a: 1.0 } },
                }
            },
            Square::None => panic!("Unrecognised square"),
        };
        
        if let Square::Ghost(_) = square {
            color.r *= SQUARE_GHOST_COLOR_COEF;
            color.g *= SQUARE_GHOST_COLOR_COEF;
            color.b *= SQUARE_GHOST_COLOR_COEF;
        }
        draw_rect(center,  splat(square_size), color, SQUARES_Z);

        color.r *= SQUARE_INNER_COLOR_COEF;
        color.g *= SQUARE_INNER_COLOR_COEF;
        color.b *= SQUARE_INNER_COLOR_COEF;

        draw_rect(center,  splat(SQUARE_SIZE_INNER_COEF * square_size), color, SQUARES_Z + 1);
    }

    fn draw_square(&self, position: (u32, u32), square: &Square) {
        if let Square::None = square {
            return;
        }
        
        let center = vec2(
            GAME_BOARD_TOP_LEFT_POSITION.0 + position.0 as f32 * SQUARE_SIZE, 
            GAME_BOARD_TOP_LEFT_POSITION.1 - position.1 as f32 * SQUARE_SIZE,
        );
        
        self.put_square(center, square, SQUARE_SIZE);
    }

    fn draw_score(&self) {
        let score_text = format!("{:0>width$}", self.score, width = SCORE_DIGITS);
        draw_rect(vec2(SCORE_LOCATION.0, SCORE_LOCATION.1), vec2(SCORE_WIDTH, SCORE_HEIGHT), BLACK, SCORE_BG_Z);
        draw_text_ex(
            &score_text,
            vec2(SCORE_LOCATION.0, SCORE_LOCATION.1),
            TextAlign::Center,
            TextParams { 
                font:FontId{family: epaint::FontFamily::Monospace, size: SCORE_FONT_SIZE},
                rotation: 0.0,
                color: WHITE 
            }
        );
    }

    fn draw_look_ahead(&self) {
        draw_rect(
            vec2(LOOK_AHEAD_POSITION.0, LOOK_AHEAD_POSITION.1),
            vec2(LOOK_AHEAD_WIDTH, LOOK_AHEAD_HEIGHT),
            BLACK,
            BOARD_Z
        );

        let next_pieces = self.game_state.get_look_ahead();
        for i in next_pieces.iter().rev().enumerate() {
            for sq in i.1.get_squares() {
                let center = vec2(
                    LOOK_AHEAD_START_DRAW.0+sq.0 as f32*LOOK_AHEAD_SQUARE_SIZE,
                    LOOK_AHEAD_START_DRAW.1+sq.1 as f32*(-LOOK_AHEAD_SQUARE_SIZE) - i.0 as f32 * LOOK_AHEAD_NEXT_PIECE_OFFSET
                );
                self.put_square(center, &Square::Normal(i.1.get_color()), LOOK_AHEAD_SQUARE_SIZE);
            }
        }
    }

    fn redraw(&self) {
        clear_background(comfy::Color { r:BG_COLOR_R, g: BG_COLOR_G, b: BG_COLOR_B, a: 1.0 });
        self.draw_game_board_bg();
        let squares = self.game_state.get_board_squares();
        for (ind, s) in squares.iter().enumerate() {
            self.draw_square(((ind%WIDTH) as u32, (ind/WIDTH) as u32), s);
        }
        self.draw_score();
        self.draw_look_ahead();
    }

    fn on_lines_cleared(&mut self, lines: Vec<usize>) {
        if lines.is_empty() {
            return;
        }

        println!("cleared lines:{}", lines.len());
        self.difficulty += DIFFICULTY_INCREASE;
        self.score = self.game_state.get_score();
    }

    fn handle_rotate_left(&mut self){
        let result = self.game_state.rotate_left();
        if result {
            return;
        }
        if self.game_state.move_left() {
            if self.game_state.rotate_left() {
                return;
            }else{
                assert!(self.game_state.move_right());
            }
        }
        if self.game_state.move_right() {
            if self.game_state.rotate_left() {
                return;
            }else{
                assert!(self.game_state.move_left());
            }
        }
    }

    fn handle_rotate_right(&mut self){
        let result = self.game_state.rotate_right();
        if result {
            return;
        }
        if self.game_state.move_left() {
            if self.game_state.rotate_right() {
                return;
            }else{
                assert!(self.game_state.move_right());
            }
        }
        if self.game_state.move_right() {
            if self.game_state.rotate_right() {
                return;
            }else{
                assert!(self.game_state.move_left());
            }
        }
    }

    fn handle_set_piece(&mut self){
        self.game_state.set_piece_down();
        let events = self.game_state.next_step();
        self.time_passed = 0.0;
        self.handle_game_events(events);
    }

    fn draw_game_over(&self) {
        clear_background(comfy::Color { r:BG_COLOR_R, g: BG_COLOR_G, b: BG_COLOR_B, a: 1.0 });
        draw_text_ex(
            format!("Game Over! Score:{}", self.score).as_str(),
            splat(0.0),
            TextAlign::Center,
            TextParams{
                font: FontId { 
                    size: GAME_OVER_TEXT_SIZE,
                    family: epaint::FontFamily::Proportional,
                },
                rotation: 0.0,
                color: WHITE 
            }
        );
    }

    fn handle_game_events(&mut self, events: Vec<GameEvent>){
        for i in events {
            match i {
                GameEvent::GameOver(score) => {
                    println!("Game over! score:{score}");
                    self.is_game_over = true;
                },
                GameEvent::PieceSet => {
                    println!("Piece set")
                },
                GameEvent::LinesCleared(lines) => {self.on_lines_cleared(lines)},
                _ => {},
            }
        }
    }

}
impl GameLoop for GameLoopImpl{
    fn new(_c: &mut EngineState) -> Self {
        let game_state = Game::new(WIDTH, HEIGHT, LOOK_AHEAD)
            .expect("Error starting game");
        
        let game_loop = GameLoopImpl{game_state: game_state, time_passed: 1.0, difficulty: START_DIFFICULTY, is_game_over: false, score: 0};     
        
        game_loop
    }

    fn update(&mut self, _c: &mut EngineContext) {
        if self.is_game_over {
            self.draw_game_over();
            return;
        }

        self.time_passed += _c.delta;
        dbg!(self.time_passed);
        let mut game_events: Vec<GameEvent> = vec![];
        
        if is_key_pressed(KeyCode::Space) {
            self.handle_set_piece();
        }

        if is_key_pressed(KeyCode::Left) || is_key_pressed(KeyCode::A) {
            self.game_state.move_left();
        } 

        if is_key_pressed(KeyCode::Right) || is_key_pressed(KeyCode::D) {
            self.game_state.move_right();
        }

        if is_key_pressed(KeyCode::Q) || is_key_pressed(KeyCode::Z) {
            self.handle_rotate_left();
        }

        if is_key_pressed(KeyCode::E) || is_key_pressed(KeyCode::X) ||
            is_key_pressed(KeyCode::W) || is_key_pressed(KeyCode::Up){
            self.handle_rotate_right();
        }

        if is_key_down(KeyCode::Down) || is_key_down(KeyCode::S) {
            let received = self.game_state.next_step();
            for i in received {
                game_events.push(i);
            }
        }

        let mut step_delay = if self.game_state.can_move_down() {
            1.0/self.difficulty
        }else{
            (1.0/self.difficulty)*PLACE_PIECE_DELAY_MULTIPLIER
        };

        if step_delay > PLACE_PIECE_DELAY_MAX {
            step_delay = PLACE_PIECE_DELAY_MAX;
        }

        if self.time_passed >= step_delay {
            self.time_passed = 0.0;
            let received = self.game_state.next_step();
            for i in received {
                game_events.push(i);
            }
        }
        self.redraw();
        self.handle_game_events(game_events);
    }
}


pub fn _comfy_default_config(config: GameConfig) -> GameConfig {
    let mut new_config = config.clone();
    new_config.resolution = ResolutionConfig::Logical(WINDOW_WIDTH, WINDOW_HEIGHT);
    new_config.min_resolution = ResolutionConfig::Logical(MIN_WINDOW_WIDTH, MIN_WINDOW_HEIGHT);

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


