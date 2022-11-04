use macroquad::prelude::*;
use libm::atan2f;

pub struct Player {
    rect: Rect,
    number: i8,
    speed: f32,
    pub pos: Vec2,
    rot: f32,
    img: Texture2D,
    color: Color,
    direction: Vec2,
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
            direction: Vec2{x: 0.0, y: 0.0},
        }
    }

    pub fn movement(&mut self, dt: f32) {
        if self.direction.length() != 0.0 {
            self.direction = self.direction.normalize();
        }
        self.pos += self.direction * self.speed * dt;

        if self.number == 1 {
            if is_key_down(KeyCode::D) {
                self.direction.x = 1.0;
            } else if is_key_down(KeyCode::A) {
                self.direction.x = -1.0;
            } else {
                self.direction.x = 0.0;
            }

            if is_key_down(KeyCode::W) {
                self.direction.y = -1.0;
            } else if is_key_down(KeyCode::S) {
                self.direction.y = 1.0;
            } else {
                self.direction.y = 0.0;
            }
        }

        else if self.number == 2 {
            if is_key_down(KeyCode::Right) {
                self.direction.x = 1.0;
            } else if is_key_down(KeyCode::Left) {
                self.direction.x = -1.0;
            } else {
                self.direction.x = 0.0;
            }

            if is_key_down(KeyCode::Up) {
                self.direction.y = -1.0;
            } else if is_key_down(KeyCode::Down) {
                self.direction.y = 1.0;
            } else {
                self.direction.y = 0.0;
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