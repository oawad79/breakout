use editor::Editor;
use game::{level_pack::LevelPack, world::level::Level, Game};
use macroquad::prelude::*;
use main_menu::MainMenu;
// use sapp_jsutils::JsObject;

pub mod game;
pub mod editor;
pub mod gui;
pub mod text_renderer;
pub mod main_menu;

#[cfg(target_arch = "wasm32")]
use game::level_pack::try_load_level;

pub enum SceneChange {
    MainMenu,
    Game,
    Editor { new: bool },
}
pub trait Scene {
    fn update(&mut self, mouse_pos: Vec2, level_pack: &Option<LevelPack>) -> Option<SceneChange>;
    fn draw(&self, texture: &Texture2D);
}

fn window_conf()-> Conf {
    let window_size = Level::view_size();
    macroquad::logging::info!("{}, {}", window_size.x as i32 * 3, window_size.y as i32 * 3);

    Conf { 
        window_title: String::from("Breakout"),
        window_width:  window_size.x as i32 * 6,
        window_height: window_size.y as i32 * 6,
        high_dpi: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf())]
async fn main() {
    macroquad::logging::info!("hello?");
    macroquad::rand::srand(macroquad::miniquad::date::now() as _);
    
    let view_size = Level::view_size();
    let texture = Texture2D::from_file_with_format(include_bytes!("../res/sprites.png"), None);
    texture.set_filter(FilterMode::Nearest);

    let camera = Camera2D::from_display_rect(Rect::new(0.0, view_size.y, view_size.x, -view_size.y));

    #[cfg(not(target_arch = "wasm"))]
    let level_pack = Some(LevelPack::load_from_file(include_bytes!("../SPACE.brk").into()).unwrap());
    #[cfg(target_arch = "wasm32")]
    let mut level_pack = None;

    let mut scene: Box<dyn Scene> = Box::new(MainMenu::new());

    loop {
        #[cfg(target_arch = "wasm32")]
        if let Some(lp) = try_load_level() {
            level_pack = Some(lp);
        }

        set_camera(&camera);
        let mouse_pos = camera.screen_to_world(vec2(mouse_position().0, mouse_position().1));

        let change = scene.update(mouse_pos, &level_pack);
        scene.draw(&texture);

        if let Some(change) = change {
            scene = match (change, &level_pack) {
                (SceneChange::Editor { new: true }, Some(lp)) => Box::new(Editor::from_level_pack(lp.clone())),
                (SceneChange::Editor { new: false }, _)       => Box::new(Editor::default()),
                (SceneChange::Game, Some(lp)) => Box::new(Game::new(lp.clone())),
                (SceneChange::MainMenu, _) =>Box::new(MainMenu::new()),
                _ => scene
            };
        };

        next_frame().await;
    }
}