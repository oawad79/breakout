use editor::Editor;
use game::{level::Level, Game};
use macroquad::prelude::*;

pub mod game;
pub mod editor;


pub trait Scene {
    fn new() -> Self;
    fn update(&mut self);
    fn draw(&self, texture: &Texture2D);
}


fn window_conf()-> Conf {
    let window_size = Level::view_size();
    Conf { 
        window_title: String::from("Breakout"),
        window_width: window_size.x as i32 * 2,
        window_height: window_size.y as i32 * 2,
        high_dpi: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf())]
async fn main() {
    // let mut editor = Editor::new();
    let mut game = Game::new();

    let view_size = Level::view_size();
    let camera = Camera2D::from_display_rect(Rect::new(0.0, view_size.y, view_size.x, -view_size.y));
    let texture = Texture2D::from_file_with_format(include_bytes!("../res/sprites.png"), None);
    texture.set_filter(FilterMode::Nearest);

    loop {
        set_camera(&camera);

        game.update();
        game.draw(&texture);

        next_frame().await;
    }
}