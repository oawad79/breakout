use macroquad::{color::WHITE, math::{vec2, Rect, Vec2}, rand::gen_range, texture::{draw_texture_ex, DrawTextureParams, Texture2D}};

pub const LEVEL_WIDTH: usize = 16;
pub const LEVEL_HEIGHT: usize = 22;

pub const LEVEL_HEIGHT_PADDING_TOP: usize = 2;
pub const LEVEL_HEIGHT_PADDING_BOTTOM: usize = 6;

pub const TILE_WIDTH: f32 = 11.0;
pub const TILE_HEIGHT: f32 = 6.0;
pub const TILE_GAP: f32 = 1.0;

pub const LEVEL_NAME_LEN: usize = 16;

pub type TileArray = [Tile; LEVEL_WIDTH*LEVEL_HEIGHT];


#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Tile {
    White, Red, Orange, Yellow, Green, Cyan, Blue, Purple, Pink, Brown, Black,
    Stone, StoneCracked,
    Metal, Gold,
    Air,
}

impl Tile {
    pub fn texture_rect(&self) -> Rect {
        let along = *self as usize;
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
    // Returns whether it dropped a powerup
    pub fn hit(&mut self) -> bool {
        if !self.breakable() {
            return false;
        }

        let spawn_powerup = gen_range(0, 15) == 0 && !matches!(*self, Tile::Stone | Tile::StoneCracked);

        *self = match *self {
            Self::Stone => Self::StoneCracked,
            _ => Self::Air,
        };

        spawn_powerup
    }
}

#[derive(Clone)]
pub struct Level {
    tiles: TileArray,
    powerup_buffer: Vec<usize>,
    name: String,
}

impl Level {
    pub fn new() -> Self {
        Self {
            tiles: [Tile::Air; LEVEL_WIDTH*LEVEL_HEIGHT],
            powerup_buffer: Vec::with_capacity(10),
            name: String::new(),
        }
    }

    pub fn tiles(&self) -> &TileArray {
        &self.tiles
    }
    pub fn tiles_mut(&mut self) -> &mut TileArray {
        &mut self.tiles
    }
    pub fn name(&self) -> &String {
        &self.name
    }
    pub fn name_mut(&mut self) -> &mut String {
        &mut self.name
    }

    pub fn break_tile(&mut self, index: usize) {
        self.tiles.get_mut(index).map(|t| if t.hit() {
            self.powerup_buffer.push(index)
        });
    }
    
    pub fn powerup_buffer_next(&mut self) -> Option<usize> {
        self.powerup_buffer.pop()
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
            if *t == Tile::Air {
                continue;
            }
            let tile_pos = Level::tile_pos(i);
            let tile_rect = t.texture_rect();

            draw_texture_ex(texture, tile_pos.x, tile_pos.y, WHITE, DrawTextureParams {
                source: Some(tile_rect),
                ..Default::default()
            });
        }
    }
}