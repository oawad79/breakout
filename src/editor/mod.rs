use std::collections::{HashMap, VecDeque};

use macroquad::{camera::Camera2D, color::{Color, WHITE}, input::{clear_input_queue, get_char_pressed, is_key_pressed, is_mouse_button_pressed, is_mouse_button_released, mouse_position, KeyCode, MouseButton}, math::{vec2, Rect}, shapes::{draw_line, draw_rectangle, draw_rectangle_lines}, texture::{draw_texture_ex, DrawTextureParams, Texture2D}, window::clear_background};

use crate::{game::{level::{Level, Tile, TileArray, LEVEL_HEIGHT, LEVEL_HEIGHT_PADDING_TOP, LEVEL_NAME_LEN, LEVEL_WIDTH, TILE_GAP, TILE_HEIGHT, TILE_WIDTH}, Game, Lives}, gui::{Button, Gui, Id}, text_renderer::{char_valid, render_text, TextAlign}};

pub mod timewarp;

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

const ARROW_TEXTURE: Rect = Rect { x: 158.0, y: 8.0, w: 7.0, h: 9.0 };
const UNDO_TEXTURE: Rect = Rect { x: 166.0, y: 8.0, w: 7.0, h: 6.0 };

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum ClickAction {
    None, Draw, Erase,
}

pub struct Editor {
    gui: Gui,
    ids_begin: Id,

    draw_type: Tile,
    click_action: ClickAction,
    editing_name: bool,
    flashing_timer: f32,

    level: Level,
    previous_state: TileArray,
    undo_states: VecDeque<TileArray>,
    redo_states: Vec<TileArray>,

    game: Option<Game>,
    paddle_pos: Option<f32>,
}

impl Editor {
    pub fn new() -> Self {
        let view_size = Level::view_size();

        let mut buttons = HashMap::new();

        for (i, _) in TILES_BUTTONS.iter().enumerate() {
            let rect = Rect {
                x: i as f32 * (TILE_WIDTH + 1.0),
                y: view_size.y - (TILE_HEIGHT + 1.0),
                w: TILE_WIDTH + 1.0,
                h: TILE_HEIGHT + 1.0,
            };
            buttons.insert(i, Button::new(rect));
        }
        let ids_begin = TILES_BUTTONS.len();

        buttons.insert(ids_begin,   Button::new(Rect::new(view_size.x - 47.0, view_size.y - 29.0, 19.0, 8.0))); // New
        buttons.insert(ids_begin+1, Button::new(Rect::new(view_size.x - 27.0, view_size.y - 29.0, 27.0, 8.0))); // Save
        buttons.insert(ids_begin+2, Button::new(Rect::new(view_size.x - 19.0, view_size.y - 38.0,  9.0, 8.0))); // Undo
        buttons.insert(ids_begin+3, Button::new(Rect::new(view_size.x -  9.0, view_size.y - 38.0,  9.0, 8.0))); // Redo

        buttons.insert(ids_begin+4, Button::new(Rect::new(view_size.x - 97.0, 0.0, 97.0, 7.0))); // Name

        Editor {
            gui: Gui::new(buttons),
            ids_begin,
            editing_name: false,
            flashing_timer: 0.0,

            draw_type: Tile::Red,
            click_action: ClickAction::None,
            
            level: Level::new(),
            previous_state: [Tile::Air; LEVEL_WIDTH*LEVEL_HEIGHT],
            undo_states: VecDeque::with_capacity(50),
            redo_states: Vec::with_capacity(50),

            game: None,
            paddle_pos: None,
        }
    }

