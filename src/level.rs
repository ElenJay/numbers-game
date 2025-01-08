use raylib::prelude::*;
use raylib::consts::MouseButton::*;

use crate::game;
use crate::timer;
use crate::utils::{ generate_numbers_array, draw_text_center, Button };

const RECTANGLE_WIDTH: f32 = 100.0;
const RECTANGLE_HEIGHT: f32 = 60.0;
const MAX_H_OPACITY: f32 = 100.0;
const MIN_H_OPACITY: f32 = 20.0;
const MAX_V_OPACITY: f32 = 100.0;
const MIN_V_OPACITY: f32 = 20.0;
const H_COUNT: i32 = 8;
const V_COUNT: i32 = 7;

const BTN_TRY_AGAIN_TEXT: &str = "Try Again";
const BTN_EXIT_TEXT: &str = "Exit";
const BTN_TEXT_FONTSIZE: f32 = 48.0;

pub struct Level {
    numbers: Vec<i32>,
    buttons: Vec<Rectangle>,
    active_btn_index: i32,
    incorrect_btn_index: i32,
    correct_buttons: Vec<i32>,
    score: i32,
    fails: i32,
    timer: timer::Timer,
    btn_game_exit: Button,
    btn_after_game_try_again: Button,
    btn_after_game_exit: Button,
}

