pub const WIDTH: usize = 10;
pub const HEIGHT: usize = 20;
pub const LOOK_AHEAD: i32 = 3;

pub const WINDOW_WIDTH: u32 = 800;
pub const WINDOW_HEIGHT: u32 = 800;

pub const MIN_WINDOW_WIDTH: u32 = 750;
pub const MIN_WINDOW_HEIGHT: u32 = 700;

pub const SQUARE_SIZE: f32 = 1.35;
pub const SQUARE_SIZE_INNER_COEF: f32 = 0.7;

pub const GAME_BOARD_TOP_LEFT_POSITION: (f32, f32) = (-13.8, 13.8);

pub const SCORE_LOCATION: (f32, f32) = (7.5, -12.0);
pub const SCORE_WIDTH: f32 = 13.0;
pub const SCORE_HEIGHT: f32 = 3.0;
pub const SCORE_BG_Z: i32 = 0;
pub const SCORE_DIGITS: usize = 13;
pub const SCORE_FONT_SIZE: f32 = 38.0;


pub const CLEARS_TO_CHANGE_BG: usize = 12;

pub const BG_COLOR1_R: f32 = 0.3;
pub const BG_COLOR1_G: f32 = 0.3;
pub const BG_COLOR1_B: f32 = 0.8;

pub const BG_COLOR2_R: f32 = 0.4;
pub const BG_COLOR2_G: f32 = 0.4;
pub const BG_COLOR2_B: f32 = 0.2;

pub const BG_COLOR3_R: f32 = 0.1;
pub const BG_COLOR3_G: f32 = 0.5;
pub const BG_COLOR3_B: f32 = 0.7;

pub const BG_COLOR4_R: f32 = 0.6;
pub const BG_COLOR4_G: f32 = 0.2;
pub const BG_COLOR4_B: f32 = 0.3;

pub const BG_COLOR5_R: f32 = 0.1;
pub const BG_COLOR5_G: f32 = 0.5;
pub const BG_COLOR5_B: f32 = 0.2;


pub const SQUARE_RED_R: f32 = 1.0;
pub const SQUARE_RED_G: f32 = 0.1;
pub const SQUARE_RED_B: f32 = 0.1;

pub const SQUARE_GREEN_R: f32 = 0.0;
pub const SQUARE_GREEN_G: f32 = 0.8;
pub const SQUARE_GREEN_B: f32 = 0.0;

pub const SQUARE_BLUE_R: f32 = 0.1;
pub const SQUARE_BLUE_G: f32 = 0.1;
pub const SQUARE_BLUE_B: f32 = 1.0;

pub const SQUARE_YELLOW_R: f32 = 0.5;
pub const SQUARE_YELLOW_G: f32 = 0.5;
pub const SQUARE_YELLOW_B: f32 = 0.0;

pub const SQUARE_INNER_COLOR_COEF: f32 = 0.6;
pub const SQUARE_GHOST_COLOR_COEF: f32 = 0.4;

pub const BOARD_Z: i32 = 0;
pub const SQUARES_Z: i32 = 1;

pub const START_DIFFICULTY: f32 = 1.0;
pub const DIFFICULTY_INCREASE: f32 = 0.18;
pub const MAX_DIFFICULTY: f32 = 16.0;

pub const SCORE_REWARD_LINES1: i32 = 200;
pub const SCORE_REWARD_LINES2: i32 = 400;
pub const SCORE_REWARD_LINES3: i32 = 800;
pub const SCORE_REWARD_LINES4: i32 = 1600;
pub const SCORE_REWARD_QUICK_PLACE: i32 = 10;

pub const LOOK_AHEAD_POSITION: (f32, f32) = (6.5, -2.0);
pub const LOOK_AHEAD_WIDTH: f32 = 7.0;
pub const LOOK_AHEAD_HEIGHT: f32 = 14.0;
pub const LOOK_AHEAD_SQUARE_SIZE: f32 = 0.9;
pub const LOOK_AHEAD_START_DRAW: (f32, f32) = (LOOK_AHEAD_POSITION.0 - LOOK_AHEAD_WIDTH*0.7, LOOK_AHEAD_POSITION.1 + LOOK_AHEAD_HEIGHT/2.8);
pub const LOOK_AHEAD_NEXT_PIECE_OFFSET: f32 = LOOK_AHEAD_SQUARE_SIZE*4.0 + LOOK_AHEAD_SQUARE_SIZE/2.0;

pub const HELD_PIECE_POSITION: (f32, f32) = (6.5, 10.0);
pub const HELD_PIECE_BG_SIZE: f32 = LOOK_AHEAD_WIDTH;
pub const HELD_PIECE_SQUARE_SIZE: f32 = 1.2;
pub const HELD_PIECE_EMPTY_TEXT: &str = "HOLD(E)";
pub const HELD_PIECE_EMPTY_TEXT_SIZE: f32 = 32.0;

pub const GAME_OVER_POSITION: (f32, f32) = (0.0, 5.0);
pub const GAME_OVER_TEXT: &str = "Game Over!";
pub const GAME_OVER_TEXT_SIZE: f32 = 72.0;
pub const RESULTS_TEXT_SIZE: f32 = 28.0;
pub const RESULTS_SIZE: (f32, f32) = (500.0, 300.0);

pub const RESTART_TEXT: &str = "Press (R) to restart";
pub const RESTART_TEXT_POSITION: (f32, f32) = (0.0, -13.0);
pub const RESTART_TEXT_SIZE: f32 = 38.0;

pub const PAUSE_TEXT: &str = "Paused (esc)";
pub const PAUSE_TEXT_SIZE: f32 = 38.0;
pub const PAUSE_TEXT_POSITION: (f32, f32) = (0.0, 2.0);

pub const PLACE_PIECE_DELAY_MULTIPLIER: f32 = 8.0;
pub const BASE_PIECE_FALL_SPEED: f32 = 1.0;
pub const PLACE_PIECE_DELAY_MAX: f32 = BASE_PIECE_FALL_SPEED * 1.2;

pub const PUSH_DOWN_SPEED: f32 = 0.5;

pub const LIGHT_EFFECT: f32 = 0.3;

pub const EFFECT_Z: i32 = 2;
pub const CLEAR_EFFECT_DELAY: f32 = 0.15;
pub const PLACE_EFFECT_DELAY: f32 = 0.05;
pub const PLACE_EFFECT_SIZE: f32 = 0.7;
pub const CLEAR_PARTICLE_Z: i32 = 3;
pub const CLEAR_PARTICLE_LIFE: f32 = 0.3;
pub const CLEAR_PARTICLE_VELOCITY_START: f32 = 15.0;
pub const CLEAR_PARTICLE_VELOCITY_END: f32 = 2.0;
pub const CLEAR_PARTICLE_SIZE: f32 = 0.5;
pub const CLEAR_PARTICLES_SPAWN: f32 = 20.0;
pub const CLEAR_PARTICLES_MAX_VERTICIES: usize = 8;

pub const PARTICLE_TEXTURE_TAG: &str = "Particle";

pub const PLACE_SOUND_TAG: &str = "Place";
pub const CLEAR_SOUND_TAG: &str = "Clear";
pub const GAMEOVER_SOUND_TAG: &str = "GameOver";
pub const CANTSWAP_SOUND_TAG: &str = "CantSwap";
pub const SWAP_SOUND_TAG: &str = "Swap";
pub const ROTATE_SOUND_TAG: &str = "Rotate";
pub const MOVE_SOUND_TAG: &str = "Move";
pub const MUSIC_SOUND_TAG: &str = "Music";
