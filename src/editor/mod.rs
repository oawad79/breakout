use editor_gui::EditorGui;
use editor_level_pack::EditorLevelPack;
use macroquad::{color::WHITE, input::{clear_input_queue, is_key_pressed, is_mouse_button_pressed, is_mouse_button_released, MouseButton}, math::{vec2, Rect, Vec2}, shapes::draw_line, texture::Texture2D, window::clear_background};

use crate::{game::{level_pack::LevelPack, world::{level::{Level, Tile, LEVEL_HEIGHT, LEVEL_HEIGHT_PADDING_TOP, LEVEL_WIDTH, TILE_GAP, TILE_HEIGHT, TILE_WIDTH}, Lives, World, BG_COL}, KEY_PAUSE}, gui::GRID_COL, text_renderer::{render_text, TextAlign}, Scene, SceneChange};

pub mod editor_gui;
pub mod editor_level_pack;
pub mod timewarp;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum ClickAction {
    None, Draw, Erase,
}

pub struct Editor {
    gui: EditorGui,

    draw_type: Tile,
    click_action: ClickAction,

    level_pack: EditorLevelPack,

    world: Option<World>,
    paddle_pos: Option<f32>,
}

impl Default for Editor {
    fn default() -> Self {
        Self {
            gui: EditorGui::new(),

            draw_type: Tile::Red,
            click_action: ClickAction::None,
            
            level_pack: EditorLevelPack::new(),

            world: None,
            paddle_pos: None,
        }
    }
}

impl Editor {
    pub fn from_level_pack(level_pack: LevelPack) -> Self {
        Editor {
            level_pack: level_pack.into(),
            ..Default::default()
        }
    }
}

impl Scene for Editor {
    fn update(&mut self, mouse_pos: Vec2) -> Option<SceneChange> {
        if is_key_pressed(KEY_PAUSE) {
            if self.world.is_some() {
                self.paddle_pos = self.world.as_ref().map(|g| g.paddle_pos());
                self.world = None;
            } else {
                self.gui.stop_editing_name();
                self.world = Some(World::new(self.level_pack.level().clone(), None, self.paddle_pos, Lives::Infinite, None));
            }
            clear_input_queue();
        }

        if let Some(world) = &mut self.world {
            world.update();
            return None;
        }

        // Gui
        self.gui.update(mouse_pos, &mut self.level_pack, &mut self.draw_type);

        // Clear
        if self.gui.button_clear() && *self.level_pack.level().tiles() != [Tile::Air; LEVEL_WIDTH*LEVEL_HEIGHT] {
            self.level_pack.timewarp_save_previous_state();
            self.level_pack.timewarp_push_current_state();
            *self.level_pack.level_mut() = Level::new();
        }

        // Exit / Save
        if self.gui.button_exit() {
            return Some(SceneChange::MainMenu);
        }
        if self.gui.button_save() {
            self.level_pack.save();
        }
        
        // Undo / Redo
        if self.gui.button_undo() {
            self.level_pack.timewarp_undo();
        }
        if self.gui.button_redo() {
            self.level_pack.timewarp_redo();
        }

        // Level shifting / adding / all that jazz
        if self.gui.button_level_add() {
            self.level_pack.add_level();
        }
        if self.gui.button_level_next() {
            self.level_pack.next();
        }
        if self.gui.button_level_prev() {
            self.level_pack.prev();
        }
        if self.gui.button_level_shift_next() {
            self.level_pack.shift_next();
        }
        if self.gui.button_level_shift_prev() {
            self.level_pack.shift_prev();
        }
        if self.gui.button_level_delete() {
            self.level_pack.delete_level();
        }

        // Editing tiles
        let level_area_rect = Rect::new(0.0, LEVEL_HEIGHT_PADDING_TOP as f32 * (TILE_HEIGHT + TILE_GAP), LEVEL_WIDTH as f32 * (TILE_WIDTH + TILE_GAP), LEVEL_HEIGHT as f32 * (TILE_HEIGHT + TILE_GAP));

        if level_area_rect.contains(mouse_pos) && !self.gui.popup_open() {
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
                    self.level_pack.timewarp_save_previous_state();
                }
                if is_mouse_button_released(mouse_button) && self.click_action == click_action {
                    self.click_action = ClickAction::None;
                    self.level_pack.timewarp_push_current_state();
                }
            }
            
            if let Some(tile_index) = hovered_tile_index {
                let tile_type = match self.click_action {
                    ClickAction::Draw  => Some(self.draw_type),
                    ClickAction::Erase => Some(Tile::Air),
                    _ => None
                };
                if let Some(tile_type) = tile_type {
                    self.level_pack.level_mut().tiles_mut().get_mut(tile_index).map(|t| *t = tile_type);
                }
            }
        } else {
            if self.click_action != ClickAction::None {
                self.click_action = ClickAction::None;
                self.level_pack.timewarp_push_current_state();
            }
        }

        None
    }

    fn draw(&self, texture: &Texture2D, _: Option<(&String, &String)>) {
        if let Some(world) = &self.world {
            world.draw(texture);
            render_text(&String::from("PRESS ESC TO RETURN TO EDITOR."), vec2(0.0, 7.0), WHITE, TextAlign::Left, texture);
            return;
        }

        clear_background(BG_COL);

        let view_size = Level::view_size();

        // Grid
        for x in 1..=LEVEL_WIDTH {
            let x = x as f32 * (TILE_WIDTH + TILE_GAP) - 0.5;
            draw_line(x, LEVEL_HEIGHT_PADDING_TOP as f32 * (TILE_HEIGHT + TILE_GAP), x, (LEVEL_HEIGHT_PADDING_TOP + LEVEL_HEIGHT) as f32  * (TILE_HEIGHT + TILE_GAP), 1.0, GRID_COL);
        }
        for y in 0..=LEVEL_HEIGHT {
            let y = (y + LEVEL_HEIGHT_PADDING_TOP) as f32 * (TILE_HEIGHT + TILE_GAP) - 0.5;
            draw_line(0.0, y, view_size.x, y, 1.0, GRID_COL);
        }
        self.level_pack.level().draw(texture);

        // Gui
        self.gui.draw(texture, &self.level_pack, &self.draw_type);
    }
}