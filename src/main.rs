mod player;
mod cursor;
mod projectile;

use macroquad::prelude::*;

#[macroquad::main("game")]
async fn main() {
    let mut bullets = Vec::new();

    let player_texture: Texture2D = load_texture("res/player.png").await.unwrap();

    let mut player1 = player::Player::new(1,
                                          Vec2{x: 0.0, y: 0.0},
                                          player_texture,
                                          RED);
    let mut player2 = player::Player::new(2,
                                          Vec2{x: 0.0, y: 0.0},
                                          player_texture,
                                          BLUE);

    loop {
        let dt = get_frame_time() * 100.0;
        let m_pos = cursor::cursor_player();

        if is_key_pressed(KeyCode::Escape) {
            break
        }
        if is_mouse_button_pressed(MouseButton::Left) {
            bullets.push(projectile::Bullet::new(player1.pos, m_pos));
        }

        clear_background(WHITE);
        player1.update(dt, m_pos);
        player2.update(dt, m_pos);
        for bullet in bullets.iter_mut() {
            bullet.movement();
        }

        bullets.retain(|bullet|
            (bullet.current_pos.x > 0.0 && bullet.current_pos.x < screen_width()) ||
                (bullet.current_pos.y > 0.0 && bullet.current_pos.y < screen_height()));

        next_frame().await
    }
}
