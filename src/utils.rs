use raylib::prelude::*;

use crate::game;

pub fn draw_text_center(
    d: &mut RaylibDrawHandle,
    text: &str,
    y: i32,
    font_size: i32,
    color: Color
) {
    let text_length = d.measure_text(text, font_size);
    d.draw_text(text, (game::WINDOW_WIDTH - text_length) / 2, y, font_size, color);
}