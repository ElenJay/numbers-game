use raylib::prelude::*;
use raylib::consts::KeyboardKey::*;

use crate::game;

pub struct Player {
    position: Vector2,
    speed: f32,
    radius: f32,
    color: Color,
}

impl Player {
    pub fn new() -> Player {
        Player {
            position: Vector2::new(game::WINDOW_WIDTH as f32 / 2.0, game::WINDOW_HEIGHT as f32 / 2.0),
            speed: 60.0,
            radius: 30.0,
            color: Color::RED
        }
    }
    
    pub fn process_player_controller(&mut self, rl: &RaylibHandle, game: &game::Game) {
        if game.get_state() == game::GameState::Game {
            let frame_time = rl.get_frame_time();
            if rl.is_key_down(KEY_W) {
                self.position.y -= self.speed * frame_time;
            }
            if rl.is_key_down(KEY_S) {
                self.position.y += self.speed * frame_time;
            }
            if rl.is_key_down(KEY_A) {
                self.position.x -= self.speed * frame_time;
            }
            if rl.is_key_down(KEY_D) {
                self.position.x += self.speed * frame_time;
            }
        }
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle, game: &game::Game) {
        if game.get_state() == game::GameState::Game {
            d.draw_circle_v(self.position, self.radius, self.color);
        }
    }
}