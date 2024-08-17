use macroquad::{color::WHITE, math::{vec2, Rect, Vec2}, texture::{draw_texture_ex, DrawTextureParams, Texture2D}};

use super::level::Level;

const BULLET_SPEED: f32 = 200.0;
const BULLET_HEIGHT: f32 = 6.0;

pub const BULLET_TEXTURE: Rect = Rect { x: 130.0, y: 8.0, w: 1.0, h: 1.0 };

pub struct Bullet {
    pos: Vec2,
}

impl Bullet {
    pub fn new(pos: Vec2) -> Self {
        Self { pos }
    }
    pub fn update(&mut self, delta: f32, level: &mut Level) -> bool {
        self.pos.y -= delta * BULLET_SPEED;

        let rect = Rect::new(self.pos.x, self.pos.y + BULLET_HEIGHT, 2.0, 1.0);
        let mut hit_tile = None;
        for (i, t) in level.tiles_mut().iter_mut().enumerate() {
            if Level::tile_rect(i).overlaps(&rect) && t.breakable() {
                hit_tile = Some(i);
                break;
            }
        }
        if let Some(i) = hit_tile {
            level.break_tile(i);
        }
        hit_tile.is_some()
    }

    pub fn draw(&self, texture: &Texture2D) {
        draw_texture_ex(texture, self.pos.x, self.pos.y - BULLET_HEIGHT, WHITE, DrawTextureParams {
            source: Some(BULLET_TEXTURE),
            dest_size: Some(vec2(1.0, BULLET_HEIGHT)),
            ..Default::default()
        })
    }
}