use macroquad::{color::WHITE, math::{vec2, BVec2, Rect, Vec2}, rand::gen_range, texture::{draw_texture_ex, DrawTextureParams, Texture2D}};

use super::{level::{Level, Tile, LEVEL_HEIGHT, LEVEL_HEIGHT_PADDING_TOP, LEVEL_WIDTH, TILE_GAP, TILE_HEIGHT, TILE_WIDTH}, paddle::Paddle};

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
    Tiles(Vec<usize>),
}

impl Ball {
    pub fn new(pos: Vec2, angle: f32, speed: f32) -> Self {
        Self {
            pos,
            vel: Vec2::from_angle(angle) * speed,
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

    pub fn update(&mut self, delta: f32, paddle: &Paddle, level: &Level, safe: bool) -> BallHitState {
        let mut prev_pos = self.pos;
        let rect = Rect::new(0.0, 0.0, BALL_SIZE, BALL_SIZE);
        let mut bounce = BVec2::new(false, false);
        
        self.pos += self.vel * delta * BALL_SPEED;

        // Ideal approch - check the 3x3 area of tiles around the ball rather than all of them
        let mut tiles_to_check = Vec::with_capacity(9);
        let ball_tile_pos = (self.pos - vec2(0.0, LEVEL_HEIGHT_PADDING_TOP as f32 * TILE_HEIGHT)) / (vec2(TILE_WIDTH, TILE_HEIGHT) + TILE_GAP);
        
        for x in -1..=1 {
            for y in -1..=1 {
                let tile_pos = ball_tile_pos + vec2(x as f32, y as f32);
                let tile_index = match tile_pos.x < 0.0 || tile_pos.x >= LEVEL_WIDTH as f32 || tile_pos.y < 0.0 || tile_pos.x >= LEVEL_HEIGHT as f32 {
                    false => tile_pos.y.floor() as usize * LEVEL_WIDTH + tile_pos.x.floor() as usize,
                    true => continue,
                };
                tiles_to_check.push(tile_index);
            }
        }

        let mut hit_tiles = Vec::new();
        for i in tiles_to_check {
            if !level.tiles().get(i).is_some_and(|t| *t != Tile::Air) {
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
                hit_tiles.push(i);
            }
        }

        let mut hit_paddle = false;
        if self.vel.y > 0.0 && paddle.collision_rect().overlaps(&rect.offset(vec2(prev_pos.x, self.pos.y))) {
            self.pos = prev_pos;
            bounce.y = true;
            hit_paddle = true;

            let center_dist = paddle.center_dist(self.pos.x + BALL_SIZE / 2.0);
            let magnitude = self.vel.length();
            let angle = self.vel.angle_between(vec2(-1.0, 0.0));
            
            let new_angle = angle.to_degrees() - 30.0 * center_dist * self.vel.x.signum();
            let new_angle = new_angle.clamp(90.0 - 60.0, 90.0 + 60.0);

            let new_magnitude = (magnitude * gen_range(1.0, 1.05)).clamp(1.0, 1.3);

            self.vel = Vec2::from_angle(new_angle.to_radians()) * new_magnitude * vec2(-1.0, 1.0);
            // self.vel.y *= 1.0 + center_dist * 0.2; 

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
            prev_pos.y = prev_pos.y.min(Level::view_size().y - BALL_SIZE);
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

        match (!hit_tiles.is_empty(), self.pos.y >= Level::view_size().y, hit_paddle) {
            (true, _, _) => BallHitState::Tiles(hit_tiles),
            (_, true, _) => BallHitState::Floor,
            (_, _, true) => BallHitState::Paddle,
            _ => BallHitState::None,
        }
    }

    pub fn draw(&self, texture: &Texture2D) {
        draw_texture_ex(texture, self.pos.x, self.pos.y, WHITE, DrawTextureParams {
            source: Some(BALL_TEXTURE),
            ..Default::default()
        });
    }
}