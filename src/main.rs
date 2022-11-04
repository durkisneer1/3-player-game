pub mod player;
mod cursor;

use macroquad::prelude::*;

#[macroquad::main("game")]
async fn main() {
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
        if is_key_pressed(KeyCode::Escape) {
            break
        }

        let dt = get_frame_time() * 100.0;

        clear_background(WHITE);
        let m_pos = cursor::cursor_player();
        player1.update(dt, m_pos);
        player2.update(dt, m_pos);

        next_frame().await
    }
}
