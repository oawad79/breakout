use ball::{Ball, BallHitState, BALL_SIZE, BALL_TEXTURE};
use bullet::{Bullet, BulletHitState};
use level::Level;
use macroquad::{color::{Color, WHITE}, color_u8, math::{vec2, Rect}, rand::{gen_range, ChooseRandom}, texture::{draw_texture_ex, DrawTextureParams, Texture2D}, window::clear_background};
use paddle::Paddle;
use powerup::{Powerup, PowerupHitState, PowerupKind};

use crate::text_renderer::{render_text, TextAlign};

pub mod paddle;
pub mod ball;
pub mod powerup;
pub mod bullet;
pub mod level;

pub const CARRY_ICON_TEXTURE: Rect = Rect { x: 118.0, y: 8.0, w: 4.0, h: 4.0 };
pub const SAFE_TEXTURE: Rect = Rect { x: 157.0, y: 18.0, w: 1.0, h: 6.0 };
pub const BG_COL: Color = color_u8!(25, 31, 58, 255);

pub enum Lives {
    Default, Some(usize), Infinite,
}

#[derive(PartialEq, Eq)]
pub enum WorldUpdateReturn {
    BallStuck,
    GameOver,
    None,
}

pub struct World {
    level: Level,
    paddle: Paddle,
    lives: Option<usize>,
    balls:    Vec<Ball>,
    powerups: Vec<Powerup>,
    bullets:  Vec<Bullet>,
    score: usize,

    balls_to_dispense: Vec<(f32, Vec<Ball>)>,

    ball_stuck_timer: f32,
    next_powerup: usize,
}

impl World {
    pub fn new(level: Level, score: Option<usize>, paddle_pos: Option<f32>, lives: Lives, carries: Option<usize>) -> Self {
        let lives = match lives {
            Lives::Default => Some(2),
            Lives::Some(l) => Some(l),
            Lives::Infinite => None
        };

        Self {
            level,
            paddle: Paddle::new(paddle_pos, carries),
            lives,
            balls:    Vec::with_capacity(100),
            powerups: Vec::with_capacity(20),
            bullets:  Vec::with_capacity(20),
            score: score.unwrap_or(0),
            balls_to_dispense: Vec::with_capacity(5),

            ball_stuck_timer: 0.0,
            next_powerup: gen_range(0, 5),
        }
    }

    pub fn level_complete(&self) -> bool {
        !self.level.tiles().iter().any(|t| t.breakable())
    }

    pub fn score(&self) -> usize {
        self.score
    }
    pub fn paddle_pos(&self) -> f32 {
        self.paddle.x()
    }
    pub fn lives(&self) -> Lives {
        match self.lives {
            Some(l) => Lives::Some(l),
            None => Lives::Infinite,
        }
    }
    pub fn carries(&self) -> usize {
        self.paddle.carries()
    }

    pub fn break_tile(&mut self, index: usize) {
        if !self.level.break_tile(index) {
            return;
        }
        self.score += 10;
        if self.next_powerup == 0 {
            self.next_powerup = gen_range(2, 5);
            // TODO: Balance powerup giving
            self.powerups.push(Powerup::new(index));
            return;
        }
        self.next_powerup -= 1;
        self.ball_stuck_timer = 0.0;
    }

    pub fn give_free_ball(&mut self) {
        self.paddle.carry_new();
    }

    pub fn trail_balls(&mut self) {
        let amount = 4;
        let mut new_balls = Vec::with_capacity(self.balls.len() * amount);
        for b in &self.balls {
            let mut speed = 1.0;
            for _ in 0..amount {
                speed -= 0.1;
                new_balls.push(Ball::new(b.pos(), b.vel().to_angle(), speed));
            }
        }
        new_balls.shuffle();
        new_balls.truncate(20);
        self.balls.extend(new_balls);
    }

    pub fn dispense_angled_balls(&mut self, amount: usize) {
        let pos = vec2(self.paddle.x(), Level::view_size().y - gen_range(23.0, 40.0));
        let rotation = gen_range(-90.0, -75.0);
        let rotation_step = gen_range(5.0, 15.0);

        let mut current_rotation: f32 = rotation;
        let mut balls = Vec::new();
        for _ in 0..amount {
            balls.push(Ball::new(pos, current_rotation.to_radians(), 1.0));
            current_rotation += rotation_step;
        }
        self.balls_to_dispense.push((f32::INFINITY, balls));
    }

