use macroquad::math::Vec2;

use super::level::Level;

pub struct Ball {
    pos: Vec2,
    vel: Vec2,
    // tile_pos: U16Vec2,
    // tile_pos_prev: U16Vec2,
}

impl Ball {
    pub fn update(&mut self, delta: f32, _level: &mut Level) {
        self.pos += self.vel * delta;
        // self.tile_pos_prev = self.tile_pos;
        // self.ti
    }

}