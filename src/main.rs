mod model;
mod constants;
mod view;

use comfy::epaint::FontId;

use comfy::*;
use model::Square;
use view::load_sounds;
use view::put_square;
use view::BGColorSelector;
use view::Effect;

use crate::model::Game;
use crate::constants::*;
use crate::model::GameEvent;
use crate::view::ClearEffect;
use crate::view::PlaceEffect;

use comfy::vec2;

use comfy::GameLoop;

struct GameStats{
    time: f32,
    swaps: i32,
    line_clears_1: i32,
    line_clears_2: i32,
    line_clears_3: i32,
    line_clears_4: i32,
}
impl GameStats{
    fn new() -> GameStats{
        GameStats{
            time: 0.0,
            swaps: 0,
            line_clears_1: 0,
            line_clears_2: 0,
            line_clears_3: 0,
            line_clears_4: 0
        }
    }
}

struct GameLoopImpl{
    game_state: Game,
    time_passed: f32,
    difficulty: f32,
    is_game_over: bool,
    score: i32,
    effects: Vec<Box<dyn Effect>>,
    bg_color: BGColorSelector,
    game_stats: GameStats,
    is_paused: bool,
    are_textures_loaded: bool,
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
        
        put_square(center, square, SQUARE_SIZE);
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
                put_square(center, &Square::Normal(i.1.get_color()), LOOK_AHEAD_SQUARE_SIZE);
            }
        }
    }

    fn draw_held(&self) {
        draw_rect(
            vec2(HELD_PIECE_POSITION.0, HELD_PIECE_POSITION.1),
            splat(HELD_PIECE_BG_SIZE),
            BLACK,
            BOARD_Z
        );

        let held = self.game_state.get_held();
        if held.is_none() {
            draw_text_ex(
                HELD_PIECE_EMPTY_TEXT,
                vec2(HELD_PIECE_POSITION.0, HELD_PIECE_POSITION.1),
                TextAlign::Center,
                TextParams { 
                    font:FontId{family: epaint::FontFamily::Proportional, size: HELD_PIECE_EMPTY_TEXT_SIZE},
                    rotation: 0.0,
                    color: WHITE 
                }
            );
            return;
        }

        let (squares, color) = held.unwrap();
        for i in squares {
            let center = vec2(
                HELD_PIECE_POSITION.0 + i.0 as f32 * HELD_PIECE_SQUARE_SIZE - HELD_PIECE_BG_SIZE/1.2,
                HELD_PIECE_POSITION.1 - i.1 as f32 * HELD_PIECE_SQUARE_SIZE + HELD_PIECE_BG_SIZE/5.0
            );
            put_square(center, &Square::Normal(color), HELD_PIECE_SQUARE_SIZE);
        }
    }

    fn redraw(&mut self, delta: f32) {
        clear_background(self.bg_color.get_color());
        self.draw_game_board_bg();
        let squares = self.game_state.get_board_squares();
        for (ind, s) in squares.iter().enumerate() {
            self.draw_square(((ind%WIDTH) as u32, (ind/WIDTH) as u32), s);
        }
        self.draw_score();
        self.draw_look_ahead();
        self.draw_held();
        self.draw_effects(delta);
    }

    fn on_lines_cleared(&mut self, lines: Vec<usize>) {
        if lines.is_empty() {
            return;
        }
        
        match lines.len() {
            1 => self.game_stats.line_clears_1 += 1,
            2 => self.game_stats.line_clears_2 += 1,
            3 => self.game_stats.line_clears_3 += 1,
            4 => self.game_stats.line_clears_4 += 1,
            _ => {
                dbg!(&lines);
                panic!("Invalid lines cleared {}", lines.len())
            },
        }

        self.bg_color.on_cleared();
        play_sound(CLEAR_SOUND_TAG);

        println!("cleared lines:{}", lines.len());
        self.difficulty += DIFFICULTY_INCREASE;
        if self.difficulty > MAX_DIFFICULTY {
            self.difficulty = MAX_DIFFICULTY;
        }
        self.score = self.game_state.get_score();

        for i in lines {
            let center = vec2(
                GAME_BOARD_TOP_LEFT_POSITION.0 + WIDTH as f32 * SQUARE_SIZE / 2.0 - SQUARE_SIZE/2.0,
                GAME_BOARD_TOP_LEFT_POSITION.1 - i as f32 * SQUARE_SIZE);

            self.effects.push(
                Box::new(
                    ClearEffect::new(
                        center,
                        vec2(SQUARE_SIZE*WIDTH as f32, SQUARE_SIZE),
                        CLEAR_EFFECT_DELAY
                    )
                )
            );
        }
    }

    fn handle_rotate_left(&mut self){
        let result = self.game_state.rotate_left();
        if result {
            play_sound(ROTATE_SOUND_TAG);
            return;
        }

        
        if self.game_state.move_left() {
            if self.game_state.rotate_left() {
                play_sound(ROTATE_SOUND_TAG);
                return;
            }else{
                assert!(self.game_state.move_right());
            }
        }
        if self.game_state.move_right() {
            if self.game_state.rotate_left() {
                play_sound(ROTATE_SOUND_TAG);
                return;
            }else{
                assert!(self.game_state.move_left());
            }
        }

        if self.game_state.move_up() {
            if self.game_state.rotate_left() {
                play_sound(ROTATE_SOUND_TAG);
                return;
            }else{
                assert!(self.game_state.move_down());
            }
        }
    }

    fn handle_rotate_right(&mut self){
        let result = self.game_state.rotate_right();
        if result {
            play_sound(ROTATE_SOUND_TAG);
            return;
        }

        if self.game_state.move_left() {
            if self.game_state.rotate_right() {
                play_sound(ROTATE_SOUND_TAG);
                return;
            }else{
                assert!(self.game_state.move_right());
            }
        }
        if self.game_state.move_right() {
            if self.game_state.rotate_right() {
                play_sound(ROTATE_SOUND_TAG);
                return;
            }else{
                assert!(self.game_state.move_left());
            }
        }

        if self.game_state.move_up() {
            if self.game_state.rotate_right() {
                play_sound(ROTATE_SOUND_TAG);
                return;
            }else{
                assert!(self.game_state.move_down());
            }
        }
    }

    fn handle_swap_piece(&mut self) {
        let result = self.game_state.swap_held();
        if result {
            play_sound(SWAP_SOUND_TAG);
            self.game_stats.swaps += 1;
            self.time_passed = 0.0;
        }else{
            play_sound(CANTSWAP_SOUND_TAG);
        }
    }

    fn handle_move_left(&mut self) {
        if self.game_state.move_left() {
            play_sound(MOVE_SOUND_TAG);
        }
    }

    fn handle_move_right(&mut self) {
        if self.game_state.move_right() {
            play_sound(MOVE_SOUND_TAG);
        }
    }

    fn handle_set_piece(&mut self){
        self.game_state.set_piece_down();
        self.score = self.game_state.get_score();
        let events = self.game_state.next_step();
        self.time_passed = 0.0;
        self.handle_game_events(events);
    }

    fn draw_pause_screen(&self) {
        clear_background(self.bg_color.get_color());
        draw_text_ex(
            PAUSE_TEXT,
            vec2(PAUSE_TEXT_POSITION.0,PAUSE_TEXT_POSITION.1),
            TextAlign::Center,
            TextParams{
                font: FontId { 
                    size: PAUSE_TEXT_SIZE,
                    family: epaint::FontFamily::Proportional,
                },
                rotation: 0.0,
                color: WHITE 
            }
        );
    }

    fn restart(&mut self) {
        self.game_state = Game::new(WIDTH, HEIGHT, LOOK_AHEAD).expect("Error reloading game");
        self.time_passed = 1.0;
        self.difficulty = START_DIFFICULTY;
        self.is_game_over = false;
        self.score = 0;
        self.effects = vec![];
        self.game_stats = GameStats::new();
        self.bg_color = BGColorSelector::new(CLEARS_TO_CHANGE_BG);
        self.is_paused = false;
        self.are_textures_loaded = false;

        play_sound(MUSIC_SOUND_TAG);    
    }

    fn draw_game_over(&mut self) {
        if is_key_pressed(KeyCode::R){
            self.restart();
            return;
        }

        clear_background(self.bg_color.get_color());
        draw_text_ex(
            GAME_OVER_TEXT,
            vec2(GAME_OVER_POSITION.0,GAME_OVER_POSITION.1),
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

        draw_text_ex(
            RESTART_TEXT,
            vec2(RESTART_TEXT_POSITION.0,RESTART_TEXT_POSITION.1),
            TextAlign::Center,
            TextParams{
                font: FontId { 
                    size: RESTART_TEXT_SIZE,
                    family: epaint::FontFamily::Proportional,
                },
                rotation: 0.0,
                color: WHITE 
            }
        );

        egui::Window::new("Results")
        .anchor(egui::Align2::CENTER_BOTTOM, egui::vec2(0.0, -100.0))
        .default_width(RESULTS_SIZE.0)
        .default_height(RESULTS_SIZE.1)
        .fixed_size(egui::vec2(RESULTS_SIZE.0, RESULTS_SIZE.1))
        .collapsible(false)
        .show(egui(), |ui| {
            ui.label(egui::RichText::new(format!("Score: {}", self.score))
                .font(FontId::proportional(RESULTS_TEXT_SIZE))
            );

            ui.label(egui::RichText::new(format!("Time: {:?}:{:0>2}",
                self.game_stats.time as i32 /60,
                self.game_stats.time as i32 %60)).font(FontId::proportional(RESULTS_TEXT_SIZE))
            );

            ui.label(egui::RichText::new(format!("Swaps: {}", self.game_stats.swaps))
                .font(FontId::proportional(RESULTS_TEXT_SIZE))
            );

            ui.label(egui::RichText::new(format!("Single line clears: {}", self.game_stats.line_clears_1))
                .font(FontId::proportional(RESULTS_TEXT_SIZE))
            );

            ui.label(egui::RichText::new(format!("Double line clears: {}", self.game_stats.line_clears_2))
                .font(FontId::proportional(RESULTS_TEXT_SIZE))
            );

            ui.label(egui::RichText::new(format!("Tripple line clears: {}", self.game_stats.line_clears_3))
                .font(FontId::proportional(RESULTS_TEXT_SIZE))
            );

            ui.label(egui::RichText::new(format!("Quadruple line clears: {}", self.game_stats.line_clears_4))
                .font(FontId::proportional(RESULTS_TEXT_SIZE))
            );

            ui.label(egui::RichText::new(format!("Total lines cleared: {}",
                self.game_stats.line_clears_4*4 +
                self.game_stats.line_clears_3*3 +
                self.game_stats.line_clears_2*2 +
                self.game_stats.line_clears_1))
                .font(FontId::proportional(RESULTS_TEXT_SIZE))
            );
        });
    }

    fn handle_input(&mut self) {
        if is_key_pressed(KeyCode::Space) {
            self.handle_set_piece();
        }

        if is_key_pressed(KeyCode::Left) || is_key_pressed(KeyCode::A) {
            self.handle_move_left();
        } 

        if is_key_pressed(KeyCode::Right) || is_key_pressed(KeyCode::D) {
            self.handle_move_right();
        }

        if is_key_pressed(KeyCode::Z) {
            self.handle_rotate_left();
        }

        if is_key_pressed(KeyCode::X) || is_key_pressed(KeyCode::W) || is_key_pressed(KeyCode::Up){
            self.handle_rotate_right();
        }

        if is_key_pressed(KeyCode::E) {
            self.handle_swap_piece();
        }

        if is_key_down(KeyCode::Down) || is_key_down(KeyCode::S) {
            self.time_passed += self.get_step_delay()*PUSH_DOWN_SPEED;
        }
    }

    fn draw_effects(&mut self, delta: f32) {
        for i in &mut self.effects {
            i.draw(delta);
        }

        self.effects.retain(|x| !x.is_complete());
    }

    fn handle_game_events(&mut self, events: Vec<GameEvent>){
        for i in events {
            match i {
                GameEvent::GameOver(score) => {
                    println!("Game over! score:{score}");
                    play_sound(GAMEOVER_SOUND_TAG);
                    self.is_game_over = true;
                },
                GameEvent::PieceSet((squares, _color)) => {
                    println!("Piece set");
                    self.effects.push(Box::new(PlaceEffect::new(squares)));
                    play_sound(PLACE_SOUND_TAG);
                },
                GameEvent::LinesCleared(lines) => {self.on_lines_cleared(lines)},
            }
        }
    }

    fn get_step_delay(&mut self) -> f32 {
        let mut step_delay = if self.game_state.can_move_down() {
            BASE_PIECE_FALL_SPEED/self.difficulty
        }else{
            (BASE_PIECE_FALL_SPEED/self.difficulty)*PLACE_PIECE_DELAY_MULTIPLIER
        };
           

        if step_delay > PLACE_PIECE_DELAY_MAX {
            step_delay = PLACE_PIECE_DELAY_MAX;
        }

        step_delay
    }

}
impl GameLoop for GameLoopImpl{
    fn new(_c: &mut EngineState) -> Self {
        let game_state = Game::new(WIDTH, HEIGHT, LOOK_AHEAD)
            .expect("Error starting game");

        GameLoopImpl{
            game_state,
            time_passed: 1.0,
            difficulty: START_DIFFICULTY,
            is_game_over: false,
            score: 0,
            effects: vec![],
            game_stats: GameStats::new(),
            bg_color: BGColorSelector::new(CLEARS_TO_CHANGE_BG),
            is_paused: false,
            are_textures_loaded: false,
        }  
    }

