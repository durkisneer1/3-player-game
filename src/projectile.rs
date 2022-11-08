use libm::{atan2f, cosf, sinf};
use macroquad::prelude::*;

pub struct Bullet {
    pub rect: Rect,
    x_vel: f32,
    y_vel: f32,
}

impl Bullet {
    pub fn new(player_pos: Vec2, mouse_pos: Vec2) -> Self {
        let speed: f32 = 8.0;
        let (x_vel, y_vel) = Bullet::direction(player_pos,mouse_pos, speed);
        let (x, y) = (player_pos.x, player_pos.y);
        Self {
            rect: Rect::new(
                x,
                y,
                10.0,
                10.0
            ),
            x_vel,
            y_vel,
        }
    }

    pub fn direction(start_pos: Vec2, end_pos: Vec2, speed: f32) -> (f32, f32) {
        let radians = atan2f(start_pos.y - end_pos.y, start_pos.x - end_pos.x);
        let x_vel = cosf(radians) * speed;
        let y_vel = sinf(radians) * speed;
        return (x_vel, y_vel);
    }

    pub fn movement(&mut self) {
        self.rect.x -= self.x_vel;
        self.rect.y -= self.y_vel;
        draw_circle(self.rect.x, self.rect.y, self.rect.w / 2.0, DARKGREEN)
    }
}