impl Level {
    pub fn new(game: &game::Game) -> Self {
        let window_width: f32 = game.get_window_width();
        let window_height: f32 = game.get_window_height();
        let mut h_opacity: f32 = (window_width - 600.0 - H_COUNT as f32 * RECTANGLE_WIDTH) / (H_COUNT - 1) as f32;
        let mut v_opacity: f32 = (window_height - 600.0 - V_COUNT as f32 * RECTANGLE_HEIGHT) / (V_COUNT - 1) as f32;
        if h_opacity > MAX_H_OPACITY { h_opacity = MAX_H_OPACITY; }
        if v_opacity > MAX_V_OPACITY { v_opacity = MAX_V_OPACITY; }

        let mut obj: Self = Self {
            numbers: Vec::with_capacity((H_COUNT * V_COUNT) as usize),
            buttons: Vec::new(),
            active_btn_index: -1,
            incorrect_btn_index: -1,
            correct_buttons: Vec::new(),
            score: 0,
            fails: 0,
            timer: timer::Timer::new(Self::get_timer_duration(game)),
            btn_game_exit: Button::new(Rectangle {
                x: window_width - 150.0 - 10.0, 
                y: 80.0, 
                width: 150.0, 
                height: 60.0, 
            }, BTN_EXIT_TEXT, Color::WHITE),
            btn_after_game_try_again: Button::new(Rectangle {
                x: window_width / 2.0 - 250.0, 
                y: window_height / 2.0 + 100.0, 
                width: 250.0, 
                height: 60.0, 
            }, BTN_TRY_AGAIN_TEXT, Color::WHITE),
            btn_after_game_exit: Button::new(Rectangle {
                x: window_width / 2.0 + 50.0, 
                y: window_height / 2.0 + 100.0, 
                width: 150.0, 
                height: 60.0, 
            }, BTN_EXIT_TEXT, Color::WHITE),
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

    fn get_timer_duration(game: &game::Game) -> i32 {
        if game.get_mode() == game::GameMode::Release {
            match game.get_difficulty() {
                game::GameDifficulty::Easy => 3 * 60,
                game::GameDifficulty::Medium => 2 * 60,
                game::GameDifficulty::Hard => 2 * 60,
            }
        } else if game.get_mode() == game::GameMode::Debug {
            match game.get_difficulty() {
                game::GameDifficulty::Easy => 3 * 60,
                game::GameDifficulty::Medium => 60,
                game::GameDifficulty::Hard => 10,
            }
        } else { 0 }
    }

    pub fn is_started(&self) -> bool {
        self.numbers.len() > 0
    }

    pub fn resume(&mut self, game: &mut game::Game) {
        game.set_state(game::GameState::Game);
        self.timer.resume();
    }

    pub fn start(&mut self, game: &game::Game) {
        self.numbers = generate_numbers_array(H_COUNT * V_COUNT);
        self.active_btn_index = -1;
        self.incorrect_btn_index = -1;
        self.correct_buttons.clear();
        self.score = 0;
        self.timer = timer::Timer::new(Self::get_timer_duration(game));
        self.timer.start();
    }

    pub fn restart(&mut self, game: &game::Game) {
        self.active_btn_index = -1;
        self.incorrect_btn_index = -1;
        self.correct_buttons.clear();
        self.score = 0;
        self.timer = timer::Timer::new(Self::get_timer_duration(game));
        self.timer.start();
    }
    
    pub fn process_controller(&mut self, rl: &RaylibHandle, game: &mut game::Game) {
        let mouse_pos: Vector2 = rl.get_mouse_position();

        if game.get_state() == game::GameState::Menu && self.is_started() {
            self.timer.pause();
        }
        if game.get_state() == game::GameState::Game {
            let mut has_collision: bool = false;
            let mut index: i32;

            if self.btn_game_exit.get_rec().check_collision_point_rec(mouse_pos) {
                self.btn_game_exit.set_color(Color::LIGHTGREEN);
                if rl.is_mouse_button_released(MOUSE_BUTTON_LEFT) {
                    game.set_state(game::GameState::Menu);
                    self.btn_game_exit.set_color(Color::WHITE);
                }
            } else {
                self.btn_game_exit.set_color(Color::WHITE);
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
                                if self.incorrect_btn_index != -1 {
                                    self.fails += 1;
                                }
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
        } else if game.get_state() == game::GameState::Win || game.get_state() == game::GameState::Lose {
            if self.btn_after_game_try_again.get_rec().check_collision_point_rec(mouse_pos) {
                self.btn_after_game_try_again.set_color(Color::LIGHTGREEN);
                if rl.is_mouse_button_released(MOUSE_BUTTON_LEFT) {
                    self.btn_after_game_try_again.set_color(Color::WHITE);
                    game.set_state(game::GameState::Game);
                    self.restart(game);
                }
            } else {
                self.btn_after_game_try_again.set_color(Color::WHITE);
            }

            if self.btn_after_game_exit.get_rec().check_collision_point_rec(mouse_pos) {
                self.btn_after_game_exit.set_color(Color::LIGHTGREEN);
                if rl.is_mouse_button_released(MOUSE_BUTTON_LEFT) {
                    game.set_state(game::GameState::Menu);
                    self.btn_after_game_exit.set_color(Color::WHITE);
                }
            } else {
                self.btn_after_game_exit.set_color(Color::WHITE);
            }
        }
    }

    pub fn update_btn_positions(&mut self, game: &game::Game) {
        let window_width: f32 = game.get_window_width();
        let window_height: f32 = game.get_window_height();
        let mut h_opacity: f32 = (window_width - 600.0 - H_COUNT as f32 * RECTANGLE_WIDTH) / (H_COUNT - 1) as f32;
        let mut v_opacity: f32 = (window_height - 600.0 - V_COUNT as f32 * RECTANGLE_HEIGHT) / (V_COUNT - 1) as f32;
        let mut index: usize;

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

    pub fn draw(&self, d: &mut RaylibDrawHandle, game: &game::Game) {
        if game.get_state() == game::GameState::Game {
            self.draw_game(d, game);
            self.draw_score(d, &game);
            self.draw_game_exit_button(d, &game);
        } else if game.get_state() == game::GameState::Win {
            self.draw_win(d, game);
        } else if game.get_state() == game::GameState::Lose {
            self.draw_lose(d, game);
        }
    }

    fn draw_game(&self, d: &mut RaylibDrawHandle, game: &game::Game) {
        let mut text: String;
        let mut index: i32;
        let mut text_color: Color = Color::BLACK;
        let mut text_sizes: Vector2;
        let mut text_padding: Vector2;
        let is_hard_difficulty: bool = game.get_difficulty() == game::GameDifficulty::Hard;

        for (i, el) in self.buttons.iter().enumerate() {
            index = i as i32;
            text = format!("{0}", self.numbers[i]);
            text_sizes = game.get_font().measure_text(&text, 48.0, game.get_font_spacing());
            text_padding = Vector2 {
                x: el.x + (RECTANGLE_WIDTH - text_sizes.x) / 2.0, 
                y: el.y + (RECTANGLE_HEIGHT - text_sizes.y) / 2.0
            };

            if self.correct_buttons.contains(&index) {
                if is_hard_difficulty {
                    d.draw_rectangle_lines_ex(el, 2.0, Color::BLACK);
                } else {
                    d.draw_rectangle_rec(el, Color::GREEN);
                    text_color = Color::WHITE;
                }
            } else if self.incorrect_btn_index == index {
                d.draw_rectangle_rec(el, Color::RED);
                text_color = Color::WHITE;
            } else if self.active_btn_index == index {
                d.draw_rectangle_rec(el, Color::LIGHTGREEN);
            } else {
                d.draw_rectangle_lines_ex(el, 2.0, Color::BLACK);
            }
            d.draw_text_ex(game.get_font(), &text, text_padding, 48.0, game.get_font_spacing(), text_color);
            text_color = Color::BLACK;
        }

        self.timer.draw(d, &game);
    }

    fn draw_win(&self, d: &mut RaylibDrawHandle, game: &game::Game) {
        draw_text_center(d, "Congratulations. You win!!!", game.get_window_height() / 2.0 - 30.0, 60.0, Color::GREEN, &game);

        self.draw_after_game_buttons(d, game);
    }

    fn draw_lose(&self, d: &mut RaylibDrawHandle, game: &game::Game) {
        let lose_text: String = format!("Sorry. You lose with score: {0} points (and {1} fails).", self.score, self.fails);
        draw_text_center(d, lose_text.as_str(), game.get_window_height() / 2.0 - 30.0, 60.0, Color::RED, &game);

        self.draw_after_game_buttons(d, game);
    }

    fn draw_score(&self, d: &mut RaylibDrawHandle, game: &game::Game) {
        let text: String = format!("Your score  -  {0} points.", self.score);
        draw_text_center(d, text.as_str(), 24.0, 36.0, Color::GREEN, &game);
    }

    fn draw_game_exit_button(&self, d: &mut RaylibDrawHandle, game: &game::Game) {
        let btn_text_sizes: Vector2 = game.get_font().measure_text(BTN_EXIT_TEXT, BTN_TEXT_FONTSIZE, game.get_font_spacing());
        let btn_padding: Vector2 = Vector2 {
            x: self.btn_game_exit.get_rec().x + (self.btn_game_exit.get_rec().width - btn_text_sizes.x) / 2.0, 
            y: self.btn_game_exit.get_rec().y + (self.btn_game_exit.get_rec().height - btn_text_sizes.y) / 2.0
        };

        if self.btn_game_exit.get_color() == Color::WHITE {
            d.draw_rectangle_lines_ex(self.btn_game_exit.get_rec(), 1.0, Color::BLACK);
        } else {
            d.draw_rectangle_rec(self.btn_game_exit.get_rec(), self.btn_game_exit.get_color());
        }
        d.draw_text_ex(game.get_font(), BTN_EXIT_TEXT, btn_padding, BTN_TEXT_FONTSIZE, game.get_font_spacing(), Color::BLACK);
    }

    fn draw_after_game_buttons(&self, d: &mut RaylibDrawHandle, game: &game::Game) {
        for btn in [&self.btn_after_game_try_again, &self.btn_after_game_exit].iter() {
            // Draw button
            let btn_text_sizes: Vector2 = game.get_font().measure_text(btn.get_title().as_str(), BTN_TEXT_FONTSIZE, game.get_font_spacing());
            let btn_padding: Vector2 = Vector2 {
                x: btn.get_rec().x + (btn.get_rec().width - btn_text_sizes.x) / 2.0, 
                y: btn.get_rec().y + (btn.get_rec().height - btn_text_sizes.y) / 2.0
            };

            if btn.get_color() == Color::WHITE {
                d.draw_rectangle_lines_ex(btn.get_rec(), 1.0, Color::BLACK);
            } else {
                d.draw_rectangle_rec(btn.get_rec(), btn.get_color());
            }
            d.draw_text_ex(game.get_font(), btn.get_title().as_str(), btn_padding, BTN_TEXT_FONTSIZE, game.get_font_spacing(), Color::BLACK);
        }
    }
}