    pub fn update(&mut self, camera: &Camera2D) {
        if is_key_pressed(KeyCode::Escape) {
            if self.game.is_some() {
                self.paddle_pos = self.game.as_ref().map(|g| g.paddle_pos());
                self.game = None;
            } else {
                self.game = Some(Game::new(self.level.clone(), self.paddle_pos, Lives::Infinite));
            }
            clear_input_queue();
        }
        if let Some(game) = &mut self.game {
            game.update();
            return;
        }

        // Gui
        let mouse_pos = camera.screen_to_world(vec2(mouse_position().0, mouse_position().1));
        self.gui.update(mouse_pos);

        // Selecting a tile type
        for (id, tile) in TILES_BUTTONS.iter().enumerate() {
            if self.gui.button(id).is_some_and(|b| b.released()) {
                self.draw_type = *tile;
            }
        }
        // New
        if self.gui.button(self.ids_begin).is_some_and(|b| b.released()) {
            if self.previous_state != *self.level.tiles() {
                self.previous_state = self.level.tiles().clone();
                self.undo_states.push_front(self.previous_state);
                self.redo_states.clear();
            }
            self.level = Level::new();
        }
        // Editing name
        if self.editing_name && self.gui.button(self.ids_begin+4).is_some_and(|b| !b.hovered()) && is_mouse_button_pressed(MouseButton::Left) || is_mouse_button_pressed(MouseButton::Right) {
            self.editing_name = false;
        } else if self.gui.button(self.ids_begin+4).is_some_and(|b| b.released()) {
            self.editing_name = !self.editing_name;
            self.flashing_timer = 0.0;
            clear_input_queue();
        }
        self.flashing_timer = (self.flashing_timer + macroquad::time::get_frame_time()) % 0.4;

        if self.editing_name {
            if let Some(c) = get_char_pressed() {
                let c = c.to_ascii_uppercase();
                if char_valid(c) && self.level.name().len() < LEVEL_NAME_LEN {
                    self.level.name_mut().push(c);
                }
                if c == '\u{8}' && self.level.name().len() > 0 {
                    let new_len = self.level.name().len() - 1;
                    self.level.name_mut().truncate(new_len);
                }
                if c == '\r' {
                    self.editing_name = false;
                }
            }
        }

        // Undo / Redo
        if self.gui.button(self.ids_begin+2).is_some_and(|b| b.released()) {
            if let Some(undo_state) = self.undo_states.pop_front() {
                self.redo_states.push(self.level.tiles().clone());
                *self.level.tiles_mut() = undo_state;
            }
        }
        if self.gui.button(self.ids_begin+3).is_some_and(|b| b.released()) {
            if let Some(redo_state) = self.redo_states.pop() {
                self.undo_states.push_front(self.level.tiles().clone());
                *self.level.tiles_mut() = redo_state;
            }
        }

        // Editing tiles
        let level_area_rect = Rect::new(0.0, LEVEL_HEIGHT_PADDING_TOP as f32 * (TILE_HEIGHT + TILE_GAP), LEVEL_WIDTH as f32 * (TILE_WIDTH + TILE_GAP), LEVEL_HEIGHT as f32 * (TILE_HEIGHT + TILE_GAP));

        if level_area_rect.contains(mouse_pos) {
            let hovered_tile_pos = ((mouse_pos - vec2(0.0, LEVEL_HEIGHT_PADDING_TOP as f32 * (TILE_HEIGHT * TILE_GAP) + TILE_GAP)) / (vec2(TILE_WIDTH, TILE_HEIGHT) + TILE_GAP)).floor();
            let hovered_tile_index = match hovered_tile_pos.x >= 0.0 && hovered_tile_pos.x < LEVEL_WIDTH as f32 && hovered_tile_pos.y >= 0.0 && hovered_tile_pos.y < LEVEL_HEIGHT as f32 {
                false => None,
                true => Some(hovered_tile_pos.y as usize * LEVEL_WIDTH + hovered_tile_pos.x as usize),
            };
    
            for (mouse_button, click_action) in [
                (MouseButton::Left, ClickAction::Draw),
                (MouseButton::Right, ClickAction::Erase),
            ] {
                if is_mouse_button_pressed(mouse_button) {
                    self.click_action = click_action;
                    self.previous_state = self.level.tiles().clone();
                }
                if is_mouse_button_released(mouse_button) && self.click_action == click_action {
                    self.click_action = ClickAction::None;
                    if self.previous_state != *self.level.tiles() {
                        self.undo_states.push_front(self.previous_state);
                        self.redo_states.clear();
                    }
                }
            }
            
            if let Some(tile_index) = hovered_tile_index {
                let tile_type = match self.click_action {
                    ClickAction::Draw  => Some(self.draw_type),
                    ClickAction::Erase => Some(Tile::Air),
                    _ => None
                };
                if let Some(tile_type) = tile_type {
                    self.level.tiles_mut().get_mut(tile_index).map(|t| *t = tile_type);
                }
            }
        }
    }

