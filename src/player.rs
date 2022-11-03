use macroquad::prelude::*;

pub struct Player {
    rect: Rect,
    number: i8,
    speed: f32,
    pos: Vec2,
    color: Color
}

impl Player {
    pub fn new(player_number: i8, player_pos: Vec2, player_color: Color) -> Self {
        Self {
            rect: Rect::new(
                0f32,
                0f32,
                100f32,
                100f32
            ),
            number: player_number,
            speed: 7.0,
            pos: player_pos,
            color: player_color
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

        (self.rect.x, self.rect.y) = <(f32, f32)>::from(self.pos);
    }

    pub fn collision(&mut self) {
        if self.pos.x < 0f32 {
            self.pos.x = 0f32;
        }
        else if self.pos.x > screen_width() - self.rect.w {
            self.pos.x = screen_width() - self.rect.w;
        }
        if self.pos.y < 0f32 {
            self.pos.y = 0f32;
        }
        else if self.pos.y > screen_height() - self.rect.h {
            self.pos.y = screen_height() - self.rect.h;
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.movement(dt);
        self.collision();
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, self.color);
    }
}