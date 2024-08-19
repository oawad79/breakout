use std::collections::HashMap;

use macroquad::{color::{Color, WHITE}, color_u8, input::{get_char_pressed, is_mouse_button_pressed, is_mouse_button_released, MouseButton}, math::{vec2, Rect, Vec2}, shapes::{draw_rectangle, draw_rectangle_lines}, texture::{draw_texture_ex, DrawTextureParams, Texture2D}};

use crate::{game::world::level::LEVEL_NAME_LEN, text_renderer::{char_valid, render_text, TextAlign}};

pub const GRID_COL: Color = color_u8!(18, 78, 137, 255);
pub const BUTTON_COL_IDLE: Color = color_u8!(25, 31, 58, 255);
pub const BUTTON_COL_HOVER: Color = color_u8!(23, 56, 96, 255);
pub const BUTTON_DETAIL_GREY: Color = color_u8!(139, 155, 180, 255);
pub const BUTTON_DETAIL_HELP: Color = color_u8!(254, 231, 97, 255);
pub const DARKEN_BACKGROUND: Color = color_u8!(0, 0, 0, 128);

#[derive(PartialEq, Eq)]
pub enum ButtonState {
    Idle, Hovered, Released,
}

pub enum ButtonDetail {
    None,
    Text(String),
    Icon(Rect),
}

pub struct Button {
    rect: Rect,
    state: ButtonState,
    detail: ButtonDetail,
    detail_pos: Vec2,
}

impl Button {
    pub fn new(rect: Rect, detail: ButtonDetail, detail_pos: Vec2) -> Self {
        Self {
            rect,
            state: ButtonState::Idle,
            detail,
            detail_pos,
        }
    }
    pub fn rect(&self) -> Rect {
        self.rect
    }
    pub fn detail(&self) -> &ButtonDetail {
        &self.detail
    }
    pub fn detail_pos(&self) -> Vec2 {
        self.detail_pos
    }
    pub fn idle(&self) -> bool {
        self.state == ButtonState::Idle
    }
    pub fn hovered(&self) -> bool {
        self.state == ButtonState::Hovered
    }
    pub fn released(&self) -> bool {
        self.state == ButtonState::Released
    }

    pub fn draw(&self, texture: &Texture2D, detail_col: Color, bg_col: Color, outline_col: Color) {
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, bg_col);
        draw_rectangle_lines(self.rect.x-1.0, self.rect.y-1.0, self.rect.w+2.0, self.rect.h+2.0, 2.0, outline_col);

        match &self.detail {
            ButtonDetail::Text(s) => render_text(s, self.rect.point() + self.detail_pos, detail_col, TextAlign::Left, texture),
            ButtonDetail::Icon(r) => draw_texture_ex(texture, self.rect.x + self.detail_pos.x, self.rect.y + self.detail_pos.y, detail_col, DrawTextureParams {
                source: Some(*r),
                ..Default::default()
            }),
            ButtonDetail::None => {}
        };
    }
}

pub struct TextField {
    rect: Rect,
}

impl TextField {
    pub fn new(pos: Vec2) -> Self {
        Self {
            rect: Rect::new(pos.x, pos.y, 97.0, 7.0),
        }
    }

    pub fn rect(&self) -> Rect {
        self.rect
    }
    pub fn update(&mut self, text: &mut String) -> bool {
        if let Some(c) = get_char_pressed() {
            let c = c.to_ascii_uppercase();
            if char_valid(c) && text.len() < LEVEL_NAME_LEN {
                text.push(c);
            }
            if c == '\u{8}' && text.len() > 0 {
                let new_len = text.len() - 1;
                text.truncate(new_len);
            }
            if c == '\r' {
                return true;
            }
        }
        return false;
    }

    pub fn draw(&self, texture: &Texture2D, text: &String, name: &String, flash: bool, flash_col: Color, grey_col: Color) {
        let rect = self.rect;
        let underscore_amount = LEVEL_NAME_LEN.saturating_sub(text.len());
        if flash {
            draw_rectangle(rect.x, rect.y, rect.w, rect.h, flash_col);
        }
        render_text(name, rect.point(), WHITE, TextAlign::Right, texture);
        render_text(text, rect.point() + vec2(rect.w, 0.0), WHITE, TextAlign::Right, texture);
        render_text(&"_".repeat(underscore_amount), rect.point(), grey_col, TextAlign::Left, texture);
    }
}

pub type Id = u64;

pub struct Gui {
    buttons: HashMap<Id, Button>,

    hot_item: Option<Id>,
    active_item: Option<Id>,
}

impl Gui {
    pub fn new(buttons: HashMap<Id, Button>) -> Self {
        Self {
            buttons,
            hot_item: None,
            active_item: None,
        }
    }

    pub fn update(&mut self, mouse_pos: Vec2, update_only: Option<&[Id]>) {
        self.hot_item = None;

        for (id, button) in self.buttons.iter_mut() {
            button.state = ButtonState::Idle;

            if update_only.is_some_and(|u| !u.contains(id)) {
                if self.active_item == Some(*id) {
                    self.active_item = None;
                }
                continue;
            }

            if button.rect.contains(mouse_pos) && self.hot_item == None {
                button.state = ButtonState::Hovered;
                self.hot_item = Some(*id);
            }
            if is_mouse_button_pressed(MouseButton::Left) && self.hot_item == Some(*id) {
                self.active_item = Some(*id);
            }
            if is_mouse_button_released(MouseButton::Left) && self.hot_item == Some(*id) && self.active_item == Some(*id) {
                button.state = ButtonState::Released;
            }
        }
    }

    pub fn button(&self, id: Id) -> Option<&Button> {
        self.buttons.get(&id)
    }

    pub fn buttons_mut(&mut self) -> &mut HashMap<Id, Button> {
        &mut self.buttons
    }
}