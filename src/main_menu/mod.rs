use std::collections::HashMap;

use macroquad::{color::WHITE, math::{vec2, Rect, Vec2}, texture::Texture2D, window::clear_background};

use crate::{game::{level_pack::LevelPack, world::BG_COL}, gui::{Button, ButtonDetail, Gui, BUTTON_COL_HOVER, BUTTON_COL_IDLE, GRID_COL}, text_renderer::{render_text, TextAlign}, Scene, SceneChange};

pub struct MainMenu {
    gui: Gui,

    level_pack_name_author: Option<(String, String)>,
}

impl MainMenu {
    pub fn new() -> Self {
        let mut buttons = HashMap::new();
        buttons.insert(0, Button::new(Rect::new(24.0, 70.0, 144.0, 10.0),  ButtonDetail::Text(String::from("         PLAY         ")), vec2(6.0, 2.0)));
        buttons.insert(1, Button::new(Rect::new(24.0, 90.0, 144.0, 10.0),  ButtonDetail::Text(String::from("  EDIT NEW LEVEL PACK  ")), vec2(3.0, 2.0)));
        buttons.insert(2, Button::new(Rect::new(24.0, 110.0, 144.0, 10.0), ButtonDetail::Text(String::from("EDIT CURRENT LEVEL PACK")), vec2(3.0, 2.0)));
        buttons.insert(3, Button::new(Rect::new(24.0, 130.0, 144.0, 10.0), ButtonDetail::Text(String::from("         INFO         ")), vec2(6.0, 2.0)));

        Self {
            gui: Gui::new(buttons),
            level_pack_name_author: None,
        }
    }
}

impl Scene for MainMenu {
    fn update(&mut self, mouse_pos: Vec2, level_pack: &Option<LevelPack>) -> Option<SceneChange> {
        self.level_pack_name_author = match level_pack {
            Some(lp) => Some((lp.name().clone(), lp.author().clone())),
            _ => None
        };

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

    fn draw(&self, texture: &Texture2D) {
        clear_background(BG_COL);

        render_text(&String::from("JUMBLEDFOX'S BREAKOUT"), vec2(0.0, 0.0), WHITE, TextAlign::Left, texture);

        for id in [0, 1, 2, 3] {
            let button = match self.gui.button(id) {
                Some(b) => b,
                None => continue,
            };
            button.draw(texture, WHITE, if button.idle() { BUTTON_COL_IDLE } else { BUTTON_COL_HOVER }, GRID_COL);
        }
    }
}