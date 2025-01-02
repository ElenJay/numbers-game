use raylib::prelude::*;
use raylib::consts::MouseButton::*;

use crate::game;
use crate::timer;
use crate::utils::{generate_numbers_array, draw_text_center};

const RECTANGLE_WIDTH: f32 = 100.0;
const RECTANGLE_HEIGHT: f32 = 60.0;
const H_OPACITY: f32 = 40.0;
const V_OPACITY: f32 = 40.0;
const H_COUNT: i32 = 8;
const V_COUNT: i32 = 7;

pub struct Level {
    numbers: Vec<i32>,
    buttons: Vec<Rectangle>,
    active_btn_index: i32,
    incorrect_btn_index: i32,
    correct_buttons: Vec<i32>,
    score: i32,
    timer: timer::Timer,
}

impl Level {
    pub fn new() -> Self {
        let mut obj = Self {
            numbers: Vec::with_capacity((H_COUNT * V_COUNT) as usize),
            buttons: Vec::new(),
            active_btn_index: -1,
            incorrect_btn_index: -1,
            correct_buttons: Vec::new(),
            score: 0,
            timer: timer::Timer::new(2 * 60),
        };
        for v_index in 0..V_COUNT {
            for h_index in 0..H_COUNT {
                obj.buttons.push(Rectangle::new(
                    100.0 + h_index as f32 * (RECTANGLE_WIDTH + H_OPACITY), 
                    100.0 + v_index as f32 * (RECTANGLE_HEIGHT + V_OPACITY), 
                    RECTANGLE_WIDTH, 
                    RECTANGLE_HEIGHT
                ));
            }
        }
        obj
    }

    pub fn is_started(&self) -> bool {
        self.numbers.len() > 0
    }

    pub fn restart(&mut self) {
        self.numbers = generate_numbers_array(H_COUNT * V_COUNT);
        self.active_btn_index = -1;
        self.incorrect_btn_index = -1;
        self.correct_buttons.clear();
        self.score = 0;
        self.timer.start();
    }
    
    pub fn process_level_controller(&mut self, rl: &RaylibHandle, game: &mut game::Game) {
        if game.get_state() == game::GameState::Game {
            let mouse_pos = rl.get_mouse_position();
            let mut has_collision: bool = false;
            let mut index: i32;

            if self.correct_buttons.len() == (H_COUNT * V_COUNT) as usize {
                game.set_state(game::GameState::Win)
            }
            if self.timer.is_over() {
                game.set_state(game::GameState::Lose);
            }

            if self.timer.is_active() {
                for (i, el) in self.buttons.iter().enumerate() {
                    index = i as i32;
                    if self.correct_buttons.contains(&index) {
                        continue;
                    }

                    if el.check_collision_point_rec(mouse_pos) {
                        has_collision = true;
                        self.active_btn_index = index;
                        if rl.is_mouse_button_released(MOUSE_BUTTON_LEFT) {
                            if self.numbers[i] == self.correct_buttons.len() as i32 + 1 {
                                self.correct_buttons.push(index);
                                self.incorrect_btn_index = -1;
                                self.score += 1;
                            } else {
                                self.incorrect_btn_index = index;
                            }
                            self.active_btn_index = -1;
                        }
                        break;
                    }
                }
                if !has_collision {
                    self.active_btn_index = -1;
                }
            } else {
                self.timer.activate();
            }
        }
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle, game: &game::Game) {
        if game.get_state() == game::GameState::Game {
            let mut text;
            let mut index: i32;
            let mut text_color: Color = Color::BLACK;

            for (i, el) in self.buttons.iter().enumerate() {
                index = i as i32;
                text = format!("{0}", self.numbers[i]);
                let text_padding = Vector2::new(
                    el.x + (RECTANGLE_WIDTH - d.measure_text(&text, 48) as f32) / 2.0, 
                    el.y + (RECTANGLE_HEIGHT - 48.0) / 2.0
                );

                if self.correct_buttons.contains(&index) {
                    d.draw_rectangle_rec(el, Color::GREEN);
                    text_color = Color::WHITE;
                } else if self.incorrect_btn_index == index {
                    d.draw_rectangle_rec(el, Color::RED);
                    text_color = Color::WHITE;
                } else if self.active_btn_index == index {
                    d.draw_rectangle_rec(el, Color::LIGHTGREEN);
                } else {
                    d.draw_rectangle_lines_ex(el, 2.0, Color::BLACK);
                }
                d.draw_text(&text, text_padding.x as i32, text_padding.y as i32, 48, text_color);
                text_color = Color::BLACK;
            }

            self.timer.draw(d, &game);
        } else if game.get_state() == game::GameState::Win {
            draw_text_center(d, "Congratulations. You win!!!", game.get_window_height() / 2 - 30, 60, Color::GREEN, &game)
        } else if game.get_state() == game::GameState::Lose {
            let lose_text = format!("Sorry. You lose with score: {0} points.", self.score);
            draw_text_center(d, lose_text.as_str(), game.get_window_height() / 2 - 30, 60, Color::RED, &game)
        }
    }

    pub fn draw_score(&self, d: &mut RaylibDrawHandle, game: &game::Game) {
        if game.get_state() == game::GameState::Game {
            let text = format!("Your score  -  {0} points.", self.score);
            draw_text_center(d, text.as_str(), 12, 36, Color::GREEN, &game);
        }
    }
}