use macroquad::{color::WHITE, input::{is_key_down, is_key_pressed, is_key_released, KeyCode}, math::{vec2, Rect}, texture::{draw_texture_ex, DrawTextureParams, Texture2D}};

use super::{ball::{Ball, BALL_SIZE}, level::{Level, LEVEL_WIDTH, TILE_GAP, TILE_WIDTH}};

const KEY_CARRY: KeyCode = KeyCode::Space;

const WIDTH_DEFAULT: f32 = 20.0;
const WIDTH_LONG:    f32 = 30.0;

pub struct Paddle {
    x: f32,
    long:      bool,
    gun:       bool,
    can_carry: bool,
    carry: Option<Ball>,
    carry_x: f32,
}

impl Paddle {
    pub fn new() -> Self {
        Self {
            x: 0.0,
            long: false,
            gun: false,
            can_carry: true,
            carry: Some(Ball::new(vec2(0.0, 0.0), vec2(1.0, -1.0))),
            carry_x: 0.0,
        }
    }

    // The rect of the center bit of the paddle
    pub fn center_rect(&self) -> Rect {
        Rect::new(self.x + 1.0, Paddle::y(), self.width() - 2.0, 4.0)
    }

    // The one used for collision with the ball / world
    pub fn collision_rect(&self) -> Rect {
        let r = self.center_rect();
        Rect::new(
            r.x - 1.0,
            r.y,
            r.w,
            0.1,
        )
    }

    pub fn width(&self) -> f32 {
        match self.long {
            false => WIDTH_DEFAULT,
            true => WIDTH_LONG,
        }
    }

    pub fn y() -> f32 {
        Level::view_size().y - 8.0
    }

    pub fn can_carry(&self) -> bool {
        self.can_carry && self.carry.is_none() && is_key_down(KEY_CARRY)
    }
    pub fn carry(&mut self, ball: Ball) {
        self.carry_x = ball.pos().x - self.x;
        self.carry = Some(ball);
    }

    pub fn update(&mut self, delta: f32) -> Option<Ball> {
        if is_key_down(KeyCode::A) {
            self.x -= delta * 100.0;
        }
        if is_key_down(KeyCode::D) {
            self.x += delta * 100.0;
        }

        self.x = self.x.clamp(0.0, Level::view_size().x - self.width());

        let width = self.width();
        if let Some(carry) = &mut self.carry {
            self.carry_x = self.carry_x.clamp(0.0, width - BALL_SIZE);
            carry.set_pos(vec2(self.x + self.carry_x, Paddle::y() - 4.0));
        }

        if is_key_released(KEY_CARRY) {
            return self.carry.take();
        }
        None
    }

    pub fn draw(&self, texture: &Texture2D) {
        let center_rect = self.center_rect();

        // Sides
        draw_texture_ex(texture, self.x, Paddle::y(), WHITE, DrawTextureParams {
            source: Some(Rect::new(6.0, 8.0, 1.0, 4.0)),
            ..Default::default()
        });
        draw_texture_ex(texture, self.x + center_rect.w + 1.0, Paddle::y(), WHITE, DrawTextureParams {
            source: Some(Rect::new(8.0, 8.0, 1.0, 4.0)),
            ..Default::default()
        });
        // Center
        draw_texture_ex(texture, center_rect.x, Paddle::y(), WHITE, DrawTextureParams {
            source: Some(Rect::new(7.0, 8.0, 1.0, 4.0)),
            dest_size: Some(center_rect.size()),
            ..Default::default()
        });

        if let Some(carry) = &self.carry {
            carry.draw(texture);
        }
    }
}