use macroquad::{color::WHITE, math::{vec2, Rect, Vec2}, rand::gen_range, texture::{draw_texture_ex, DrawTextureParams, Texture2D}};

use super::{level::Level, paddle::Paddle};

const POWERUP_TEX_ORIGIN: Vec2 = vec2(15.0, 8.0); 
const POWERUP_SIZE: Vec2 = vec2(13.0, 7.0); 

#[derive(Clone, Copy)]
pub enum PowerupKind {
    PaddleCarry, PaddleGun, PaddleGrow, Zap, BallsFive, BallsTrail, BallsSafe
}

#[derive(PartialEq, Eq)]
pub enum PowerupHitState {
    None, Paddle, Floor,
}

pub struct Powerup {
    pos: Vec2,
    kind: PowerupKind,
    fall_speed: f32,
}

impl Powerup {
    pub fn new(tile_index: usize, spawn_carry: bool) -> Self {
        let random = gen_range(0, if spawn_carry {6} else {5});
        Self {
            pos: Level::tile_pos(tile_index) - 1.0,
            kind: match random {
                0 => PowerupKind::PaddleGun,
                1 => PowerupKind::PaddleGrow,
                2 => PowerupKind::BallsFive,
                3 => PowerupKind::BallsTrail,
                4 => PowerupKind::BallsSafe,
                _ => PowerupKind::PaddleCarry,
            },
            fall_speed: gen_range(25.0, 40.0),
        }
    }

    pub fn can_stop_game_over(&self) -> bool {
        matches!(self.kind, PowerupKind::PaddleGun | PowerupKind::BallsFive)
    }

    pub fn kind(&self) -> PowerupKind {
        self.kind
    }

    pub fn update(&mut self, delta: f32, paddle: &Paddle) -> PowerupHitState {
        self.pos.y += delta * self.fall_speed;

        let rect = Rect::new(self.pos.x, self.pos.y, POWERUP_SIZE.x, POWERUP_SIZE.y);
        if paddle.collision_rect().overlaps(&rect) {
            PowerupHitState::Paddle
        }
        else if self.pos.y >= Level::view_size().y + POWERUP_SIZE.y {
            PowerupHitState::Floor
        }
        else {
            PowerupHitState::None
        }
    }

    pub fn draw(&self, texture: &Texture2D) {
        let source_pos = POWERUP_TEX_ORIGIN + vec2(self.kind as usize as f32 * (POWERUP_SIZE.x + 1.0), 0.0); 
        let source = Rect::new(source_pos.x, source_pos.y, POWERUP_SIZE.x, POWERUP_SIZE.y);

        draw_texture_ex(texture, self.pos.x, self.pos.y, WHITE, DrawTextureParams {
            source: Some(source),
            ..Default::default()
        })
    }
}