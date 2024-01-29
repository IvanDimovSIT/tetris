pub const WIDTH: usize = 10;
pub const HEIGHT: usize = 20;
pub const LOOK_AHEAD: i32 = 3;

pub const WINDOW_WIDTH: u32 = 800;
pub const WINDOW_HEIGHT: u32 = 800;

pub const MIN_WINDOW_WIDTH: u32 = 750;
pub const MIN_WINDOW_HEIGHT: u32 = 700;

pub const SQUARE_SIZE: f32 = 1.3;
pub const SQUARE_SIZE_INNER_COEF: f32 = 0.7;

pub const GAME_BOARD_TOP_LEFT_POSITION: (f32, f32) = (-SQUARE_SIZE*(WIDTH as f32/2.0)-7.0, SQUARE_SIZE*(HEIGHT as f32/2.0)+0.0);

pub const SCORE_LOCATION: (f32, f32) = (7.5, -10.0);
pub const SCORE_WIDTH: f32 = 13.0;
pub const SCORE_HEIGHT: f32 = 3.0;
pub const SCORE_BG_Z: i32 = 0;
pub const SCORE_DIGITS: usize = 13;
pub const SCORE_FONT_SIZE: f32 = 38.0;

pub const BG_COLOR_R: f32 = 0.4;
pub const BG_COLOR_G: f32 = 0.4;
pub const BG_COLOR_B: f32 = 0.9;


pub const SQUARE_RED_R: f32 = 1.0;
pub const SQUARE_RED_G: f32 = 0.3;
pub const SQUARE_RED_B: f32 = 0.3;

pub const SQUARE_GREEN_R: f32 = 0.3;
pub const SQUARE_GREEN_G: f32 = 1.0;
pub const SQUARE_GREEN_B: f32 = 0.3;

pub const SQUARE_BLUE_R: f32 = 0.3;
pub const SQUARE_BLUE_G: f32 = 0.3;
pub const SQUARE_BLUE_B: f32 = 1.0;

pub const SQUARE_YELLOW_R: f32 = 1.0;
pub const SQUARE_YELLOW_G: f32 = 1.0;
pub const SQUARE_YELLOW_B: f32 = 0.3;

pub const SQUARE_INNER_COLOR_COEF: f32 = 0.6;
pub const SQUARE_GHOST_COLOR_COEF: f32 = 0.4;

pub const BOARD_Z: i32 = 0;
pub const SQUARES_Z: i32 = 1;

pub const START_DIFFICULTY: f32 = 1.0;
pub const DIFFICULTY_INCREASE: f32 = 0.1;

pub const SCORE_REWARD_LINES1: i32 = 100;
pub const SCORE_REWARD_LINES2: i32 = 200;
pub const SCORE_REWARD_LINES3: i32 = 400;
pub const SCORE_REWARD_LINES4: i32 = 800;

pub const LOOK_AHEAD_POSITION: (f32, f32) = (6.5, 3.0);
pub const LOOK_AHEAD_WIDTH: f32 = 8.0;
pub const LOOK_AHEAD_HEIGHT: f32 = 18.0;
pub const LOOK_AHEAD_SQUARE_SIZE: f32 = 1.0;
pub const LOOK_AHEAD_START_DRAW: (f32, f32) = (LOOK_AHEAD_POSITION.0 - LOOK_AHEAD_WIDTH*0.7, LOOK_AHEAD_POSITION.1 + LOOK_AHEAD_HEIGHT/3.0);
pub const LOOK_AHEAD_NEXT_PIECE_OFFSET: f32 = LOOK_AHEAD_SQUARE_SIZE*4.0 + LOOK_AHEAD_SQUARE_SIZE;

pub const GAME_OVER_TEXT_SIZE: f32 = 48.0;
