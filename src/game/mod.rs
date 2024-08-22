use level_pack::LevelPack;
use macroquad::{color::{Color, WHITE}, input::{is_key_pressed, KeyCode}, math::{vec2, Vec2}, shapes::{draw_rectangle, draw_rectangle_lines}, texture::Texture2D};
use pause_menu::PauseMenu;
use world::{level::Level, Lives, World, WorldUpdateReturn, BG_COL};

use crate::{gui::{BUTTON_DETAIL_GREY, BUTTON_DETAIL_HELP, GRID_COL}, text_renderer::{render_text, TextAlign}, Scene, SceneChange};

pub mod world;
pub mod level_pack;
pub mod pause_menu;

pub const KEY_PAUSE: KeyCode = KeyCode::Escape;

#[derive(PartialEq, Eq, Debug)]
enum TimerKind {
    NextLevel, BallStuck, GameOver,
}

pub struct Game {
    level_pack: LevelPack,
    current_level: usize,
    world: World,

    pause_menu: PauseMenu,
    timer: Option<(f32, TimerKind)>,
    pack_complete: bool,
    pack_time: f32,
}

impl Game {
    pub fn new(level_pack: LevelPack) -> Game {
        let world = World::new(level_pack.levels().get(0).unwrap().clone(), None, None, Lives::Default, None);
        Game {
            level_pack,
            current_level: 0,
            world,
            pause_menu: PauseMenu::new(),
            timer: None,
            pack_complete: false,
            pack_time: 0.0,
        }
    }
}

impl Scene for Game {
    fn update(&mut self, mouse_pos: Vec2) -> Option<SceneChange> {
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

        if self.pack_complete && is_key_pressed(KeyCode::Space) {
            return Some(SceneChange::MainMenu);
        }

        let delta = macroquad::time::get_frame_time();

        if let Some((t, _)) = &mut self.timer {
            *t = (*t - delta).max(0.0);
        }

        if world_update_return == WorldUpdateReturn::BallStuck && self.timer.is_none() {
            self.timer = Some((3.0, TimerKind::BallStuck));
        }
        if self.timer == Some((0.0, TimerKind::BallStuck)) {
            self.timer = None;
            self.world.give_free_ball();
        }

        if self.world.level_complete() && !matches!(self.timer, Some((_, TimerKind::NextLevel))) {
            self.timer = Some((3.0, TimerKind::NextLevel));
        }
        if self.timer == Some((0.0, TimerKind::NextLevel)) {
            // Load the next level, or return to the menu if there are none left
            self.current_level += 1;

            let next_level = match self.level_pack.levels().get(self.current_level) {
                Some(l) => l.clone(),
                None => {
                    self.pack_complete = true;
                    return None;
                },
            };
            self.world = World::new(next_level, Some(self.world.score()), Some(self.world.paddle_pos()), self.world.lives(), Some(self.world.carries()));
            self.timer = None;
        }

        if world_update_return == WorldUpdateReturn::GameOver && !matches!(self.timer, Some((_, TimerKind::GameOver)) | Some((_, TimerKind::NextLevel))) {
            self.timer = Some((6.0, TimerKind::GameOver));
        }
        if self.timer == Some((0.0, TimerKind::GameOver)) {
            return Some(SceneChange::MainMenu);
        }

        if !(matches!(self.timer, Some((_, TimerKind::NextLevel))) || self.pack_complete || self.pause_menu.paused()) {
            self.pack_time += delta;
        }

        None
    }
    
    fn draw(&self, texture: &Texture2D, _: Option<(&String, &String)>) {
        self.world.draw(texture);

        if self.pack_complete {
            let minutes = ((self.pack_time / 60.0).floor() as i32).clamp(0, 99);
            let seconds = ((self.pack_time % 60.0).floor() as i32).clamp(0, 59);
            let millis  = (((self.pack_time % 1.0) * 1000.0) as i32).clamp(0, 9999);

            draw_rectangle(27.0, 79.0, 143.0, 58.0, BG_COL);
            draw_rectangle_lines(27.0, 79.0, 143.0, 58.0, 2.0, GRID_COL);
            render_text(&String::from(" LEVEL PACK COMPLETE! "),  vec2(33.0, 82.0), WHITE, TextAlign::Left, texture);
            render_text(&String::from("PACK:                 "),  vec2(33.0, 92.0), WHITE, TextAlign::Left, texture);
            render_text(&String::from("  BY:                 "),  vec2(33.0, 100.0), WHITE, TextAlign::Left, texture);
            render_text(&format!("      {}", self.level_pack.name()),  vec2(33.0, 92.0), BUTTON_DETAIL_HELP, TextAlign::Left, texture);
            render_text(&format!("      {}", self.level_pack.author()),  vec2(33.0, 100.0), BUTTON_DETAIL_HELP, TextAlign::Left, texture);

            render_text(&format!(" SCORE: {}", self.world.score()), vec2(33.0, 110.0), WHITE, TextAlign::Left, texture);
            render_text(&format!(" TIME:  {:0>2}:{:0>2}.{:0>4}", minutes, seconds, millis), vec2(33.0, 118.0), WHITE, TextAlign::Left, texture);
            render_text(&String::from(" PRESS SPACE FOR MENU "),  vec2(33.0, 128.0), BUTTON_DETAIL_GREY, TextAlign::Left, texture);

        } else
        if matches!(self.timer, Some((t, TimerKind::NextLevel)) if t % 1.0 >= 0.5 || t >= 3.0) {
            draw_rectangle(51.0, 83.0, 89.0, 20.0, BG_COL);
            draw_rectangle_lines(51.0, 83.0, 89.0, 20.0, 2.0, GRID_COL);
            render_text(&String::from("LEVEL COMPLETE"), vec2(54.0, 86.0), WHITE, TextAlign::Left, texture);
            render_text(&String::from("  WELL DONE!  "), vec2(54.0, 94.0), WHITE, TextAlign::Left, texture);
        } else
        if matches!(self.timer, Some((_, TimerKind::GameOver))) {
            draw_rectangle(51.0, 83.0, 89.0, 20.0, BG_COL);
            draw_rectangle_lines(51.0, 83.0, 89.0, 20.0, 2.0, GRID_COL);
            render_text(&String::from("  GAME OVER!  "), vec2(54.0, 86.0), WHITE, TextAlign::Left, texture);
            render_text(&String::from(" YOU LOSE :P "), vec2(57.0, 94.0), WHITE, TextAlign::Left, texture);
        } else
        if matches!(self.timer, Some((_, TimerKind::BallStuck))) {
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