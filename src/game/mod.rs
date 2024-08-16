use ball::{Ball, BallHitState, BALL_SIZE, BALL_TEXTURE};
use bullet::Bullet;
use level::Level;
use macroquad::{color::{BLACK, WHITE}, math::{vec2, Rect}, texture::{draw_texture_ex, DrawTextureParams, Texture2D}, window::clear_background};
use paddle::Paddle;
use powerup::{Powerup, PowerupHitState, PowerupKind};

use crate::text_renderer::{render_text, TextAlign};

pub mod paddle;
pub mod ball;
pub mod powerup;
pub mod bullet;
pub mod level;

pub const CARRY_ICON_TEXTURE: Rect = Rect { x: 118.0, y: 8.0, w: 4.0, h: 4.0 };

pub enum Lives {
    None, Some(usize), Infinite,
}

pub struct Game {
    level: Level,
    paddle: Paddle,
    lives: Option<usize>,
    balls:    Vec<Ball>,
    powerups: Vec<Powerup>,
    bullets:  Vec<Bullet>,
}

impl Game {
    pub fn new(level: Level, paddle_pos: Option<f32>, lives: Lives) -> Self {
        let lives = match lives {
            Lives::None => Some(3),
            Lives::Some(l) => Some(l),
            Lives::Infinite => None
        };

        Self {
            level,
            paddle: Paddle::new(paddle_pos),
            lives,
            balls:    Vec::with_capacity(100),
            powerups: Vec::with_capacity(20),
            bullets:  Vec::with_capacity(20),
        }
    }

    pub fn paddle_pos(&self) -> f32 {
        self.paddle.x()
    }

    pub fn update(&mut self) {
        let delta = macroquad::time::get_frame_time();

        // Balls
        let carried = self.paddle.update(delta, &mut self.bullets);
        if let Some(carried) = carried {
            self.balls.push(carried);
        }

        let mut new_carry = None;
        let mut remove_balls = Vec::new();
        for (i, ball) in self.balls.iter_mut().enumerate() {
            let hit_state = ball.update(delta, &self.paddle, &mut self.level, self.paddle.balls_safe());

            if hit_state == BallHitState::Floor {
                remove_balls.push(i);
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

        // Powerups
        while let Some(p) = self.level.powerup_buffer_next() {
            self.powerups.push(Powerup::new(p));
        }

        let mut remove_powerups = Vec::new();
        for (i, powerup) in self.powerups.iter_mut().enumerate() {
            let hit_state = powerup.update(delta, &self.paddle);

            if hit_state == PowerupHitState::Paddle {
                match powerup.kind() {
                    PowerupKind::PaddleCarry => self.paddle.powerup_carry(),
                    PowerupKind::PaddleGrow  => self.paddle.powerup_grow(),
                    PowerupKind::PaddleGun   => self.paddle.powerup_gun(),
                    PowerupKind::BallsSafe   => self.paddle.powerup_balls_safe(),
                    
                    PowerupKind::Zap => println!("zap!"), // TODO: ZAP!
                    PowerupKind::BallsTrail => println!("todo.."),
                    PowerupKind::BallsFive => println!("todo.."),
                };
            }

            if hit_state != PowerupHitState::None {
                remove_powerups.push(i);
            }
        }

        // Bullets
        let mut remove_bullets = Vec::new();
        for (i, b) in self.bullets.iter_mut().enumerate() {
            if b.update(delta, &mut self.level) {
                remove_bullets.push(i);
            }
        }

        for i in remove_balls.iter().rev() {
            self.balls.remove(*i);
        }
        for i in remove_powerups.iter().rev() {
            self.powerups.remove(*i);
        }
        for i in remove_bullets.iter().rev() {
            self.bullets.remove(*i);
        }
    }

    pub fn draw(&self, texture: &Texture2D) {
        clear_background(BLACK);

        // Actual stuff
        for p in &self.powerups {
            p.draw(texture);
        }
        self.level.draw(texture);
        for b in &self.balls {
            b.draw(texture);
        }
        for b in &self.bullets {
            b.draw(texture);
        }
        self.paddle.draw(texture);

        // HUD
        let mut x = 1.0;
        for _ in 0..self.lives.unwrap_or(0) {
            draw_texture_ex(texture, x, Level::view_size().y - BALL_SIZE - 1.0, WHITE, DrawTextureParams {
                source: Some(BALL_TEXTURE),
                ..Default::default()
            });
            x += BALL_SIZE + 1.0;
        }
        for _ in 0..self.paddle.carries() {
            draw_texture_ex(texture, x, Level::view_size().y - BALL_SIZE - 1.0, WHITE, DrawTextureParams {
                source: Some(CARRY_ICON_TEXTURE),
                ..Default::default()
            });
            x += BALL_SIZE + 1.0;
        }

        // Text
        render_text(&String::from("SCORE: 123457"), vec2(0.0, 0.0), WHITE, TextAlign::Left, &texture);
        render_text(self.level.name(), vec2(Level::view_size().x, 0.0), WHITE, TextAlign::Right, &texture);

        render_text(&format!("JUMBLEDFOX :3").to_uppercase(), vec2(0.0, 50.0), WHITE, TextAlign::Left, &texture);

        // "Testing" lol
        // render_text(&format!("O:3").to_uppercase(), vec2(100.0, 200.0), WHITE, TextAlign::Left, &texture);
        // println!("{:?}", self.paddle.center_dist(100.0));
    }
}