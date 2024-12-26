use rand::thread_rng;
use rand::seq::SliceRandom;
use raylib::prelude::*;

use crate::game;

pub fn draw_text_center(d: &mut RaylibDrawHandle, text: &str, y: i32, font_size: i32, color: Color, game: &game::Game) {
    let text_length = d.measure_text(text, font_size);
    d.draw_text(text, (game.get_window_width() - text_length) / 2, y, font_size, color);
}

pub fn generate_numbers_array(length: i32) -> Vec<i32> {
    let mut vec: Vec<i32> = (1..=length).collect();
    vec.shuffle(&mut thread_rng());
    vec
}