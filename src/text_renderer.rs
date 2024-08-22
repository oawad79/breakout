use macroquad::{color::Color, math::{vec2, Rect, Vec2}, texture::{draw_texture_ex, DrawTextureParams, Texture2D}};

const CHAR_WIDTH: f32 = 5.0;
const CHAR_HEIGHT: f32 = 6.0;
const CHARS_ORIGIN: Vec2 = vec2(1.0, 16.0);

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum TextAlign {
    Left, Right
}

pub fn render_text(text: &String, pos: Vec2, color: Color, align: TextAlign, texture: &Texture2D) {
    let (change, iter): (f32, Box<dyn Iterator<Item=_>>) = match align {
        TextAlign::Left  => ( CHAR_WIDTH + 1.0, Box::new(text.chars())),
        TextAlign::Right => (-CHAR_WIDTH - 1.0, Box::new(text.chars().rev())),
    };

    let mut x = 0.0;

    if align == TextAlign::Right {
        x += change;
    }

    for c in iter {
        let c_pos = match c {
            ' ' => { x += change; continue; }
            'A'..='Z' => (c as u8 - 'A' as u8, 1),
            '0'..='9' => (c as u8 - '0' as u8, 0),
            '_' => (10, 0),
            '?' => (11, 0),
            '!' => (12, 0),
            '*' => (13, 0),
            ':' => (14, 0),
            '-' => (15, 0),
            '+' => (16, 0),
            '/' => (17, 0),
            '(' => (18, 0),
            ')' => (19, 0),
            '[' => (20, 0),
            ']' => (21, 0),
            '.' => (22, 0),
            ',' => (23, 0),
            '\'' => (24, 0),
            _ => (25, 0), // Draw a sad face if the character isn't valid lol
        };

        let rect = Rect {
            x: CHARS_ORIGIN.x + c_pos.0 as f32 * (CHAR_WIDTH + 1.0),
            y: CHARS_ORIGIN.y + c_pos.1 as f32 * (CHAR_HEIGHT + 1.0),
            w: CHAR_WIDTH,
            h: CHAR_HEIGHT,
        };
        draw_texture_ex(texture, pos.x + x, pos.y, color, DrawTextureParams {
            source: Some(rect),
            ..Default::default()
        });
        x += change;
    }
}

pub fn char_valid(c: char) -> bool {
    c.is_ascii_uppercase() || c.is_ascii_digit() || [' ', '_', '?', '!', '*', ':', '-', '+', '/', '(', ')', '[', ']', '.', ',', '\''].contains(&c)
}