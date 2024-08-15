use macroquad::{color::WHITE, math::{vec2, Rect, Vec2}, rand::gen_range, texture::{draw_texture_ex, DrawTextureParams, Texture2D}};

#[derive(Clone, Copy, PartialEq, Eq)]
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
            x: along as f32 * (TILE_WIDTH + 1.0) + 1.0,
            y: 1.0,
            w: TILE_WIDTH,
            h: TILE_HEIGHT,
        }
    }

    pub fn breakable(&self) -> bool {
        !matches!(*self, Self::Air | Self::Metal | Self::Gold)
    }
    pub fn hit(&mut self) {
        if !self.breakable() {
            return;
        }
        *self = match *self {
            Self::Stone => Self::StoneCracked,
            _ => Self::Air,
        }
    }
}

pub const LEVEL_WIDTH: usize = 100;
pub const LEVEL_HEIGHT: usize = 70;

pub const LEVEL_HEIGHT_PADDING_TOP: usize = 2;
pub const LEVEL_HEIGHT_PADDING_BOTTOM: usize = 6;

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

        let mut tiles = [Tile::Air; LEVEL_WIDTH*LEVEL_HEIGHT];

        for (i, t) in &mut tiles.iter_mut().rev().enumerate() {
            // if i / LEVEL_WIDTH <= 3 {
            //     continue;
            // }
            *t = match gen_range(0, 13) {
                1  => Tile::Red,
                2  => Tile::Orange,
                3  => Tile::Yellow,
                4  => Tile::Green,
                5  => Tile::Cyan,
                6  => Tile::Blue,
                7  => Tile::Pink,
                8  => Tile::Stone,
                9  => Tile::StoneCracked,
                10 => Tile::Metal,
                11 => Tile::Gold,
                _  => Tile::Air,
            };
            
            // if (i % LEVEL_WIDTH) == 0 || (i % LEVEL_WIDTH) == LEVEL_WIDTH - 1 {
            //     continue;
            // } 
            // *t = match i / LEVEL_WIDTH {
            //     16 => Tile::Stone,
            //     17 => Tile::Red,
            //     18 => Tile::Orange,
            //     19 => Tile::Yellow,
            //     20 => Tile::Green,
            //     21 => Tile::Cyan,
            //     22 => Tile::Blue,
            //     23 => Tile::Pink,
            //     _  => Tile::Air,
            // };
        }

        Self {
            tiles,
            name: String::from("NEW LEVEL"),
        }
    }

    pub fn tiles_mut(&mut self) -> &mut [Tile] {
        &mut self.tiles
    }

    pub fn tile_pos(index: usize) -> Vec2 {
        let (x, y) = (
            index % LEVEL_WIDTH,
            index / LEVEL_WIDTH + LEVEL_HEIGHT_PADDING_TOP
        );
        vec2(x as f32, y as f32) * (vec2(TILE_WIDTH, TILE_HEIGHT) + TILE_GAP)
    }

    pub fn tile_rect(index: usize) -> Rect {
        let pos = Level::tile_pos(index);
        Rect::new(
            pos.x,
            pos.y,
            TILE_WIDTH,
            TILE_HEIGHT,
        )
    }

    pub fn view_size() -> Vec2 {
        vec2(LEVEL_WIDTH as f32, (LEVEL_HEIGHT + LEVEL_HEIGHT_PADDING_TOP + LEVEL_HEIGHT_PADDING_BOTTOM) as f32) * (vec2(TILE_WIDTH, TILE_HEIGHT) + TILE_GAP)
    }

    pub fn draw(&self, texture: &Texture2D) {
        for (i, t) in self.tiles.iter().enumerate() {
            let tile_pos = Level::tile_pos(i);
            let tile_rect = t.texture_rect();

            draw_texture_ex(texture, tile_pos.x, tile_pos.y, WHITE, DrawTextureParams {
                source: Some(tile_rect),
                ..Default::default()
            });
        }
    }
}