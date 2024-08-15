use macroquad::texture::Texture2D;

use crate::{game::{level::Level, Game}, Scene};

pub struct Editor {
    game: Option<Game>,
    level: Level,
}

impl Scene for Editor {
    fn new() -> Self {
        Editor {
            game: None,
            level: Level::new(),
        }
    }

    fn update(&mut self) {
        
    }

    fn draw(&self, texture: &Texture2D) {
        
    }
}