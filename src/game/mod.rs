use std::f32::consts::PI;

use ball::{Ball, BallHitState};
use level::Level;
use macroquad::{color::{BLACK, RED, WHITE}, math::vec2, rand::gen_range, shapes::draw_rectangle_lines, texture::Texture2D, window::clear_background};
use paddle::Paddle;

use crate::Scene;

pub mod paddle;
pub mod ball;
pub mod powerup;
pub mod level;

pub struct Game {
    paddle: Paddle,
    balls: Vec<Ball>,
    level: Level,
}

impl Scene for Game {
    fn new() -> Self {
        let balls = (0..10).map(|_| Ball::new(vec2(gen_range(0.0, 10.0), gen_range(0.0, 10.0)), vec2(1.0, 1.0))).collect();
        Self {
            paddle: Paddle::new(),
            balls,
            level: Level::new(),
        }
    }
    fn update(&mut self) {
        let delta = macroquad::time::get_frame_time();

        let carried = self.paddle.update(delta);
        if let Some(carried) = carried {
            self.balls.push(carried);
        }

        let mut new_carry = None;
        let mut floor_balls = Vec::new();

        for (i, ball) in self.balls.iter_mut().enumerate() {
            let hit_state = ball.update(delta, &self.paddle, &mut self.level);

            if hit_state == BallHitState::Floor {
                floor_balls.push(i);
            }
            if hit_state == BallHitState::Paddle && new_carry.is_none() {
                new_carry = Some(i);
            }
        }

        if let Some(new_carry) = new_carry {
            if self.paddle.can_carry() {
                self.paddle.carry(self.balls.remove(new_carry));
            }
        }
        for i in floor_balls.iter().rev() {
            self.balls.remove(*i);
        }
    }

    fn draw(&self, texture: &Texture2D) {
        clear_background(BLACK);

        self.level.draw(texture);
        self.paddle.draw(texture);
        for b in &self.balls {
            b.draw(texture);
        }
    }
}