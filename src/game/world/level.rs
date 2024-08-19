use macroquad::{color::WHITE, math::{vec2, Rect, Vec2}, texture::{draw_texture_ex, DrawTextureParams, Texture2D}};

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

impl TryFrom<u8> for Tile {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0  => Ok(Tile::White),
            1  => Ok(Tile::Red),
            2  => Ok(Tile::Orange),
            3  => Ok(Tile::Yellow),
            4  => Ok(Tile::Green),
            5  => Ok(Tile::Cyan),
            6  => Ok(Tile::Blue),
            7  => Ok(Tile::Purple),
            8  => Ok(Tile::Pink),
            9  => Ok(Tile::Brown),
            10 => Ok(Tile::Black),
            11 => Ok(Tile::Stone),
            12 => Ok(Tile::StoneCracked),
            13 => Ok(Tile::Metal),
            14 => Ok(Tile::Gold),
            15 => Ok(Tile::Air),
            _ => Err(())
        }
    }
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

    pub fn hit(&mut self) -> bool {
        if !self.breakable() {
            return false;
        }
        *self = match *self {
            Self::Stone => Self::StoneCracked,
            _ => Self::Air,
        };
        *self == Tile::Air
    }
}

#[derive(Clone)]
pub struct Level {
    tiles: TileArray,
    name: String,
}

impl Level {
    pub fn new() -> Self {
        Self {
            tiles: [Tile::Air; LEVEL_WIDTH*LEVEL_HEIGHT],
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

    pub fn break_tile(&mut self, index: usize) -> bool {
        self.tiles.get_mut(index).is_some_and(|t| t.hit())
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