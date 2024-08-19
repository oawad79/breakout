use std::collections::HashMap;

use macroquad::{color::WHITE, input::{clear_input_queue, is_key_pressed, is_mouse_button_pressed, KeyCode, MouseButton}, math::{vec2, Rect, Vec2}, shapes::{draw_line, draw_rectangle, draw_rectangle_lines}, texture::{draw_texture_ex, DrawTextureParams, Texture2D}};

use crate::{game::world::{level::{Level, Tile, TILE_HEIGHT, TILE_WIDTH}, BG_COL}, gui::{Button, ButtonDetail, Gui, Id, TextField, BUTTON_COL_HOVER, BUTTON_COL_IDLE, BUTTON_DETAIL_GREY, BUTTON_DETAIL_HELP, DARKEN_BACKGROUND, GRID_COL}, text_renderer::{render_text, TextAlign}};

use super::editor_level_pack::EditorLevelPack;

const TILES_BUTTONS: &[Tile] = &[
    Tile::White,
    Tile::Red,
    Tile::Orange,
    Tile::Yellow,
    Tile::Green,
    Tile::Cyan,
    Tile::Blue,
    Tile::Purple,
    Tile::Pink,
    Tile::Brown,
    Tile::Black,
    Tile::Stone,
    Tile::StoneCracked,
    Tile::Metal,
    Tile::Gold,
    Tile::Air
];

const ARROW_TEXTURE: Rect = Rect { x: 157.0, y: 8.0, w: 7.0, h: 9.0 };
const UNDO_TEXTURE: Rect = Rect { x: 165.0, y: 8.0, w: 7.0, h: 6.0 };
const REDO_TEXTURE: Rect = Rect { x: 173.0, y: 8.0, w: 7.0, h: 6.0 };

const SHIFT_NEXT_TEXTURE: Rect = Rect { x: 187.0, y: 8.0, w: 6.0, h: 5.0 };
const SHIFT_PREV_TEXTURE: Rect = Rect { x: 187.0, y: 14.0, w: 6.0, h: 5.0 };
const LEVEL_NEXT_TEXTURE: Rect = Rect { x: 173.0, y: 15.0, w: 7.0, h: 5.0 };
const LEVEL_PREV_TEXTURE: Rect = Rect { x: 165.0, y: 15.0, w: 7.0, h: 5.0 };
const LEVEL_ADD_TEXTURE: Rect = Rect { x: 181.0, y: 8.0, w: 5.0, h: 5.0 };

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Confirmation {
    None,
    LevelDelete,
    Exit,
    Save,
}

pub struct EditorGui {
    gui: Gui,

    confirmation: Confirmation,
    confirmation_popup: Confirmation,

    text_fields: HashMap<Id, TextField>,
    active_text_field: Option<Id>,
    text_field_flash: f32,
}

