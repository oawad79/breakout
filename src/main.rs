use editor::Editor;
use game::{level::Level, Game};
use macroquad::prelude::*;

pub mod game;
pub mod editor;
pub mod gui;
pub mod text_renderer;

fn window_conf()-> Conf {
    let window_size = Level::view_size();
    Conf { 
        window_title: String::from("Breakout"),
        window_width: window_size.x as i32 * 6,
        window_height: window_size.y as i32 * 6,
        high_dpi: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf())]
async fn main() {
    let mut editor = Editor::new();

    let mut game = Game::new(Level::new(), None, None);

    let view_size = Level::view_size();
    let camera = Camera2D::from_display_rect(Rect::new(0.0, view_size.y, view_size.x, -view_size.y));
    let texture = Texture2D::from_file_with_format(include_bytes!("../res/sprites.png"), None);
    texture.set_filter(FilterMode::Nearest);

    loop {
        set_camera(&camera);

        editor.update(&camera);
        editor.draw(&texture);

        next_frame().await;
    }
}