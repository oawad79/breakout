use level_pack::LevelPack;
use macroquad::{color::{Color, WHITE}, input::{is_key_pressed, KeyCode}, math::{vec2, Vec2}, shapes::{draw_rectangle, draw_rectangle_lines}, texture::Texture2D};
use pause_menu::PauseMenu;
use world::{level::Level, Lives, World, WorldUpdateReturn, BG_COL};

use crate::{gui::GRID_COL, text_renderer::{render_text, TextAlign}, Scene, SceneChange};

pub mod world;
pub mod level_pack;
pub mod pause_menu;

pub const KEY_PAUSE: KeyCode = KeyCode::Escape;

pub struct Game {
    level_pack: LevelPack,
    current_level: usize,
    world: World,

    pause_menu: PauseMenu,
    well_done_timer: Option<f32>,
    ball_stuck_timer: Option<f32>,
}

impl Game {
    pub fn new(level_pack: LevelPack) -> Game {
        Game {
            world: World::new(level_pack.levels().get(0).unwrap().clone(), None, None, Lives::Default, None),
            current_level: 0,
            well_done_timer: None,
            ball_stuck_timer: None,
            pause_menu: PauseMenu::new(),
            level_pack,
        }
    }
}

impl Scene for Game {
    fn update(&mut self, mouse_pos: Vec2, _: &Option<LevelPack>) -> Option<SceneChange> {
        self.pause_menu.update(mouse_pos);

        if is_key_pressed(KEY_PAUSE) {
            *self.pause_menu.paused_mut() = !self.pause_menu.paused();
        }
        if self.pause_menu.button_exit() {
            return Some(SceneChange::MainMenu);
        }
        if self.pause_menu.paused() {
            return None;
        }

        let world_update_return = self.world.update();

        let delta = macroquad::time::get_frame_time();
        self.well_done_timer = self.well_done_timer.map(|t| t - delta);
        self.ball_stuck_timer = self.ball_stuck_timer.map(|t| t - delta);

        if world_update_return == WorldUpdateReturn::BallStuck && self.well_done_timer.is_none() {
            self.ball_stuck_timer = Some(3.0);
        }
        if self.ball_stuck_timer.is_some_and(|t| t <= 0.0) {
            self.ball_stuck_timer = None;
            self.world.give_free_ball();
        }

        if self.world.level_complete() && self.well_done_timer.is_none() {
            self.well_done_timer = Some(3.0);
        }
        if self.well_done_timer.is_some_and(|t| t <= 0.0) {
            // Load the next level, or return to the menu if there are none left
            self.current_level += 1;

            let next_level = match self.level_pack.levels().get(self.current_level) {
                Some(l) => l.clone(),
                None => return Some(SceneChange::MainMenu),
            };
            self.world = World::new(next_level, Some(self.world.score()), Some(self.world.paddle_pos()), self.world.lives(), Some(self.world.carries()));
            self.well_done_timer = None;
        }

        None
    }
    
    fn draw(&self, texture: &Texture2D) {
        self.world.draw(texture);

        if self.well_done_timer.is_some_and(|t| t % 1.0 >= 0.5 || t >= 3.0) {
            draw_rectangle(51.0, 83.0, 89.0, 20.0, BG_COL);
            draw_rectangle_lines(51.0, 83.0, 89.0, 20.0, 2.0, GRID_COL);
            render_text(&String::from("LEVEL COMPLETE"), vec2(54.0, 86.0), WHITE, TextAlign::Left, texture);
            render_text(&String::from("  WELL DONE!  "), vec2(54.0, 94.0), WHITE, TextAlign::Left, texture);
        } else
        if self.ball_stuck_timer.is_some() && self.well_done_timer.is_none() {
            draw_rectangle(27.0, 79.0, 143.0, 28.0, BG_COL);
            draw_rectangle_lines(27.0, 79.0, 143.0, 28.0, 2.0, GRID_COL);
            render_text(&String::from(" IT APPEARS YOUR BALL "),  vec2(33.0, 82.0), WHITE, TextAlign::Left, texture);
            render_text(&String::from("   HAS BECOME STUCK!   "), vec2(30.0, 90.0), WHITE, TextAlign::Left, texture);
            render_text(&String::from("GIVING YOU A NEW ONE :3"), vec2(30.0, 98.0), WHITE, TextAlign::Left, texture);
        }

        // Text
        render_text(self.level_pack.author(), vec2(Level::view_size().x, 7.0), Color::from_rgba(255, 255, 255, 128), TextAlign::Right, &texture);

        self.pause_menu.draw(texture);
    }
}