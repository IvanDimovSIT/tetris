use comfy::{ 
    draw_rect, load_sound_from_bytes, random_box, random_usize, spawn_particle, splat, texture_id, vec2, BlendMode,
    Color, FadeType, Particle, StaticSoundSettings, Trail, TrailRef, Vec2, BLACK, BLUE, GREEN, RED, YELLOW
};

use crate::{constants::*, model::{self, Square}};

pub trait Effect{
    fn draw(&mut self, delta: f32);
    fn is_complete(&self) -> bool;
}

pub struct ClearEffect {
    delay: f32,
    time: f32,
    position: Vec2,
    size: Vec2
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

        self.draw_particles(delta);
    }

    fn is_complete(&self) -> bool {
        self.time <= 0.0
    }
}
impl ClearEffect {
    pub fn new(position: Vec2, size: Vec2, delay: f32) -> ClearEffect {
        ClearEffect{delay, time: delay, position, size}
    }

    fn draw_particles(&self, delta: f32) {
        let draw_count: i32 = (self.size.x * self.size.y * CLEAR_PARTICLES_SPAWN * delta) as i32; 

        for _ in 0..draw_count {
            let color = vec![RED, GREEN, BLUE, YELLOW][random_usize(0, 3)];

            spawn_particle(Particle {
                position: random_box(self.position, self.size),
                size: splat(CLEAR_PARTICLE_SIZE),
                velocity: CLEAR_PARTICLE_VELOCITY_START,
                velocity_end: CLEAR_PARTICLE_VELOCITY_END,
                lifetime_max: CLEAR_PARTICLE_LIFE,
                fade_type: FadeType::Alpha,
                texture: texture_id(PARTICLE_TEXTURE_TAG),
                trail: TrailRef::Local(Trail::new(
                    CLEAR_PARTICLE_SIZE/5.0,
                    1.0,
                    CLEAR_PARTICLE_Z,
                    color,
                    BLACK,
                    CLEAR_PARTICLES_MAX_VERTICIES,
                    0.5,
                    5.0,
                    None,
                    None,
                    BlendMode::Additive,
                )),
                ..Default::default()
            });
        }
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

pub fn put_square(center: Vec2, square: &Square, square_size: f32) {
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

pub struct BGColorSelector {
    colors: Vec<comfy::Color>,
    current: usize,
    clears: usize,
    clears_needed: usize,
}
impl BGColorSelector{
    pub fn new(clears_needed: usize) -> BGColorSelector {
        BGColorSelector{
            colors: vec![
                comfy::Color{r:BG_COLOR1_R, g:BG_COLOR1_G, b:BG_COLOR1_B, a:1.0},
                comfy::Color{r:BG_COLOR2_R, g:BG_COLOR2_G, b:BG_COLOR2_B, a:1.0},
                comfy::Color{r:BG_COLOR3_R, g:BG_COLOR3_G, b:BG_COLOR3_B, a:1.0},
                comfy::Color{r:BG_COLOR4_R, g:BG_COLOR4_G, b:BG_COLOR4_B, a:1.0},
                comfy::Color{r:BG_COLOR5_R, g:BG_COLOR5_G, b:BG_COLOR5_B, a:1.0},
            ],
            current: 0,
            clears: 0,
            clears_needed
        }
    }

    pub fn on_cleared(&mut self) {
        self.clears += 1;
        if self.clears >= self.clears_needed {
            self.clears = 0;
            self.current += 1;
            if self.current >= self.colors.len() {
                self.current = 0;
            }
        }
    }

    pub fn get_color(&self) -> comfy::Color {
        self.colors[self.current]
    }
}
