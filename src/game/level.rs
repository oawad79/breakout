use macroquad::{color::WHITE, math::Rect, texture::{draw_texture_ex, DrawTextureParams, Texture2D}};

#[derive(Clone, Copy)]
pub enum Tile {
    Air,
    Red, Orange, Yellow, Green, Cyan, Blue, Pink,
    Stone, StoneCracked,
    Metal, Gold,
}

impl Tile {
    pub fn texture_rect(&self) -> Rect {
        if matches!(self, Tile::Air) {
            return Rect::new(0.0, 0.0, 0.0, 0.0);
        }
        let along = *self as usize - 1;
        Rect {
            x: along as f32 * TILE_WIDTH,
            y: 0.0,
            w: TILE_WIDTH,
            h: TILE_HEIGHT,
        }
    }
}

pub const LEVEL_WIDTH: usize = 16;
pub const LEVEL_HEIGHT: usize = 27;
pub const TILE_WIDTH: f32 = 11.0;
pub const TILE_HEIGHT: f32 = 6.0;
pub const TILE_GAP: f32 = 1.0;

pub const NAME_LEN: usize = 20;

pub struct Level {
    tiles: [Tile; LEVEL_WIDTH*LEVEL_HEIGHT],
    name: String,
}

impl Level {
    pub fn new() -> Self {
        Self {
            tiles: [Tile::Gold; LEVEL_WIDTH*LEVEL_HEIGHT],
            name: String::from("NEW LEVEL"),
        }
    }

    pub fn draw(&self, texture: &Texture2D) {
        for (i, t) in self.tiles.iter().enumerate() {
            let (x, y) = (i % LEVEL_WIDTH, i / LEVEL_WIDTH);
            let (tile_x, tile_y) = (x as f32 * (TILE_WIDTH + TILE_GAP), y as f32 * (TILE_HEIGHT + TILE_GAP));
            let tile_rect = t.texture_rect();

            draw_texture_ex(texture, tile_x, tile_y, WHITE, DrawTextureParams {
                source: Some(tile_rect),
                ..Default::default()
            });
        }
    }
}