impl EditorGui {
    pub fn new() -> Self {
        let view_size = Level::view_size();
        let mut buttons = HashMap::new();

        for (i, tile) in TILES_BUTTONS.iter().enumerate() {
            let rect = Rect {
                x: i as f32 * (TILE_WIDTH + 1.0),
                y: view_size.y - (TILE_HEIGHT + 1.0),
                w: TILE_WIDTH + 1.0,
                h: TILE_HEIGHT + 1.0,
            };
            buttons.insert(i as Id, Button::new(rect, ButtonDetail::Icon(tile.texture_rect()), vec2(0.0, 0.0)));
        }
        buttons.insert(100, Button::new(Rect::new(view_size.x - 97.0, 0.0, 97.0, 7.0), ButtonDetail::None, vec2(0.0, 0.0))); // Name
        let mut text_fields = HashMap::new();
        text_fields.insert(100, TextField::new(vec2(view_size.x - 97.0, 0.0)));

        buttons.insert(101, Button::new(Rect::new(view_size.x - 28.0, view_size.y - 29.0, 27.0, 8.0), ButtonDetail::Text(String::from("EXIT")), vec2(2.0, 1.0)));
        buttons.insert(102, Button::new(Rect::new(view_size.x - 56.0, view_size.y - 29.0, 27.0, 8.0), ButtonDetail::Text(String::from("SAVE")), vec2(2.0, 1.0)));
        buttons.insert(103, Button::new(Rect::new(view_size.x - 90.0, view_size.y - 29.0, 33.0, 8.0), ButtonDetail::Text(String::from("CLEAR")), vec2(2.0, 1.0)));
        buttons.insert(104, Button::new(Rect::new(view_size.x - 20.0, view_size.y - 38.0,  9.0, 8.0), ButtonDetail::Icon(UNDO_TEXTURE), vec2(1.0, 1.0)));
        buttons.insert(105, Button::new(Rect::new(view_size.x - 10.0, view_size.y - 38.0,  9.0, 8.0), ButtonDetail::Icon(REDO_TEXTURE), vec2(1.0, 1.0)));
        buttons.insert(106, Button::new(Rect::new(view_size.x - 60.0, view_size.y - 38.0, 39.0, 8.0), ButtonDetail::Text(String::from("HELP??")), vec2(2.0, 1.0)));

        buttons.insert(200, Button::new(Rect::new(87.0, view_size.y - 37.0, 9.0, 7.0), ButtonDetail::Icon(LEVEL_ADD_TEXTURE), vec2(2.0, 1.0)));
        buttons.insert(201, Button::new(Rect::new(77.0, view_size.y - 37.0, 9.0, 7.0), ButtonDetail::Icon(LEVEL_NEXT_TEXTURE), vec2(1.0, 1.0)));
        buttons.insert(202, Button::new(Rect::new(67.0, view_size.y - 37.0, 9.0, 7.0), ButtonDetail::Icon(LEVEL_PREV_TEXTURE), vec2(1.0, 1.0)));
        buttons.insert(203, Button::new(Rect::new(44.0, view_size.y - 37.0, 9.0, 7.0), ButtonDetail::Icon(SHIFT_NEXT_TEXTURE), vec2(2.0, 1.0)));
        buttons.insert(204, Button::new(Rect::new(34.0, view_size.y - 37.0, 9.0, 7.0), ButtonDetail::Icon(SHIFT_PREV_TEXTURE), vec2(1.0, 1.0)));
        buttons.insert(205, Button::new(Rect::new(1.0, view_size.y - 29.0, 39.0, 8.0), ButtonDetail::Text(String::from("DELETE")), vec2(2.0, 1.0)));

        Self {
            gui: Gui::new(buttons),
            confirmation: Confirmation::None,
            confirmation_popup: Confirmation::None,
            
            text_fields,
            active_text_field: None,
            text_field_flash: 0.0,
        }
    }

    pub fn mouse_clicked(&self) -> bool {
        is_mouse_button_pressed(MouseButton::Left) || is_mouse_button_pressed(MouseButton::Right)
    }

    fn button_released(&self, id: Id) -> bool {
        self.gui.button(id).is_some_and(|b| b.released())
    }

    pub fn popup_open(&self) -> bool {
        self.confirmation_popup != Confirmation::None
    }

    pub fn button_exit(&self) -> bool {
        self.confirmation == Confirmation::Exit
    }
    pub fn button_save(&self) -> bool {
        self.confirmation == Confirmation::Save
    }
    pub fn button_clear(&self) -> bool {
        self.button_released(103)
    }
    pub fn button_undo(&self) -> bool {
        self.button_released(104)
    }
    pub fn button_redo(&self) -> bool {
        self.button_released(105)
    }
    pub fn button_help(&self) -> bool {
        self.button_released(106)
    }

    pub fn button_level_add(&self) -> bool {
        self.button_released(200)
    }
    pub fn button_level_next(&self) -> bool {
        self.button_released(201)
    }
    pub fn button_level_prev(&self) -> bool {
        self.button_released(202)
    }
    pub fn button_level_shift_next(&self) -> bool {
        self.button_released(203)
    }
    pub fn button_level_shift_prev(&self) -> bool {
        self.button_released(204)
    }
    pub fn button_level_delete(&self) -> bool {
        self.confirmation == Confirmation::LevelDelete
    }

    pub fn stop_editing_name(&mut self) {
        if self.active_text_field.is_some_and(|id| id == 100) {
            self.active_text_field = None;
        }
    }

