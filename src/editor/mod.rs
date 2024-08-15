use macroquad::{color::WHITE, input::{is_key_pressed, KeyCode}, math::vec2, texture::Texture2D};

use crate::{game::{level::Level, Game}, text_renderer::{render_text, TextAlign}, Scene};

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
        if let Some(game) = &mut self.game {
            game.update();
            return;
        }

        if is_key_pressed(KeyCode::Escape) {
            self.game = Some(Game::new(self.level.clone(), None, None));
        }
    }

    fn draw(&self, texture: &Texture2D) {
        if let Some(game) = &self.game {
            game.draw(texture);
            return;
        }

        render_text(&String::from("OMG WELCOME TO THE EDITOR :D :D"), vec2(0.0, 0.0), WHITE, TextAlign::Left, texture);
        render_text(&String::from("GOOD LUCK..."), vec2(0.0, 8.0), WHITE, TextAlign::Left, texture);
    }
}