use rand::thread_rng;
use rand::seq::SliceRandom;
use raylib::prelude::*;

use crate::game;

pub fn draw_text_center(d: &mut RaylibDrawHandle, text: &str, y: f32, font_size: f32, color: Color, game: &game::Game) {
    let text_length = d.measure_text(text, font_size as i32);
    let text_pos: Vector2 = Vector2 {
        x: (game.get_window_width() - text_length) as f32 / 2.0, 
        y: y
    };
    d.draw_text_ex(game.get_font(), text, text_pos, font_size as f32, game.get_font_spacing(), color);
}

pub fn generate_numbers_array(length: i32) -> Vec<i32> {
    let mut vec: Vec<i32> = (1..=length).collect();
    vec.shuffle(&mut thread_rng());
    vec
}