use macroquad::prelude::*;
// use rand::gen_range;
use libm::{atan2f};

#[derive(Clone)]
pub struct Enemy {
    rect: Rect,
    speed: f32,
    pub pos: Vec2,
    rot: f32,
    img: Texture2D,
    direction: Vec2,
    frame_list: Vec<Texture2D>,
    current_frame: f32,
    animation_speed: f32,
}

impl Enemy {
    pub async fn new(enemy_pos: Vec2, enemy_img: Texture2D) -> Self {
        let frame_list: Vec<Texture2D> = Enemy::frame_load().await;
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
            frame_list,
            current_frame: 0.0,
            animation_speed: 0.2,
        }
    }

    pub async fn frame_load() -> Vec<Texture2D>{
        let mut frames: Vec<Texture2D> = Vec::new();
        for i in 0..6 {
            let dir = &*format!("res/zombie/{}.png", i);
            let import: Texture2D = load_texture(dir).await.unwrap();
            frames.push(import);
        }
        return frames;
    }

    pub fn animation(&mut self, dt: f32) {
        let frames = &self.frame_list;
        self.current_frame += self.animation_speed * dt;
        if self.current_frame >= frames.len() as f32 {
            self.current_frame = 0.0;
        }
    
        self.img = frames[self.current_frame as usize];
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
        self.animation(dt);
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