    pub fn update(&mut self, mouse_pos: Vec2, level_pack: &mut EditorLevelPack, draw_type: &mut Tile) {
        self.confirmation = Confirmation::None;

        let update_only: Option<&[Id]> = match self.confirmation_popup {
            Confirmation::None => None,
            _ => Some(&[300, 301, 302, 303]),
        };
        self.gui.update(mouse_pos, update_only);

        // Confirmation popup
        let prev_confirmation_popup = self.confirmation_popup;
        self.confirmation_popup = match (self.button_released(101), self.button_released(102), self.button_released(205)) {
            (true, _, _) => Confirmation::Exit,
            (_, true, _) => Confirmation::Save,
            (_, _, true) => Confirmation::LevelDelete,
            _ => self.confirmation_popup,
        };

        // A new popup!
        if self.confirmation_popup != Confirmation::None && prev_confirmation_popup == Confirmation::None {
            self.gui.buttons_mut().insert(300, Button::new(Rect::new(70.0, 95.0, 21.0, 8.0), ButtonDetail::Text(String::from("YES")), vec2(2.0, 1.0)));
            self.gui.buttons_mut().insert(301, Button::new(Rect::new(100.0, 95.0, 21.0, 8.0), ButtonDetail::Text(String::from("NO")), vec2(5.0, 1.0)));

            if self.confirmation_popup == Confirmation::Save {
                clear_input_queue();
                self.gui.buttons_mut().insert(302, Button::new(Rect::new(70.0, 75.0, 97.0, 7.0), ButtonDetail::None, vec2(0.0, 0.0))); // Pack name
                self.gui.buttons_mut().insert(303, Button::new(Rect::new(70.0, 85.0, 97.0, 7.0), ButtonDetail::None, vec2(0.0, 0.0))); // Pack author
                self.text_fields.insert(302, TextField::new(vec2(70.0, 75.0)));
                self.text_fields.insert(303, TextField::new(vec2(70.0, 85.0)));
                self.active_text_field = Some(302);
            }
        }
        // Updating the popup...
        if self.confirmation_popup != Confirmation::None {
            let (yes, no) = (self.button_released(300), self.button_released(301));
            if yes {
                self.confirmation = self.confirmation_popup;
            }
            // Close the popup (Don't close it if it's 'Exit' and yes to avoid the single frame of flicker!!)
            if (yes && self.confirmation != Confirmation::Exit) || no {
                self.gui.buttons_mut().remove(&300);
                self.gui.buttons_mut().remove(&301);
                self.gui.buttons_mut().remove(&302);
                self.gui.buttons_mut().remove(&303);
                self.text_fields.remove(&302);
                self.text_fields.remove(&303);
                self.confirmation_popup = Confirmation::None;
            }
        }
        
        // Text fields
        let mut hovering_any = false; 
        for id in [100, 302, 303] {
            if self.gui.button(id).is_some_and(|b| b.rect().contains(mouse_pos)) {
                hovering_any = true;
            }
            if self.button_released(id) {
                clear_input_queue();
                self.active_text_field = match self.active_text_field {
                    Some(i) if i == id => None,
                    _ => Some(id)
                };
                self.text_field_flash = 0.0;
            }
        }
        if !hovering_any && self.mouse_clicked() {
            self.active_text_field = None;
        }
        if let Some(id) = self.active_text_field {
            let exit = if let Some(text_field) = self.text_fields.get_mut(&id) {
                let update_text = if id == 100 { level_pack.level_mut().name_mut() } else if id == 302 { level_pack.name_mut() } else { level_pack.author_mut() };
                text_field.update(update_text)
            } else { false };
            if exit {
                self.active_text_field = None;
            }

            if is_key_pressed(KeyCode::Tab) {
                if id == 302 { self.active_text_field = Some(303) }
                if id == 303 { self.active_text_field = Some(302) }
            }
        }
        self.text_field_flash = (self.text_field_flash + macroquad::time::get_frame_time()) % 0.4;

        // Selecting tile
        for (id, tile) in TILES_BUTTONS.iter().enumerate() {
            if self.button_released(id as Id) {
                *draw_type = *tile;
            }
        }
    }

