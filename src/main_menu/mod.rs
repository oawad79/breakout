use std::collections::HashMap;

use macroquad::{color::{Color, BLUE, GREEN, ORANGE, PURPLE, RED, WHITE, YELLOW}, math::{vec2, Rect, Vec2}, shapes::{draw_rectangle, draw_rectangle_lines}, texture::{draw_texture_ex, DrawTextureParams, Texture2D}, window::clear_background};

use crate::{game::world::BG_COL, gui::{Button, ButtonDetail, Gui, Id, BUTTON_COL_HOVER, BUTTON_COL_IDLE, BUTTON_DETAIL_GREY, GRID_COL}, text_renderer::{render_text, TextAlign}, Scene, SceneChange};

pub struct MainMenu {
    gui: Gui,
    name_timer: f32,
    on_info_tab: bool,
    info_flash: f32,
}

impl MainMenu {
    pub fn new() -> Self {
        let mut buttons = HashMap::new();
        buttons.insert(0, Button::new(Rect::new(24.0, 110.0, 144.0, 10.0), ButtonDetail::Text(String::from("         PLAY         ")), vec2(6.0, 2.0)));
        buttons.insert(1, Button::new(Rect::new(24.0, 130.0, 144.0, 10.0), ButtonDetail::Text(String::from("EDIT CURRENT LEVEL PACK")), vec2(3.0, 2.0)));
        buttons.insert(2, Button::new(Rect::new(24.0, 150.0, 144.0, 10.0), ButtonDetail::Text(String::from("  EDIT NEW LEVEL PACK  ")), vec2(3.0, 2.0)));
        buttons.insert(3, Button::new(Rect::new(24.0, 170.0, 144.0, 10.0), ButtonDetail::Text(String::from("         INFO         ")), vec2(6.0, 2.0)));
        buttons.insert(4, Button::new(Rect::new(24.0, 170.0, 144.0, 10.0), ButtonDetail::Text(String::from("         BACK         ")), vec2(6.0, 2.0)));

        Self {
            gui: Gui::new(buttons),
            name_timer: 0.0,
            on_info_tab: false,
            info_flash: 0.0,
        }
    }
}

impl Scene for MainMenu {
    fn update(&mut self, mouse_pos: Vec2) -> Option<SceneChange> {
        let update_only: Option<&[Id]> = match self.on_info_tab {
            false => Some(&[0, 1, 2, 3]),
            true => Some(&[4]),
        };
        self.gui.update(mouse_pos, update_only);

        for (id, scene_change) in [
            (0, SceneChange::Game),
            (1, SceneChange::Editor { new: false }),
            (2, SceneChange::Editor { new: true }),
        ] {
            if self.gui.button(id).is_some_and(|b| b.released()) {
                return Some(scene_change);
            }
        }

        self.info_flash = (self.info_flash + macroquad::time::get_frame_time()) % 1.0;
        self.name_timer += macroquad::time::get_frame_time();

        if self.gui.button(3).is_some_and(|b| b.released()) {
            self.info_flash = 0.0;
            self.on_info_tab = true;
        }
        if self.gui.button(4).is_some_and(|b| b.released()) {
            self.on_info_tab = false;
        }
        
        None
    }

