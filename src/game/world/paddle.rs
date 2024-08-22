use std::f32::NEG_INFINITY;

use macroquad::{color::WHITE, input::{is_key_down, is_key_released, KeyCode}, math::{vec2, Rect, Vec2}, texture::{draw_texture_ex, DrawTextureParams, Texture2D}};

use super::{ball::{Ball, BALL_SIZE}, bullet::Bullet, level::Level};

const KEY_CARRY: KeyCode = KeyCode::Space;

const PADDLE_SPEED: f32 = 100.0;

const WIDTH_DEFAULT: f32 = 20.0;
const WIDTH_LONG:    f32 = 40.0;
const GROWTH_SPEED:  f32 = 40.0;

pub const PADDLE_LEFT_TEXTURE: Rect = Rect { x: 122.0, y: 8.0, w: 1.0, h: 4.0 };
pub const PADDLE_CENTER_TEXTURE: Rect = Rect { x: 123.0, y: 8.0, w: 1.0, h: 4.0 };
pub const PADDLE_RIGHT_TEXTURE: Rect = Rect { x: 124.0, y: 8.0, w: 1.0, h: 4.0 };

pub struct Paddle {
    x: f32,
    vel: f32,
    width: f32,
    target_width: f32,

    carries: usize,
    carry: Option<Ball>,
    carry_x: f32,

    long:       Option<f32>,
    gun:        Option<f32>,
    balls_safe: Option<f32>,

    shot_timer: f32,
}

impl Paddle {
    pub fn new(x: Option<f32>, carries: Option<usize>) -> Self {
        Self {
            x: x.unwrap_or((Level::view_size().x - WIDTH_DEFAULT) / 2.0),
            vel: 0.0,
            width: WIDTH_DEFAULT,
            target_width: WIDTH_DEFAULT,

            carries: carries.unwrap_or(0),
            carry: Some(Ball::new(vec2(0.0, 0.0), f32::to_radians(90.0), 1.0)),
            carry_x: (WIDTH_DEFAULT - BALL_SIZE) / 2.0,

            long:       None,
            gun:        None,
            balls_safe: None,

            shot_timer: NEG_INFINITY,
        }
    }

    // How far 'x' is from the center to the edge of the paddle, mapped from -1.0 (left edge) to 0.0 (center) to 1.0 (right edge)  
    pub fn center_dist(&self, x: f32) -> f32 {
        let center = self.x + self.width / 2.0;
        let dist = x - center;
        (dist / (self.width / 2.0)).clamp(-1.0, 1.0)
    }

    pub fn vel(&self) -> f32 {
        self.vel
    }
    pub fn x(&self) -> f32 {
        self.x
    }
    pub fn y() -> f32 {
        Level::view_size().y - 12.0
    }
    pub fn carries(&self) -> usize {
        self.carries
    }

    pub fn carrying(&self) -> bool {
        self.carry.is_some()
    }
    pub fn can_carry(&self) -> bool {
        self.carries != 0 && self.carry.is_none() && is_key_down(KEY_CARRY)
    }
    pub fn carry(&mut self, ball: Ball) {
        self.carries = self.carries.saturating_sub(1);
        self.carry_x = ball.pos().x - self.x;
        self.carry = Some(ball);
    }
    pub fn carry_new(&mut self) {
        self.carry = Some(Ball::new(vec2(0.0, 0.0), 0.0, 1.0));
        self.carry_x = (self.width - BALL_SIZE) / 2.0;
    }

    pub fn powerup_gun(&mut self) {
        self.gun = Some(7.0);
    }
    pub fn powerup_grow(&mut self) {
        self.long = Some(15.0);
    }
    pub fn powerup_balls_safe(&mut self) {
        self.balls_safe = Some(7.0);
    }
    pub fn powerup_carry(&mut self) {
        if self.carries <3 { // awwww :3
            self.carries += 1
        }
    }

    pub fn has_gun_powerup(&self) -> bool {
        self.gun.is_some()
    }

