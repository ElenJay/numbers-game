use std::time::SystemTime;
use raylib::prelude::*;

use crate::game;

pub struct Timer {
    is_running: bool,
    start_time: u64,
    duration: u64,
}

impl Timer {
    // ToDo: implement pause timer feature

    pub fn new(duration: i32) -> Self {
        Self {
            is_running: false,
            start_time: 0,
            duration: duration as u64,
        }
    }

    pub fn get_current_time_in_secs() -> u64 {
        SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs()
    }

    pub fn start(&mut self) {
        self.is_running = true;
        self.start_time = Timer::get_current_time_in_secs();
    }

    pub fn finish(&mut self) {
        self.is_running = false;
        self.start_time = 0;
    }

    pub fn is_over(&self) -> bool {
        let current_time: u64 = Timer::get_current_time_in_secs();
        self.is_running && current_time >= (self.start_time + self.duration)
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle, game: &game::Game) {
        let left_time: u64 = if self.is_running { self.start_time + self.duration - Timer::get_current_time_in_secs() } else { 0 };
        let timer_str = format!("{0:0>2}:{1:0>2}", (left_time - left_time % 60) / 60, left_time % 60);
        d.draw_text(&timer_str, game.get_window_width() - d.measure_text(&timer_str, 48) - 10, 10, 48, Color::BLACK);
    }
}