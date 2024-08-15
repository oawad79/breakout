use ball::Ball;
use level::Level;
use macroquad::texture::Texture2D;

use crate::Scene;

pub mod ball;
pub mod level;

pub struct Game {
    paddle_pos: f32,
    balls: Vec<Ball>,
    level: Level,
}

impl Scene for Game {
    fn new() -> Self {
        Self {
            paddle_pos: 0.0,
            balls: Vec::with_capacity(50),
            level: Level::new(),
        }
    }
    fn update(&mut self) {
        let delta = macroquad::time::get_frame_time();

    }

    fn draw(&self, texture: &Texture2D) {
        self.level.draw(texture);
    }
}