    fn update(&mut self, c: &mut EngineContext) {
        if !self.are_textures_loaded {
            c.load_texture_from_bytes(
                PARTICLE_TEXTURE_TAG,
                include_bytes!(concat!(
                    env!("CARGO_MANIFEST_DIR"),
                    "/assets/particle.png"
                )),
            );
        }

        if self.is_game_over {
            stop_sound(MUSIC_SOUND_TAG);
            self.draw_game_over();
            return;
        }

        if is_key_pressed(KeyCode::Escape){
            if self.is_paused {
                play_sound(MUSIC_SOUND_TAG);    
            }
            self.is_paused = ! self.is_paused;
        }
        
        if self.is_paused {
            stop_sound(MUSIC_SOUND_TAG);
            self.draw_pause_screen();
            return;
        }

        self.game_stats.time += c.delta;        
        self.time_passed += c.delta;
        
        self.handle_input();
    
        let mut step_delay = self.get_step_delay();

        let mut game_events: Vec<GameEvent> = vec![];        
        while self.time_passed >= step_delay {
            self.time_passed -= step_delay;
            let received = self.game_state.next_step();
            for i in received {
                game_events.push(i);
            }
            if !self.game_state.can_move_down() {
                self.time_passed = 0.0;
            }
            let new_step_delay = self.get_step_delay();
            if new_step_delay != step_delay {
                step_delay = new_step_delay;
                self.time_passed = 0.0;
            }
        }
        self.redraw(c.delta);
        self.handle_game_events(game_events);
    }
}


pub fn _comfy_default_config(config: GameConfig) -> GameConfig {
    let mut new_config = config.clone();
    new_config.resolution = ResolutionConfig::Logical(WINDOW_WIDTH, WINDOW_HEIGHT);
    new_config.min_resolution = ResolutionConfig::Logical(MIN_WINDOW_WIDTH, MIN_WINDOW_HEIGHT);
    new_config.bloom_enabled = true;
    new_config.tonemapping_enabled = true;
    new_config.lighting.bloom_lerp = LIGHT_EFFECT;

    new_config
}

pub async fn run() {
    init_game_config(
        "Tetris Game".to_string(),
        "v0.1.0",
        _comfy_default_config,
    );

    let mut engine = EngineState::new();

    load_sounds();

    let game = GameLoopImpl::new(&mut engine);
    play_sound(MUSIC_SOUND_TAG);
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