    pub fn draw(&self, texture: &Texture2D) {
        if let Some(game) = &self.game {
            game.draw(texture);
            render_text(&String::from("PRESS ESC TO RETURN TO EDITOR."), vec2(0.0, 7.0), WHITE, TextAlign::Left, texture);
            return;
        }

        clear_background(Color::from_rgba(25, 31, 58, 255));

        let grid_col = Color::from_hex(0x124E89);
        let grey_col = Color::from_rgba(139, 155, 180, 255);
        let view_size = Level::view_size();

        // Grid
        for x in 1..=LEVEL_WIDTH {
            let x = x as f32 * (TILE_WIDTH + TILE_GAP) - 0.5;
            draw_line(x, LEVEL_HEIGHT_PADDING_TOP as f32 * (TILE_HEIGHT + TILE_GAP), x, (LEVEL_HEIGHT_PADDING_TOP + LEVEL_HEIGHT) as f32  * (TILE_HEIGHT + TILE_GAP), 1.0, grid_col);
        }
        for y in 0..=LEVEL_HEIGHT {
            let y = (y + LEVEL_HEIGHT_PADDING_TOP) as f32 * (TILE_HEIGHT + TILE_GAP) - 0.5;
            draw_line(0.0, y, view_size.x, y, 1.0, grid_col);
        }
        self.level.draw(texture);

        // Gui
        draw_line(0.0, view_size.y -  8.5, view_size.x, view_size.y -  8.5, 1.0, grid_col);
        draw_line(0.0, view_size.y - 20.5, view_size.x, view_size.y - 20.5, 1.0, grid_col);

        // Tile buttons
        for (id, tile) in TILES_BUTTONS.iter().enumerate() {
            let button = match self.gui.button(id) {
                Some(b) => b,
                None => continue,
            };
            let rect = button.rect();
            let y_offset = match button.hovered() || button.released() {
                false => 0.0,
                true => -1.0,
            };
            draw_texture_ex(texture, rect.x, rect.y + y_offset, WHITE, DrawTextureParams {
                source: Some(tile.texture_rect()),
                ..Default::default()
            });
            // Arrow
            if self.draw_type == *tile {
                draw_texture_ex(texture, rect.x + (rect.w - 1.0 - 7.0) / 2.0, rect.y - 9.0 - 3.0, WHITE, DrawTextureParams {
                    source: Some(ARROW_TEXTURE),
                    ..Default::default()
                });
            }
        }

        let button_bg_idle = Color::from_rgba(0, 0, 0, 0);
        let button_bg_hover = Color::from_rgba(23, 56, 96, 255);
        for i in 0..=3 {
            let button = match self.gui.button(self.ids_begin + i) {
                Some(b) => b,
                None => continue,
            };
            let rect = button.rect();
            draw_rectangle(rect.x, rect.y, rect.w, rect.h, if button.hovered() { button_bg_hover } else { button_bg_idle });
            draw_rectangle_lines(rect.x-1.0, rect.y-1.0, rect.w+2.0, rect.h+2.0, 2.0, grid_col);

            if i == 0 || i == 1 {
                let (text, x_pad) = if i == 0 { ("NEW", 1.0) } else { ("SAVE", 2.0) };
                render_text(&text.to_owned(), rect.point()+vec2(x_pad, 1.0), WHITE, TextAlign::Left, texture);
            }
            if i == 2 || i == 3 {
                let grey = i == 2 && self.undo_states.is_empty() || i == 3 && self.redo_states.is_empty();
                draw_texture_ex(texture, rect.x + 1.0, rect.y + 1.0, if grey {grey_col} else {WHITE}, DrawTextureParams {
                    source: Some(UNDO_TEXTURE),
                    flip_x: i == 3,
                    ..Default::default()
                });
            }
        }

        render_text(&String::from("EDITOR"), vec2(0.0, 0.0), WHITE, TextAlign::Left, texture);
        render_text(&String::from("PRESS ESC TO PLAY LEVEL."), vec2(0.0, 7.0), WHITE, TextAlign::Left, texture);

        if self.flashing_timer <= 0.2 {
            if self.editing_name {
                if let Some(button) = self.gui.button(self.ids_begin+4) {
                    let rect = button.rect();
                    draw_rectangle(rect.x, rect.y, rect.w, rect.h, button_bg_hover);
                }
            }
        }

        let underscore_amount = LEVEL_NAME_LEN - self.level.name().len();
        render_text(&format!("NAME:"), vec2(view_size.x - 96.0, 0.0), WHITE, TextAlign::Right, texture);
        render_text(self.level.name(), vec2(view_size.x, 0.0), WHITE, TextAlign::Right, texture);
        render_text(&format!("{}", "_".repeat(underscore_amount)), vec2(view_size.x - 96.0, 0.0), grey_col, TextAlign::Left, texture);
    }
}