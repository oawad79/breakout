use std::collections::HashMap;

use macroquad::{input::{is_mouse_button_pressed, is_mouse_button_released, MouseButton}, math::{Rect, Vec2}};

#[derive(PartialEq, Eq)]
pub enum ButtonState {
    Idle, Hovered, Released,
}

pub struct Button {
    rect: Rect,
    state: ButtonState, 
}

impl Button {
    pub fn new(rect: Rect) -> Self {
        Self { rect, state: ButtonState::Idle }
    }
    pub fn rect(&self) -> Rect {
        self.rect
    }
    pub fn hovered(&self) -> bool {
        self.state == ButtonState::Hovered
    }
    pub fn released(&self) -> bool {
        self.state == ButtonState::Released
    }
    pub fn default() -> Self {
        Self { rect: Rect::default(), state: ButtonState::Idle }
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

    pub fn update(&mut self, mouse_pos: Vec2) {
        self.hot_item = None;

        for (id, button) in self.buttons.iter_mut() {
            button.state = ButtonState::Idle;

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
}