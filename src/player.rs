use macroquad::prelude::*;
use libm::atan2f;

pub struct Player {
    rect: Rect,
    number: i8,
    speed: f32,
    pos: Vec2,
    rot: f32,
    img: Texture2D,
    color: Color,
}

impl Player {
    pub fn new(player_number: i8, player_pos: Vec2, player_img: Texture2D, player_color: Color) -> Self {
        Self {
            rect: Rect::new(
                0.0,
                0.0,
                100.0,
                100.0
            ),
            number: player_number,
            speed: 7.0,
            pos: player_pos,
            rot: 0.0,
            img: player_img,
            color: player_color,
        }
    }

    pub fn movement(&mut self, dt: f32) {
        if self.number == 1 {
            if is_key_down(KeyCode::D) {
                self.pos.x += self.speed * dt;
            }
            if is_key_down(KeyCode::A) {
                self.pos.x -= self.speed * dt;
            }
            if is_key_down(KeyCode::W) {
                self.pos.y -= self.speed * dt;
            }
            if is_key_down(KeyCode::S) {
                self.pos.y += self.speed * dt;
            }
        }
        else if self.number == 2 {
            if is_key_down(KeyCode::Right) {
                self.pos.x += self.speed * dt;
            }
            if is_key_down(KeyCode::Left) {
                self.pos.x -= self.speed * dt;
            }
            if is_key_down(KeyCode::Up) {
                self.pos.y -= self.speed * dt;
            }
            if is_key_down(KeyCode::Down) {
                self.pos.y += self.speed * dt;
            }
        }
    }

    pub fn collision(&mut self) {
        if self.pos.x < (self.rect.w / 2.0) {
            self.pos.x = self.rect.w / 2.0;
        }
        else if self.pos.x > screen_width() - (self.rect.w / 2.0) {
            self.pos.x = screen_width() - (self.rect.w / 2.0);
        }
        if self.pos.y < (self.rect.h / 2.0) {
            self.pos.y = self.rect.h / 2.0;
        }
        else if self.pos.y > screen_height() - (self.rect.h / 2.0) {
            self.pos.y = screen_height() - (self.rect.h / 2.0);
        }
    }

    pub fn rotation(&mut self, m_pos: Vec2) {
        let adj = m_pos.x - self.pos.x;
        let opp = m_pos.y - self.pos.y;
        self.rot = atan2f(opp, adj);
    }

    pub fn update(&mut self, dt: f32, m_pos: Vec2) {
        self.movement(dt);
        self.rotation(m_pos);
        self.collision();
        draw_texture_ex(
            self.img,
            self.pos.x - (self.rect.w / 2.0),
            self.pos.y - (self.rect.h / 2.0),
            self.color,
            DrawTextureParams {
                dest_size: Some(vec2(self.rect.w, self.rect.h)),
                rotation: self.rot,
                ..Default::default()
            },
        );
    }
}