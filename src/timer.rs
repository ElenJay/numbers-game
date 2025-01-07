use std::time::SystemTime;
use raylib::prelude::*;

use crate::game;

const START_DELAY_SECS: f64 = 0.1;

pub struct Timer {
    is_running: bool,
    start_time: f64,
    pause_time: f64,
    duration: f64,
}

impl Timer {
    pub fn new(duration: i32) -> Self {
        Self {
            is_running: false,
            start_time: 0.0,
            pause_time: 0.0,
            duration: duration as f64,
        }
    }

    pub fn get_current_time_in_secs() -> f64 {
        SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs_f64()
    }

    pub fn is_active(&self) -> bool {
        self.is_running
    }

    pub fn activate(&mut self) {
        if Self::get_current_time_in_secs() > self.start_time + START_DELAY_SECS {
            self.is_running = true;
        }
    }

    pub fn start(&mut self) {
        self.is_running = false;
        self.start_time = Timer::get_current_time_in_secs();
    }

    pub fn pause(&mut self) {
        if self.pause_time == 0.0 {
            self.is_running = false;
            self.pause_time = Timer::get_current_time_in_secs();
        }
    }

    pub fn resume(&mut self) {
        if self.pause_time > 0.0 {
            self.start_time = Timer::get_current_time_in_secs() - (self.pause_time - self.start_time);
            self.pause_time = 0.0;
        }
    }

    pub fn finish(&mut self) {
        self.is_running = false;
        self.start_time = 0.0;
    }

    pub fn is_over(&self) -> bool {
        let current_time: f64 = Self::get_current_time_in_secs();
        self.is_running && current_time >= (self.start_time + self.duration)
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle, game: &game::Game, ) {
        let left_time: i32 = if self.is_running { self.start_time + self.duration - Self::get_current_time_in_secs() } else { 0.0 } as i32;
        let timer_str = format!("{0:0>2}:{1:0>2}", (left_time - left_time % 60) / 60, left_time % 60);
        let text_sizes: Vector2 = game.get_font().measure_text(&timer_str, 48.0, game.get_font_spacing());
        let text_pos: Vector2 = Vector2 {
            x: game.get_window_width() as f32 - text_sizes.x - 16.0, 
            y: 10.0
        };

        d.draw_text_ex(game.get_font(), &timer_str, text_pos, 48.0, game.get_font_spacing(), Color::BLACK);
    }
}