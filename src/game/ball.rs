use macroquad::{color::WHITE, math::{vec2, BVec2, Rect, Vec2}, texture::{draw_texture_ex, DrawTextureParams, Texture2D}};

use super::{level::{Level, Tile}, paddle::Paddle};

pub const BALL_SIZE: f32 = 4.0;

pub struct Ball {
    pos: Vec2,
    vel: Vec2,
}

#[derive(PartialEq, Eq)]
pub enum BallHitState {
    None,
    Tile { index: usize },
    Paddle,
    Floor,
}

impl Ball {
    pub fn new(pos: Vec2, vel: Vec2) -> Self {
        Self {
            pos,
            vel,
        }
    }

    pub fn pos(&self) -> Vec2 {
        self.pos
    }

    pub fn set_pos(&mut self, pos: Vec2) {
        self.pos = pos;
    }

    pub fn update(&mut self, delta: f32, paddle: &Paddle, level: &mut Level) -> BallHitState {
        let prev_pos = self.pos;
        let rect = Rect::new(0.0, 0.0, BALL_SIZE, BALL_SIZE);
        let mut bounce = BVec2::new(false, false);
        
        self.pos += self.vel * delta * 70.0;

        // Naive approach - checking EVERY TILE
        // TODO: Ideal approch - check the 3x3 area of tiles
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
                t.hit();
            }
        }

        let mut hit_paddle = false;
        if self.vel.y > 0.0 && paddle.collision_rect().overlaps(&rect.offset(vec2(prev_pos.x, self.pos.y))) {
            self.pos = prev_pos;
            bounce.y = true;

            hit_paddle = true;
        }

        if self.pos.x <= 0.0 || self.pos.x >= Level::view_size().x - BALL_SIZE {
            bounce.x = true;
        }
        if self.pos.y <= 0.0 {
            bounce.y = true;
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
            return BallHitState::Floor;
        }
        if hit_paddle {
            return BallHitState::Paddle;
        }

        BallHitState::None
    }

    pub fn draw(&self, texture: &Texture2D) {
        draw_texture_ex(texture, self.pos.x, self.pos.y, WHITE, DrawTextureParams {
            source: Some(Rect::new(1.0, 8.0, 4.0, 4.0)),
            ..Default::default()
        });
    }
}