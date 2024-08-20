use std::collections::HashMap;

use macroquad::{color::WHITE, math::{vec2, Rect, Vec2}, shapes::draw_rectangle_lines, texture::Texture2D, window::clear_background};

use crate::{game::world::BG_COL, gui::{Button, ButtonDetail, Gui, BUTTON_COL_HOVER, BUTTON_COL_IDLE, BUTTON_DETAIL_GREY, GRID_COL}, text_renderer::{render_text, TextAlign}, Scene, SceneChange};

pub struct MainMenu {
    gui: Gui,
}

impl MainMenu {
    pub fn new() -> Self {
        let mut buttons = HashMap::new();
        buttons.insert(0, Button::new(Rect::new(24.0, 70.0, 144.0, 10.0),  ButtonDetail::Text(String::from("         PLAY         ")), vec2(6.0, 2.0)));
        buttons.insert(1, Button::new(Rect::new(24.0, 90.0, 144.0, 10.0),  ButtonDetail::Text(String::from("EDIT CURRENT LEVEL PACK")), vec2(3.0, 2.0)));
        buttons.insert(2, Button::new(Rect::new(24.0, 110.0, 144.0, 10.0), ButtonDetail::Text(String::from("  EDIT NEW LEVEL PACK  ")), vec2(3.0, 2.0)));
        buttons.insert(3, Button::new(Rect::new(24.0, 130.0, 144.0, 10.0), ButtonDetail::Text(String::from("         INFO         ")), vec2(6.0, 2.0)));

        Self {
            gui: Gui::new(buttons),
        }
    }
}

impl Scene for MainMenu {
    fn update(&mut self, mouse_pos: Vec2) -> Option<SceneChange> {
        self.gui.update(mouse_pos, None);

        for (id, scene_change) in [
            (0, SceneChange::Game),
            (1, SceneChange::Editor { new: false }),
            (2, SceneChange::Editor { new: true }),
        ] {
            if self.gui.button(id).is_some_and(|b| b.released()) {
                return Some(scene_change);
            }
        }
        
        None
    }

    fn draw(&self, texture: &Texture2D, level_pack_info: Option<(&String, &String)>) {
        clear_background(BG_COL);

        // TODO: Logo
        render_text(&String::from("JUMBLEDFOX'S BREAKOUT"), vec2(0.0, 0.0), WHITE, TextAlign::Left, texture);

        // Level pack
        draw_rectangle_lines(46.0, 35.0, 100.0, 18.0, 2.0, GRID_COL);

        render_text(&String::from("LEVEL PACK LOADED:"), vec2(44.0, 26.0), WHITE, TextAlign::Left, texture);

        if let Some((name, author)) = level_pack_info {
            render_text(name, vec2(48.0, 37.0), WHITE, TextAlign::Left, texture);
            render_text(author, vec2(48.0, 45.0), BUTTON_DETAIL_GREY, TextAlign::Left, texture);
        } else {
            render_text(&String::from(" NO PACK LOADED! "), vec2(47.0, 41.0), BUTTON_DETAIL_GREY, TextAlign::Left, texture);
        }

        // Buttons
        for id in [0, 1, 2, 3] {
            let button = match self.gui.button(id) {
                Some(b) => b,
                None => continue,
            };
            let gray = (id == 0 || id == 1) && level_pack_info.is_none();
            button.draw(texture, if gray { BUTTON_DETAIL_GREY } else { WHITE }, if button.idle() || gray { BUTTON_COL_IDLE } else { BUTTON_COL_HOVER }, GRID_COL);
        }
    }
}