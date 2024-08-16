use macroquad::{color::WHITE, math::{vec2, BVec2, Rect, Vec2}, texture::{draw_texture_ex, DrawTextureParams, Texture2D}};

use super::{level::{Level, Tile}, paddle::Paddle};

pub const BALL_SIZE: f32 = 4.0;
pub const BALL_SPEED: f32 = 70.0;

pub const BALL_TEXTURE: Rect = Rect { x: 113.0, y: 8.0, w: 4.0, h: 4.0 };

pub struct Ball {
    pos: Vec2,
    vel: Vec2,
}

#[derive(PartialEq, Eq)]
pub enum BallHitState {
    None,
    Paddle,
    Floor,
}

impl Ball {
    pub fn new(pos: Vec2, angle: f32) -> Self {
        Self {
            pos,
            vel: Vec2::from_angle(angle),
        }
    }

    pub fn pos(&self) -> Vec2 {
        self.pos
    }
    pub fn vel(&self) -> Vec2 {
        self.vel
    }
    pub fn set_pos(&mut self, pos: Vec2) {
        self.pos = pos;
    }
    pub fn set_vel(&mut self, vel: Vec2) {
        self.vel = vel;
    }

    pub fn update(&mut self, delta: f32, paddle: &Paddle, level: &mut Level, safe: bool) -> BallHitState {
        let prev_pos = self.pos;
        let rect = Rect::new(0.0, 0.0, BALL_SIZE, BALL_SIZE);
        let mut bounce = BVec2::new(false, false);
        
        self.pos += self.vel * delta * BALL_SPEED;

        // Naive approach - checking EVERY TILE
        // TODO: Ideal approch - check the 3x3 area of tiles
        let mut tiles_to_break = Vec::new();
        for (i, t) in level.tiles_mut().iter_mut().enumerate() {
            if *t == Tile::Air {
                continue;
            }

            let tile_rect = Level::tile_rect(i);

            let mut break_tile = false;
            if tile_rect.overlaps(&rect.offset(vec2(self.pos.x, prev_pos.y))) {
                bounce.x = true;
                break_tile = true;
            }
            if tile_rect.overlaps(&rect.offset(vec2(prev_pos.x, self.pos.y))) {
                bounce.y = true;
                break_tile = true;
            }

            if break_tile {
                tiles_to_break.push(i);
            }
        }

        for i in tiles_to_break {
            level.break_tile(i);
        }

        // TODO: Paddle physics
        let mut hit_paddle = false;
        if self.vel.y > 0.0 && paddle.collision_rect().overlaps(&rect.offset(vec2(prev_pos.x, self.pos.y))) {
            self.pos = prev_pos;
            bounce.y = true;
            hit_paddle = true;

            let center_dist = paddle.center_dist(self.pos.x + BALL_SIZE / 2.0);
            self.vel.y *= 1.0 + center_dist * 0.2; 

            // let angle = match center_dist.abs() {
            //     0.6.. => 70,
            //     _ => 3,
            // };

            // let angle: f32 = 70.0_f32.to_radians() * center_dist * self.vel.x.signum();
            // self.vel = self.vel.rotate(Vec2::from_angle(angle));

            // draw_line(self.pos.x, self.pos.y, self.pos.x + Vec2::from_angle(angle).x * 20.0, self.pos.y + Vec2::from_angle(angle).y * 20.0, 2.0, PINK);

            // let angle = 45.0_f32.to_radians() * paddle.center_dist(self.pos.x) * self.vel.x.signum();
            // self.vel = self.vel.rotate(Vec2::from_angle(angle));
        }

        if self.pos.x <= 0.0 || self.pos.x >= Level::view_size().x - BALL_SIZE {
            bounce.x = true;
        }
        if self.pos.y <= 0.0 || (safe && self.pos.y >= Level::view_size().y - BALL_SIZE) {
            bounce.y = true;
        }

        if bounce.any() {
            // self.vel = self.vel.rotate(Vec2::from_angle(gen_range(-0.05, 0.05)));
        }

        if bounce.x {
            self.pos.x = prev_pos.x;
            self.vel.x *= -1.0;
        }
        if bounce.y {
            self.pos.y = prev_pos.y;
            self.vel.y *= -1.0;
        }

        if self.pos.y >= Level::view_size().y {
            BallHitState::Floor
        } else
        if hit_paddle {
            BallHitState::Paddle
        }
        else {
            BallHitState::None
        }

    }

    pub fn draw(&self, texture: &Texture2D) {
        draw_texture_ex(texture, self.pos.x, self.pos.y, WHITE, DrawTextureParams {
            source: Some(BALL_TEXTURE),
            ..Default::default()
        });
    }
}