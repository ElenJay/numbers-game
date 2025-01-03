use raylib::prelude::*;
use raylib::consts::MouseButton::*;

use crate::game;
use crate::timer;
use crate::utils::{generate_numbers_array, draw_text_center};

const RECTANGLE_WIDTH: f32 = 100.0;
const RECTANGLE_HEIGHT: f32 = 60.0;
const MAX_H_OPACITY: f32 = 100.0;
const MIN_H_OPACITY: f32 = 20.0;
const MAX_V_OPACITY: f32 = 100.0;
const MIN_V_OPACITY: f32 = 20.0;
const H_COUNT: i32 = 8;
const V_COUNT: i32 = 7;

const BTN_EXIT_TEXT: &str = "Exit";
const BTN_TEXT_FONTSIZE: i32 = 48;

pub struct Level {
    numbers: Vec<i32>,
    buttons: Vec<Rectangle>,
    active_btn_index: i32,
    incorrect_btn_index: i32,
    correct_buttons: Vec<i32>,
    score: i32,
    timer: timer::Timer,
    btn_exit: Rectangle,
    btn_exit_color: Color,
}

impl Level {
    pub fn new(game: &game::Game) -> Self {
        let window_width: f32 = game.get_window_width() as f32;
        let window_height: f32 = game.get_window_height() as f32;
        let mut h_opacity: f32 = (window_width - 600.0 - H_COUNT as f32 * RECTANGLE_WIDTH) / (H_COUNT - 1) as f32;
        let mut v_opacity: f32 = (window_height - 600.0 - V_COUNT as f32 * RECTANGLE_HEIGHT) / (V_COUNT - 1) as f32;
        if h_opacity > MAX_H_OPACITY { h_opacity = MAX_H_OPACITY; }
        if v_opacity > MAX_V_OPACITY { v_opacity = MAX_V_OPACITY; }

        let mut obj = Self {
            numbers: Vec::with_capacity((H_COUNT * V_COUNT) as usize),
            buttons: Vec::new(),
            active_btn_index: -1,
            incorrect_btn_index: -1,
            correct_buttons: Vec::new(),
            score: 0,
            timer: timer::Timer::new(2 * 60),
            btn_exit: Rectangle {
                x: window_width as f32 - 150.0 - 10.0, 
                y: 80.0, 
                width: 150.0, 
                height: 60.0, 
            },
            btn_exit_color: Color::WHITE,
        };
        for v_index in 0..V_COUNT {
            for h_index in 0..H_COUNT {
                obj.buttons.push(Rectangle::new(
                    h_index as f32 * (RECTANGLE_WIDTH + h_opacity) + (window_width - H_COUNT as f32 * (RECTANGLE_WIDTH + h_opacity) + h_opacity) / 2.0, 
                    v_index as f32 * (RECTANGLE_HEIGHT + v_opacity) + (window_height - V_COUNT as f32 * (RECTANGLE_HEIGHT + v_opacity) + v_opacity) / 2.0, 
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

    pub fn resume(&mut self, game: &mut game::Game) {
        game.set_state(game::GameState::Game);
        self.timer.resume();
    }

    pub fn restart(&mut self) {
        self.numbers = generate_numbers_array(H_COUNT * V_COUNT);
        self.active_btn_index = -1;
        self.incorrect_btn_index = -1;
        self.correct_buttons.clear();
        self.score = 0;
        self.timer.start();
    }
    
    pub fn process_controller(&mut self, rl: &RaylibHandle, game: &mut game::Game) {
        if game.get_state() == game::GameState::Menu && self.is_started() {
            self.timer.pause();
        }
        if game.get_state() == game::GameState::Game {
            let mouse_pos = rl.get_mouse_position();
            let mut has_collision: bool = false;
            let mut index: i32;

            if self.btn_exit.check_collision_point_rec(mouse_pos) {
                self.btn_exit_color = Color::LIGHTGREEN;
                if rl.is_mouse_button_released(MOUSE_BUTTON_LEFT) {
                    game.set_state(game::GameState::Menu);
                    self.btn_exit_color = Color::WHITE;
                }
            } else {
                self.btn_exit_color = Color::WHITE;
            }

            if self.correct_buttons.len() == (H_COUNT * V_COUNT) as usize {
                game.set_state(game::GameState::Win);
                self.timer.finish();
            }
            if self.timer.is_over() {
                game.set_state(game::GameState::Lose);
                self.timer.finish();
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

    pub fn update_btn_positions(&mut self, game: &game::Game) {
        let window_width: f32 = game.get_window_width() as f32;
        let window_height: f32 = game.get_window_height() as f32;
        let mut h_opacity: f32 = (window_width - 600.0 - H_COUNT as f32 * RECTANGLE_WIDTH) / (H_COUNT - 1) as f32;
        let mut v_opacity: f32 = (window_height - 600.0 - V_COUNT as f32 * RECTANGLE_HEIGHT) / (V_COUNT - 1) as f32;
        let mut index;

        if h_opacity > MAX_H_OPACITY {
            h_opacity = MAX_H_OPACITY;
        } else if h_opacity < MIN_H_OPACITY {
            h_opacity = MIN_H_OPACITY;
        }
        if v_opacity > MAX_V_OPACITY {
            v_opacity = MAX_V_OPACITY;
        } else if v_opacity < MIN_V_OPACITY {
            v_opacity = MIN_V_OPACITY;
        }

        for v_index in 0..V_COUNT {
            for h_index in 0..H_COUNT {
                index = (v_index * H_COUNT + h_index) as usize;
                self.buttons[index].x = h_index as f32 * (RECTANGLE_WIDTH + h_opacity) + (window_width - H_COUNT as f32 * (RECTANGLE_WIDTH + h_opacity) + h_opacity) / 2.0;
                self.buttons[index].y = v_index as f32 * (RECTANGLE_HEIGHT + v_opacity) + (window_height - V_COUNT as f32 * (RECTANGLE_HEIGHT + v_opacity) + v_opacity) / 2.0;
            }
        }
    }

    pub fn draw_level(&self, d: &mut RaylibDrawHandle, game: &game::Game) {
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
            draw_text_center(d, text.as_str(), 24, 36, Color::GREEN, &game);
        }
    }

    pub fn draw_exit_button(&self, d: &mut RaylibDrawHandle, game: &game::Game) {
        if game.get_state() == game::GameState::Game {
            let btn_text_width: i32 = d.measure_text(BTN_EXIT_TEXT, BTN_TEXT_FONTSIZE);
            let btn_padding = Vector2::new(
                self.btn_exit.x + (self.btn_exit.width - btn_text_width as f32) / 2.0, 
                self.btn_exit.y + (self.btn_exit.height - BTN_TEXT_FONTSIZE as f32) / 2.0
            );

            if self.btn_exit_color == Color::WHITE {
                d.draw_rectangle_lines_ex(self.btn_exit, 1.0, Color::BLACK);
            } else {
                d.draw_rectangle_rec(self.btn_exit, self.btn_exit_color);
            }
            d.draw_text(BTN_EXIT_TEXT, btn_padding.x as i32, btn_padding.y as i32, BTN_TEXT_FONTSIZE, Color::BLACK);
        }
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle, game: &game::Game) {
        self.draw_level(d, game);
        self.draw_score(d, game);
        self.draw_exit_button(d, game);
    }
}