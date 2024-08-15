use editor::Editor;
use game::Game;
use macroquad::prelude::*;

pub mod game;
pub mod editor;


pub trait Scene {
    fn new() -> Self;
    fn update(&mut self);
    fn draw(&self, texture: &Texture2D);
}


fn window_conf()-> Conf {
    Conf { 
        window_title: String::from("Breakout"),
        window_width: 192,
        window_height: 224,
        high_dpi: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf())]
async fn main() {
    let mut game = Game::new();
    let mut editor = Editor::new();
    let texture = Texture2D::from_file_with_format(include_bytes!("../res/sprites.png"), None);
    texture.set_filter(FilterMode::Nearest);

    loop {
        set_camera(&Camera2D::from_display_rect(Rect::new(0.0, 224.0, 192.0, -224.0)));

        game.update();
        game.draw(&texture);

        next_frame().await;
    }
}