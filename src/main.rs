pub mod player;

use macroquad::prelude::*;

#[macroquad::main("game")]
async fn main() {
    let mut player1 = player::Player::new(1,
                                          Vec2{x: 0f32, y: 0f32},
                                          BLACK);
    let mut player2 = player::Player::new(2,
                                          Vec2{x: 0f32, y: 0f32},
                                          GRAY);

    loop {
        let dt = get_frame_time() * 100f32;

        clear_background(WHITE);
        player1.update(dt);
        player2.update(dt);

        next_frame().await
    }
}
