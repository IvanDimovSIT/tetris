use comfy::{draw_rect, Color, Vec2};

use crate::constants::*;

#[derive(Clone, Copy)]
pub struct ClearEffect {
    delay: f32,
    time: f32,
    position: Vec2,
    size: Vec2,
}
impl ClearEffect {
    pub fn new(position: Vec2, size: Vec2, delay: f32) -> ClearEffect {
        ClearEffect{delay, time: delay, position, size}
    }

    pub fn draw(&mut self, delta: f32) {
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
                a: self.time/(self.delay) 
            },
            CLEAR_EFFECT_Z
        );
    }

    pub fn is_complete(&self) -> bool {
        self.time <= 0.0
    }
}

