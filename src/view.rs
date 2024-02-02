use comfy::{draw_rect, load_sound_from_bytes, splat, vec2, Color, StaticSoundSettings, Vec2};

use crate::constants::*;

pub trait Effect{
    fn draw(&mut self, delta: f32);
    fn is_complete(&self) -> bool;
}

#[derive(Clone, Copy)]
pub struct ClearEffect {
    delay: f32,
    time: f32,
    position: Vec2,
    size: Vec2,
}
impl Effect for ClearEffect {
    fn draw(&mut self, delta: f32) {
        self.time -= delta;
        if self.time <= 0.0 {
            return;
        }

        draw_rect(
            self.position,
            self.size,
            Color {
                r: 1.0,
                g: 1.0,
                b: 1.0,
                a: self.time/self.delay
            },
            EFFECT_Z
        );
    }

    fn is_complete(&self) -> bool {
        self.time <= 0.0
    }
}
impl ClearEffect {
    pub fn new(position: Vec2, size: Vec2, delay: f32) -> ClearEffect {
        ClearEffect{delay, time: delay, position, size}
    }
}

pub struct PlaceEffect {
    squares: Vec<(i32, i32)>,
    time: f32,
    delay: f32,
}
impl Effect for PlaceEffect {
    fn draw(&mut self, delta: f32) {
        self.time -= delta;
        if self.time <= 0.0 {
            return;
        }



        for i in &self.squares {
            let position = vec2(
                GAME_BOARD_TOP_LEFT_POSITION.0 + i.0 as f32 *SQUARE_SIZE,
                GAME_BOARD_TOP_LEFT_POSITION.1 - i.1 as f32 *SQUARE_SIZE
            );

            let color = Color{r: 1.0, g:1.0, b:1.0, a:self.time/self.delay};
            draw_rect(position, splat(SQUARE_SIZE+(1.0 - self.time/self.delay)*PLACE_EFFECT_SIZE), color, EFFECT_Z);
        }
    }

    fn is_complete(&self) -> bool {
        self.time <= 0.0
    }
}
impl PlaceEffect {
    pub fn new(squares: Vec<(i32, i32)>) -> PlaceEffect {
        PlaceEffect{squares, time: PLACE_EFFECT_DELAY, delay: PLACE_EFFECT_DELAY}
    }
}

pub fn load_sounds() {
    load_sound_from_bytes(
        PLACE_SOUND_TAG,
        include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/assets/place.wav"
        )),
        StaticSoundSettings::default(),
    );

    load_sound_from_bytes(
        SWAP_SOUND_TAG,
        include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/assets/swap.wav"
        )),
        StaticSoundSettings::default(),
    );

    load_sound_from_bytes(
        CANTSWAP_SOUND_TAG,
        include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/assets/cantswap.wav"
        )),
        StaticSoundSettings::default(),
    );

    load_sound_from_bytes(
        CLEAR_SOUND_TAG,
        include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/assets/clear.wav"
        )),
        StaticSoundSettings::default(),
    );
    
    load_sound_from_bytes(
        GAMEOVER_SOUND_TAG,
        include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/assets/gameover.wav"
        )),
        StaticSoundSettings::default(),
    );

    load_sound_from_bytes(
        ROTATE_SOUND_TAG,
        include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/assets/rotate.wav"
        )),
        StaticSoundSettings::default(),
    );

    load_sound_from_bytes(
        MOVE_SOUND_TAG,
        include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/assets/move.wav"
        )),
        StaticSoundSettings::default(),
    );

    load_sound_from_bytes(
        MUSIC_SOUND_TAG,
        include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/assets/music.wav"
        )),
        StaticSoundSettings::new().loop_region(..),
    );
}