    pub fn update(&mut self) -> WorldUpdateReturn {
        let delta = macroquad::time::get_frame_time();
        if !self.paddle.carrying() {
            self.ball_stuck_timer += delta;
        } else {
            self.ball_stuck_timer = 0.0;
        }

        // Balls
        let carried = self.paddle.update(delta, &mut self.bullets);
        if let Some(carried) = carried {
            self.balls.push(carried);
        }

        for (t, balls) in &mut self.balls_to_dispense {
            *t += delta;
            if *t >= 0.25 {
                if let Some(b) = balls.pop() {
                    self.balls.push(b);
                }
                *t = 0.0;
            }
        }

        self.balls_to_dispense.retain(|(_, b)| !b.is_empty());

        let mut hit_tiles = Vec::new();
        let mut new_carry = None;
        let mut remove_balls = Vec::new();
        for (i, ball) in self.balls.iter_mut().enumerate() {
            let hit_state = ball.update(delta, &self.paddle, &mut self.level, self.paddle.balls_safe());

            if hit_state == BallHitState::Floor {
                remove_balls.push(i);
            }
            if hit_state == BallHitState::Paddle {
                self.ball_stuck_timer = 0.0;
                if new_carry.is_none() {
                    new_carry = Some(i);
                }
            }
            if let BallHitState::Tiles(tiles) = hit_state {
                hit_tiles.extend(tiles);
            }
        }

        if let Some(new_carry) = new_carry {
            if self.paddle.can_carry() {
                self.paddle.carry(self.balls.remove(new_carry));
            }
        }

        // Bullets
        let mut remove_bullets = Vec::new();
        for (i, b) in self.bullets.iter_mut().enumerate() {
            let hit_state = b.update(delta, &mut self.level);
            
            if let BulletHitState::Tile(index) = hit_state {
                hit_tiles.push(index);
            }
            if hit_state != BulletHitState::None {
                remove_bullets.push(i);
            }
        }

        for index in hit_tiles {
            self.break_tile(index);
        }

        // Powerups
        let mut remove_powerups = Vec::new();
        let mut angled_balls = Vec::new();
        let mut trail = false;
        for (i, powerup) in self.powerups.iter_mut().enumerate() {
            let hit_state = powerup.update(delta, &self.paddle);

            if hit_state == PowerupHitState::Paddle {
                self.score += 15;
                match powerup.kind() {
                    PowerupKind::PaddleCarry => self.paddle.powerup_carry(),
                    PowerupKind::PaddleGrow  => self.paddle.powerup_grow(),
                    PowerupKind::PaddleGun   => self.paddle.powerup_gun(),
                    PowerupKind::BallsSafe   => self.paddle.powerup_balls_safe(),
                    PowerupKind::BallsFive   => angled_balls.push(5),
                    PowerupKind::BallsTrail  => trail = true,
                    _ => {},
                };
            }
            if hit_state != PowerupHitState::None {
                remove_powerups.push(i);
            }
        }

        for b in angled_balls {
            self.dispense_angled_balls(b);
        }
        if trail {
            self.trail_balls();
        }

        // All balls are gone, there are no powerups left, the game is lost! Lose a life and either dispense another ball or game-over
        // Only if we haven't won of course!!
        let mut gameover = false;

        // If there are no balls in play
        if self.balls.is_empty() && !self.paddle.carrying() && self.balls_to_dispense.is_empty() {
            // If we don't have infinite lives and the level isn't complete
            if self.lives.is_some() && !self.level_complete() {
                // As a last ditch attempt to avoid a game over, don't gameover if we have a gun / bullets, or if there are any powerups that can stop a game over
                if self.lives == Some(0) && !self.paddle.has_gun_powerup() && self.bullets.is_empty() && !self.powerups.iter().any(|p| p.can_stop_game_over()){
                    gameover = true;
                }
                if self.lives.is_some_and(|l| l != 0) {
                    self.lives = self.lives.map(|l| l - 1);
                    self.paddle.carry_new();
                }
            }
            // Otherwise if we do have infinite lives just give a new ball
            if self.lives.is_none() {
                self.paddle.carry_new();
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

        match (gameover, self.ball_stuck_timer >= 30.0) {
            (true, _) => WorldUpdateReturn::GameOver,
            (_, true) => {self.ball_stuck_timer = 0.0; WorldUpdateReturn::BallStuck},
            _ => WorldUpdateReturn::None
        }
    }

    pub fn draw(&self, texture: &Texture2D) {
        clear_background(BG_COL);

        let view_size = Level::view_size();
        // Safety net
        if self.paddle.balls_safe_display() {
            draw_texture_ex(texture, 0.0, view_size.y - SAFE_TEXTURE.h, WHITE, DrawTextureParams {
                source: Some(SAFE_TEXTURE),
                dest_size: Some(vec2(view_size.x, SAFE_TEXTURE.h)),
                ..Default::default()
            });
        }
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
            draw_texture_ex(texture, x, view_size.y - BALL_SIZE - 1.0, WHITE, DrawTextureParams {
                source: Some(BALL_TEXTURE),
                ..Default::default()
            });
            x += BALL_SIZE + 1.0;
        }
        for _ in 0..self.paddle.carries() {
            draw_texture_ex(texture, x, view_size.y - BALL_SIZE - 1.0, WHITE, DrawTextureParams {
                source: Some(CARRY_ICON_TEXTURE),
                ..Default::default()
            });
            x += BALL_SIZE + 1.0;
        }

        render_text(&format!("SCORE: {}", self.score), vec2(0.0, 0.0), WHITE, TextAlign::Left, &texture);
        render_text(self.level.name(), vec2(Level::view_size().x, 0.0), WHITE, TextAlign::Right, &texture);
        render_text(&format!("JUMBLEDFOX.GITHUB.IO").to_uppercase(), Level::view_size() - vec2(0.0, 7.0), Color::from_rgba(255, 255, 255, 128), TextAlign::Right, &texture);
    }
}