    fn draw(&self, texture: &Texture2D, level_pack_info: Option<(&String, &String)>) {
        clear_background(BG_COL);

        // Level pack
        if self.on_info_tab {
            let flash = self.info_flash % 1.0 <= 0.5;

            let draw_boxes = |pos: Vec2, flashing: &[usize], flash: &bool, wasd: bool| {
                let origins = [
                    (pos.x,        pos.y + 9.0),
                    (pos.x + 8.0,  pos.y + 9.0),
                    (pos.x + 8.0,  pos.y,     ),
                    (pos.x + 16.0, pos.y + 9.0),
                ];

                for (i, (x, y)) in origins.iter().enumerate() {
                    draw_rectangle_lines(*x, *y, 9.0, 10.0, 2.0, WHITE);
                    if flashing.contains(&i) && *flash {
                        draw_rectangle(*x+1.0, *y+1.0, 7.0, 8.0, Color::from_rgba(255, 255, 255, 128));
                    }
                }

                if wasd {
                    for ((x, y), s) in origins.iter().zip(["A", "S", "W", "D"]) {
                        render_text(&s.to_string(), vec2(*x, *y) + 2.0, WHITE, TextAlign::Left, texture);

                    }
                } else {
                    for ((x, y), (x_offset, rotation)) in origins
                        .iter()
                        .zip([
                            (-0.0, 0.0_f32),
                            (-0.5, 270.0),
                            (-0.5, 90.0),
                            (-1.0, 180.0),
                        ])
                    {
                        draw_texture_ex(texture, *x + 2.0 + x_offset, *y + 2.0, WHITE, DrawTextureParams {
                            source: Some(Rect::new(165.0, 15.0, 6.0, 5.0)),
                            rotation: rotation.to_radians(),
                            ..Default::default()
                        });
                    }
                }
            };

            let y = 20.0;
            render_text(&String::from(" MOVE THE PADDLE WITH "), vec2(30.0, y), WHITE, TextAlign::Left, texture);
            render_text(&String::from("          OR          "), vec2(30.0, y + 18.0), WHITE, TextAlign::Left, texture);
            draw_boxes(vec2(50.0, y + 12.0), &[0, 3], &flash, true);
            draw_boxes(vec2(115.0, y + 12.0), &[0, 3], &flash, false);

            let y = 65.0;
            render_text(&String::from("WHEN    , SHOOT BULLETS WITH"), vec2(12.0, y), WHITE, TextAlign::Left, texture);
            render_text(&String::from("     RED"), vec2(12.0, y), RED, TextAlign::Left, texture);
            render_text(&String::from("             OR             "), vec2(12.0, y + 18.0), WHITE, TextAlign::Left, texture);
            draw_boxes(vec2(50.0, y + 12.0), &[2], &flash, true);
            draw_boxes(vec2(115.0, y + 12.0), &[2], &flash, false);

            let y = 110.0;
            render_text(&String::from("   IF YOU HAVE A '     '   "), vec2(15.0, y), WHITE, TextAlign::Left, texture);
            render_text(&String::from("                  CARRY"), vec2(15.0, y), BLUE, TextAlign::Left, texture);
            render_text(&String::from("(SHOWN ON HUD NEXT TO LIVES)"),  vec2(12.0, y + 8.0), WHITE, TextAlign::Left, texture);
            render_text(&String::from(" PICK UP A BALL BY HOLDING "), vec2(15.0, y + 16.0), WHITE, TextAlign::Left, texture);
            render_text(&String::from("           SPACE           "), vec2(15.0, y + 36.0), WHITE, TextAlign::Left, texture);

            let space_rect = Rect::new(60.0, y + 34.0, 70.0, 10.0);
            draw_rectangle_lines(space_rect.x, space_rect.y, space_rect.w, space_rect.h, 2.0, WHITE);
            if flash {
                draw_rectangle(space_rect.x+1.0, space_rect.y+1.0, space_rect.w-1.0, space_rect.h-1.0, Color::from_rgba(255, 255, 255, 128));
            }

            render_text(&String::from("EVERYTHING* MADE WITH LOVE BY"), vec2(10.0, 185.0), WHITE, TextAlign::Left, texture);
            render_text(&String::from("         JUMBLEDFOX         "), vec2(12.0, 193.0), ORANGE, TextAlign::Left, texture);
            render_text(&String::from("*EXCEPT SOME OF THE JAVASCRIPT"), vec2(7.0, 201.0), BUTTON_DETAIL_GREY, TextAlign::Left, texture);

        } else {

            // Logo
            let logo_pos = |t: f32| -> Vec2 {
                let m = 40.0;
                vec2((t*m).to_radians().sin(), (t*m*2.0).to_radians().sin()) * vec2(31.0, 24.0) + vec2(33.0, 30.0)
            };
    
            for (t_offset, col) in [
                (0.0, WHITE), (1.0, RED), (2.0, ORANGE), (3.0, YELLOW), (4.0, GREEN), (5.0, BLUE), (6.0, PURPLE)
            ].iter().rev() {
                let pos = logo_pos(self.name_timer - t_offset * 0.2);
                render_text(&String::from("JUMBLEDFOX'S BREAKOUT"), pos, *col, TextAlign::Left, texture);
            }

            draw_rectangle_lines(46.0, 75.0, 100.0, 18.0, 2.0, GRID_COL);
            render_text(&String::from("LEVEL PACK LOADED:"), vec2(44.0, 66.0), WHITE, TextAlign::Left, texture);
            if let Some((name, author)) = level_pack_info {
                render_text(name, vec2(48.0, 77.0), WHITE, TextAlign::Left, texture);
                render_text(author, vec2(48.0, 85.0), BUTTON_DETAIL_GREY, TextAlign::Left, texture);
            } else {
                render_text(&String::from(" NO PACK LOADED! "), vec2(47.0, 81.0), BUTTON_DETAIL_GREY, TextAlign::Left, texture);
            }
        }
        

        // Buttons
        let ids: &[Id] = match self.on_info_tab {
            false => &[0, 1, 2, 3],
            true => &[4],
        };
        for &id in ids {
            let button = match self.gui.button(id) {
                Some(b) => b,
                None => continue,
            };
            let gray = (id == 0 || id == 1) && level_pack_info.is_none();
            button.draw(texture, if gray { BUTTON_DETAIL_GREY } else { WHITE }, if button.idle() || gray { BUTTON_COL_IDLE } else { BUTTON_COL_HOVER }, GRID_COL);
        }
    }
}