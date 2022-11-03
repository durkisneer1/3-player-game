use macroquad::prelude::*;

pub fn cursor_player() -> Vec2 {
    let m_pos = mouse_position();
    let pos = Vec2{x: m_pos.0, y: m_pos.1};
    draw_circle(pos.x, pos.y, 10.0, RED);
    return pos;
}