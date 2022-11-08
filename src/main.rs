mod player;
mod cursor;
mod projectile;
mod enemy;

use macroquad::prelude::*;

#[macroquad::main("game")]
async fn main() {
    let mut bullets = Vec::new();
    let mut enemies = Vec::new();

    let player_texture: Texture2D = load_texture("res/player.png").await.unwrap();
    let enemy_texture: Texture2D = load_texture("res/zombie/0.png").await.unwrap();

    let mut player1 = player::Player::new(1,
                                          Vec2{x: 0.0, y: 0.0},
                                          player_texture,
                                          RED);
    // let mut player2 = player::Player::new(2,
    //                                       Vec2{x: 0.0, y: 0.0},
    //                                       player_texture,
    //                                       BLUE);

    let default_enemy = enemy::Enemy::new(Vec2{x: 0.0, y: 0.0}, enemy_texture).await;
    
    loop {
        let dt = get_frame_time() * 100.0;
        let m_pos = cursor::cursor_player();

        if is_key_pressed(KeyCode::Escape) {
            break
        }
        if is_mouse_button_pressed(MouseButton::Left) {
            bullets.push(projectile::Bullet::new(player1.pos, m_pos));
            // bullets.push(projectile::Bullet::new(player2.pos, m_pos));
        }
        if is_mouse_button_pressed(MouseButton::Right) {
            enemies.push(default_enemy.clone());
        }

        clear_background(WHITE);
        player1.update(dt, m_pos);
        // player2.update(dt, m_pos);

        for bullet in bullets.iter_mut() {
            bullet.movement();
            enemies.retain(|enemy| !enemy.rect.overlaps(&bullet.rect));
        }
        for enemy in enemies.iter_mut() {
            enemy.update(dt, player1.pos);
        }

        cursor::cursor_player();

        bullets.retain(|bullet|
            (bullet.rect.x > 0.0 && bullet.rect.x < screen_width()) ||
                (bullet.rect.y > 0.0 && bullet.rect.y < screen_height()));

        next_frame().await
    }
}
