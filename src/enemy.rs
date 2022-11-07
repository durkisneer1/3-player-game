use macroquad::prelude::*;
// use rand::gen_range;

use libm::{atan2f};

pub struct Enemy {
    rect: Rect,
    speed: f32,
    pub pos: Vec2,
    rot: f32,
    img: Texture2D,
    direction: Vec2,
}

impl Enemy {
    pub fn new(enemy_pos: Vec2, enemy_img: Texture2D) -> Self {
        Self {
            rect: Rect::new(
                0.0,
                0.0,
                100.0,
                100.0
            ),
            speed: 3.0,
            pos: enemy_pos,
            rot: 0.0,
            img: enemy_img,
            direction: Vec2{x: 0.0, y: 0.0},
        }
    }

    pub fn movement(&mut self, dt: f32, destination_pos: Vec2) {
        if self.direction.length() != 0.0 {
            self.direction = self.direction.normalize();
        }
        self.pos += self.direction * self.speed * dt;

        self.direction.y = destination_pos.y - self.pos.y;
        self.direction.x = destination_pos.x - self.pos.x;
    }

    pub fn rotation(&mut self, m_pos: Vec2) {
        self.rot = atan2f(m_pos.y - self.pos.y, m_pos.x - self.pos.x);
    }

    pub fn update(&mut self, dt: f32, player_pos: Vec2) {
        self.rotation(player_pos);
        self.movement(dt, player_pos);
        draw_texture_ex(
            self.img,
            self.pos.x - (self.rect.w / 2.0),
            self.pos.y - (self.rect.h / 2.0),
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(self.rect.w, self.rect.h)),
                rotation: self.rot,
                ..Default::default()
            },
        );
    }
}