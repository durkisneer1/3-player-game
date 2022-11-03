pub mod player;
mod cursor;

use macroquad::prelude::*;

#[macroquad::main("game")]
async fn main() {
    let mut player1 = player::Player::new(1,
                                          Vec2{x: 0.0, y: 0.0},
                                          BLACK);
    let mut player2 = player::Player::new(2,
                                          Vec2{x: 0.0, y: 0.0},
                                          GRAY);

    loop {
        if is_key_pressed(KeyCode::Escape) {
            break
        }

        let dt = get_frame_time() * 100.0;

        clear_background(WHITE);
        let m_pos = cursor::cursor_player();
        player1.update(dt);
        player2.update(dt);

        next_frame().await
    }
}