    pub fn draw(&self, texture: &Texture2D, level_pack: &EditorLevelPack, draw_type: &Tile) {
        let level = level_pack.level();
        let timewarp = level_pack.timewarp();

        let view_size = Level::view_size();
        draw_line(0.0, view_size.y -  8.5, view_size.x, view_size.y -  8.5, 1.0, GRID_COL);
        draw_line(0.0, view_size.y - 20.5, view_size.x, view_size.y - 20.5, 1.0, GRID_COL);

        // Tile buttons
        for (id, tile) in TILES_BUTTONS.iter().enumerate() {
            let button = match self.gui.button(id as Id) {
                Some(b) => b,
                None => continue,
            };
            let detail = match button.detail() {
                &ButtonDetail::Icon(r) => r,
                _ => continue,
            };
            let detail_pos = button.detail_pos();
            let rect = button.rect();
            let y_offset = match button.idle() {
                true => 0.0,
                false => -1.0,
            };
            draw_texture_ex(texture, rect.x + detail_pos.x, rect.y + detail_pos.y + y_offset, WHITE, DrawTextureParams {
                source: Some(detail),
                ..Default::default()
            });
            // Arrow
            if *draw_type == *tile {
                draw_texture_ex(texture, rect.x + (rect.w - 1.0 - 7.0) / 2.0, rect.y - 9.0 - 3.0, WHITE, DrawTextureParams {
                    source: Some(ARROW_TEXTURE),
                    ..Default::default()
                });
            }
        }

        let field_flash = self.text_field_flash <= 0.2;

        if let Some(name_field) = self.text_fields.get(&100) {
            name_field.draw(texture, level.name(), &String::from("NAME:"), field_flash && self.active_text_field == Some(100), BUTTON_COL_HOVER, BUTTON_DETAIL_GREY);
        }

        // Other buttons
        for id in [
            101, 102, 103, 104, 105, 106, 200, 201, 202, 203, 204, 205,
        ] {
            let button = match self.gui.button(id) {
                Some(b) => b,
                None => continue,
            };

            let grey = (id == 104 && !timewarp.can_undo())
            || (id == 105 && !timewarp.can_redo())
            || (id == 200 && !level_pack.can_add())
            || (id == 201 && !level_pack.can_next())
            || (id == 202 && !level_pack.can_prev())
            || (id == 203 && !level_pack.can_shift_next())
            || (id == 204 && !level_pack.can_shift_prev());

            let bg_col = if button.idle() { BUTTON_COL_IDLE } else { BUTTON_COL_HOVER };
            let detail_col = if grey { BUTTON_DETAIL_GREY } else if id == 106 { BUTTON_DETAIL_HELP } else { WHITE };

            button.draw(texture, detail_col, bg_col, GRID_COL);
        }


        render_text(&String::from("SHIFT"), vec2(2.0, view_size.y - 37.0), BUTTON_DETAIL_GREY, TextAlign::Left, texture);
        render_text(&format!("LVL {:0>2}/{:0>2}", level_pack.current() + 1, level_pack.level_count()), vec2(43.0, view_size.y - 28.0), WHITE, TextAlign::Left, texture);

        render_text(&String::from("EDITOR"), vec2(0.0, 0.0), WHITE, TextAlign::Left, texture);
        render_text(&String::from("PRESS ESC TO PLAY LEVEL."), vec2(0.0, 7.0), WHITE, TextAlign::Left, texture);


        if self.confirmation_popup != Confirmation::None {
            draw_rectangle(0.0, 0.0, view_size.x, view_size.y, DARKEN_BACKGROUND);

            let rect = match self.confirmation_popup {
                Confirmation::Save => Rect::new(26.0, 63.0, 143.0, 43.0),
                _ => Rect::new(49.0, 68.0, 93.0, 38.0),
            };

            draw_rectangle_lines(rect.x-1.0, rect.y-1.0, rect.w+2.0, rect.h+2.0, 2.0, GRID_COL);
            draw_rectangle(rect.x, rect.y, rect.w, rect.h, BG_COL);

            if self.confirmation_popup == Confirmation::Exit {
                render_text(&String::from("     EXIT?     "), vec2(51.0, 70.0), WHITE, TextAlign::Left, texture);
                render_text(&String::from("UNSAVED CHANGES"), vec2(51.0, 78.0), WHITE, TextAlign::Left, texture);
                render_text(&String::from(" WILL BE LOST! "), vec2(51.0, 86.0), WHITE, TextAlign::Left, texture);
            }
            if self.confirmation_popup == Confirmation::LevelDelete {
                render_text(&String::from(" DELETE LEVEL? "), vec2(51.0, 70.0), WHITE, TextAlign::Left, texture);
                render_text(&String::from("  THIS CANNOT  "), vec2(51.0, 78.0), WHITE, TextAlign::Left, texture);
                render_text(&String::from("   BE UNDONE   "), vec2(51.0, 86.0), WHITE, TextAlign::Left, texture);
            }
            if self.confirmation_popup == Confirmation::Save {
                render_text(&String::from("SAVE LEVEL PACK"), vec2(51.0, 65.0), WHITE, TextAlign::Left, texture);
            }


            for id in [300, 301] {
                self.gui.button(id).map(|b| b.draw(texture, WHITE, if b.hovered() { BUTTON_COL_HOVER } else { BUTTON_COL_IDLE }, GRID_COL));
            }
            for (id, text, name) in [
                (302, level_pack.name(), String::from("NAME:")),
                (303, level_pack.author(), String::from("AUTHOR:")),
            ] {
                let field = match self.text_fields.get(&id) {
                    Some(f) => f,
                    None => continue
                };
                field.draw(texture, text, &name, field_flash && self.active_text_field == Some(id), BUTTON_COL_HOVER, BUTTON_DETAIL_GREY);
            }
        }
    }
}