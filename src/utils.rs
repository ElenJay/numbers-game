use rand::thread_rng;
use rand::seq::SliceRandom;
use raylib::prelude::*;

use crate::game;

pub struct Button {
    rec: Rectangle,
    color: Color,
    title: String,
}

impl Button {
    pub fn new(rec: Rectangle, title: &str, color: Color) -> Self {
        Self {
            rec: rec, 
            color: color, 
            title: title.to_string(), 
        }
    }

    pub fn get_rec(&self) -> &Rectangle {
        &self.rec
    }

    pub fn get_title(&self) -> &String {
        &self.title
    }

    pub fn get_color(&self) -> Color {
        self.color
    }

    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }
}

pub fn draw_text_center(d: &mut RaylibDrawHandle, text: &str, y: f32, font_size: f32, color: Color, game: &game::Game) {
    let text_sizes: Vector2 = game.get_font().measure_text(text, font_size, game.get_font_spacing());
    let text_pos: Vector2 = Vector2 {
        x: (game.get_window_width() - text_sizes.x) / 2.0, 
        y: y
    };
    d.draw_text_ex(game.get_font(), text, text_pos, font_size as f32, game.get_font_spacing(), color);
}

pub fn generate_numbers_array(length: i32) -> Vec<i32> {
    let mut vec: Vec<i32> = (1..=length).collect();
    vec.shuffle(&mut thread_rng());

    vec
}