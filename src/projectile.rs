use libm::*;
use macroquad::prelude::*;

pub struct Bullet {
    pub current_pos: Vec2,
    x_vel: f32,
    y_vel: f32,
}

impl Bullet {
    pub fn new(player_pos: Vec2, mouse_pos: Vec2) -> Self {
        let speed: f32 = 10.0;
        let (x_vel, y_vel) = Bullet::direction(player_pos,mouse_pos, speed);
        Self {
            current_pos: player_pos,
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
        self.current_pos.x -= self.x_vel;
        self.current_pos.y -= self.y_vel;
        draw_circle(self.current_pos.x, self.current_pos.y, 5.0, GREEN)
    }
}