    pub fn balls_safe(&self) -> bool {
        self.balls_safe.is_some()
    }
    pub fn balls_safe_display(&self) -> bool {
        self.balls_safe.is_some_and(|t| t % 0.25 <= 0.125 || t > 1.5)
    }

    // The rect of the center bit of the paddle
    pub fn center_rect(&self) -> Rect {
        Rect::new(self.x + 1.0, Paddle::y(), self.width - 2.0, 4.0)
    }

    // The one used for collision with the ball / world
    pub fn collision_rect(&self) -> Rect {
        let r = self.center_rect();
        Rect::new(
            r.x - 1.0,
            r.y,
            r.w,
            0.01,
        )
    }

    pub fn update(&mut self, delta: f32, bullets: &mut Vec<Bullet>) -> Option<Ball> {
        let prev_x = self.x;
        // Powerup timers
        for timer in [&mut self.gun, &mut self.long, &mut self.balls_safe] {
            if let Some(t) = timer {
                *t -= delta;
            }
            if timer.is_some_and(|t| t <= 0.0) {
                *timer = None;
            }
        }

        // Shooting
        self.shot_timer -= delta;
        if (is_key_down(KeyCode::W) || is_key_down(KeyCode::Up)) && self.gun.is_some() && self.shot_timer <= 0.0 {
            self.shot_timer = 0.3;
            bullets.push(Bullet::new(vec2(self.x + 2.0, Paddle::y())));
            bullets.push(Bullet::new(vec2(self.x - 2.0 + self.width, Paddle::y())));
        }

        // Growing / shrinking
        self.target_width = match self.long {
            Some(_) => WIDTH_LONG,
            _ => WIDTH_DEFAULT,
        };
        if self.width != self.target_width {
            let change = if self.width > self.target_width { -1.0 } else { 1.0 } * delta * GROWTH_SPEED;
            self.width = (self.width + change).clamp(self.width.min(self.target_width), self.width.max(self.target_width));
            
            self.x -= change / 2.0;
            self.carry_x += prev_x - self.x;
        }
        
        self.vel = 0.0;
        if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) {
            self.x -= delta * PADDLE_SPEED;
            self.vel -= 1.0;
        }
        if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right){
            self.x += delta * PADDLE_SPEED;
            self.vel += 1.0;
        }

        self.x = self.x.clamp(0.0, Level::view_size().x - self.width);

        if let Some(carry) = &mut self.carry {
            self.carry_x = self.carry_x.clamp(0.0, self.width - BALL_SIZE);
            carry.set_pos(vec2(self.x + self.carry_x, Paddle::y() - 4.0));
            carry.set_vel(Vec2::from_angle(45.0_f32.to_radians()) * if self.vel == -1.0 { -1.0 } else { 1.0 })
        }

        if is_key_released(KEY_CARRY) {
            return self.carry.take();
        }
        None
    }

    pub fn draw(&self, texture: &Texture2D) {
        let center_rect = self.center_rect();

        let paddle_texture_offset = match self.gun {
            Some(t) if t > 2.0 || t % 0.2 >= 0.1 => 4.0,
            _ => 0.0,
        };

        // Sides
        draw_texture_ex(texture, self.x, Paddle::y(), WHITE, DrawTextureParams {
            source: Some(PADDLE_LEFT_TEXTURE.offset(vec2(paddle_texture_offset, 0.0))),
            ..Default::default()
        });
        draw_texture_ex(texture, self.x + center_rect.w + 1.0, Paddle::y(), WHITE, DrawTextureParams {
            source: Some(PADDLE_RIGHT_TEXTURE.offset(vec2(paddle_texture_offset, 0.0))),
            ..Default::default()
        });
        // Center
        draw_texture_ex(texture, center_rect.x, Paddle::y(), WHITE, DrawTextureParams {
            source: Some(PADDLE_CENTER_TEXTURE.offset(vec2(paddle_texture_offset, 0.0))),
            dest_size: Some(center_rect.size()),
            ..Default::default()
        });

        if let Some(carry) = &self.carry {
            carry.draw(texture);
        }
    }
}