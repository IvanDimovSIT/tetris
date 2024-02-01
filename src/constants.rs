pub const WIDTH: usize = 10;
pub const HEIGHT: usize = 20;
pub const LOOK_AHEAD: i32 = 3;

pub const WINDOW_WIDTH: u32 = 800;
pub const WINDOW_HEIGHT: u32 = 800;

pub const MIN_WINDOW_WIDTH: u32 = 750;
pub const MIN_WINDOW_HEIGHT: u32 = 700;

pub const SQUARE_SIZE: f32 = 1.35;
pub const SQUARE_SIZE_INNER_COEF: f32 = 0.7;

pub const GAME_BOARD_TOP_LEFT_POSITION: (f32, f32) = (-SQUARE_SIZE*(WIDTH as f32/2.0)-7.0, SQUARE_SIZE*(HEIGHT as f32/2.0)+0.0);

pub const SCORE_LOCATION: (f32, f32) = (7.5, -12.0);
pub const SCORE_WIDTH: f32 = 13.0;
pub const SCORE_HEIGHT: f32 = 3.0;
pub const SCORE_BG_Z: i32 = 0;
pub const SCORE_DIGITS: usize = 13;
pub const SCORE_FONT_SIZE: f32 = 38.0;

pub const BG_COLOR_R: f32 = 0.3;
pub const BG_COLOR_G: f32 = 0.3;
pub const BG_COLOR_B: f32 = 0.8;


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

pub const SCORE_REWARD_LINES1: i32 = 100;
pub const SCORE_REWARD_LINES2: i32 = 200;
pub const SCORE_REWARD_LINES3: i32 = 400;
pub const SCORE_REWARD_LINES4: i32 = 800;

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

pub const GAME_OVER_TEXT_SIZE: f32 = 48.0;

pub const PLACE_PIECE_DELAY_MULTIPLIER: f32 = 3.0;
pub const PLACE_PIECE_DELAY_MAX: f32 = 1.5; 

pub const LIGHT_EFFECT: f32 = 0.25;

pub const CLEAR_EFFECT_Z: i32 = 2;
pub const CLEAR_EFFECT_DELAY: f32 = 0.15;

pub const PLACE_SOUND_TAG: &str = "Place";
pub const CLEAR_SOUND_TAG: &str = "Clear";
pub const GAMEOVER_SOUND_TAG: &str = "GameOver";
pub const CANTSWAP_SOUND_TAG: &str = "CantSwap";
pub const SWAP_SOUND_TAG: &str = "Swap";
pub const ROTATE_SOUND_TAG: &str = "Rotate";
pub const MOVE_SOUND_TAG: &str = "Move";
