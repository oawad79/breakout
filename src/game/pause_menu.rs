use std::collections::HashMap;

use macroquad::{color::WHITE, math::{vec2, Rect, Vec2}, shapes::draw_rectangle, texture::Texture2D};

use crate::{gui::{Button, ButtonDetail, Gui, BUTTON_COL_HOVER, BUTTON_COL_IDLE, DARKEN_BACKGROUND, GRID_COL}, text_renderer::{render_text, TextAlign}};

use super::world::level::Level;

pub struct PauseMenu {
    gui: Gui,
    paused: bool,
}

impl PauseMenu {
    pub fn new() -> Self {
        let mut buttons = HashMap::new();

        buttons.insert(0, Button::new(Rect::new(55.0, 105.0, 39.0, 10.0), ButtonDetail::Text(String::from("RESUME")), vec2(2.0, 2.0)));
        buttons.insert(1, Button::new(Rect::new(98.0, 105.0, 39.0, 10.0), ButtonDetail::Text(String::from("EXIT")), vec2(8.0, 2.0)));

        PauseMenu {
            gui: Gui::new(buttons),
            paused: false,
        }
    }

    pub fn paused(&self) -> bool {
        self.paused
    }
    pub fn paused_mut(&mut self) -> &mut bool {
        &mut self.paused
    }

    pub fn button_exit(&self) -> bool {
        self.gui.button(1).is_some_and(|b| b.released())
    }

    pub fn update(&mut self, mouse_pos: Vec2) {
        if !self.paused {
            return;
        }

        self.gui.update(mouse_pos, None);

        if self.gui.button(0).is_some_and(|b| b.released()) {
            self.paused = false;
        }
    }

    pub fn draw(&self, texture: &Texture2D) {
        if !self.paused {
            return;
        }

        let view_size = Level::view_size();
        draw_rectangle(0.0, 0.0, view_size.x, view_size.y, DARKEN_BACKGROUND);
        
        render_text(&String::from("PAUSED"), vec2(79.0, 94.0), WHITE, TextAlign::Left, texture);
        for id in [0, 1] {
            let button = match self.gui.button(id) {
                Some(b) => b,
                None => continue,
            };
            button.draw(texture, WHITE, if button.idle() { BUTTON_COL_IDLE } else { BUTTON_COL_HOVER }, GRID_COL)
        }
    }
}