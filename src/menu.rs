use raylib::prelude::*;
use raylib::consts::MouseButton::*;

use crate::game;
use crate::player;

const BTN_START_TEXT: &str = "Start";
const BTN_LOAD_TEXT: &str = "Load";
const BTN_SETTINGS_TEXT: &str = "Settings";
const BTN_EXIT_TEXT: &str = "Exit";
const BTN_FULLSCREEN_TEXT: &str = "Fullscreen";
const BTN_VSYNC_TEXT: &str = "Vsync";
const BTN_BACK_TEXT: &str = "Back";

#[derive(Clone, Copy, PartialEq)]
pub enum MenuState {
    // None,
    Primary,
    Settings,
}

pub struct Menu {
    state: MenuState,
    btn_start: Rectangle,
    btn_start_color: Color,
    btn_load: Rectangle,
    btn_load_color: Color,
    btn_settings: Rectangle,
    btn_settings_color: Color,
    btn_exit: Rectangle,
    btn_exit_color: Color,
    btn_fullscreen: Rectangle,
    btn_fullscreen_color: Color,
    btn_back: Rectangle,
    btn_back_color: Color,
}

impl Menu {
    pub fn new(game: &game::Game) -> Self {
        Self {
            state: MenuState::Primary,
            btn_start: Rectangle::new(
                (game.get_window_width() as f32 - 400.0) / 2.0, 
                (game.get_window_height() as f32) / 2.0 - 220.0, 
                400.0, 
                80.0,
            ),
            btn_start_color: Color::LIGHTGRAY,
            btn_load: Rectangle::new(
                (game.get_window_width() as f32 - 400.0) / 2.0, 
                (game.get_window_height() as f32) / 2.0 - 100.0, 
                400.0, 
                80.0,
            ),
            btn_load_color: Color::LIGHTGRAY,
            btn_settings: Rectangle::new(
                (game.get_window_width() as f32 - 400.0) / 2.0, 
                (game.get_window_height() as f32) / 2.0 + 20.0, 
                400.0, 
                80.0
            ),
            btn_settings_color: Color::LIGHTGRAY,
            btn_exit: Rectangle::new(
                (game.get_window_width() as f32 - 400.0) / 2.0, 
                (game.get_window_height() as f32) / 2.0 + 140.0, 
                400.0, 
                80.0
            ),
            btn_exit_color: Color::LIGHTGRAY,
            btn_fullscreen: Rectangle::new(
                (game.get_window_width() as f32 - 400.0) / 2.0, 
                (game.get_window_height() as f32) / 2.0 - 100.0, 
                400.0, 
                80.0
            ),
            btn_fullscreen_color: Color::LIGHTGRAY,
            btn_back: Rectangle::new(
                (game.get_window_width() as f32 - 400.0) / 2.0, 
                (game.get_window_height() as f32) / 2.0 + 20.0, 
                400.0, 
                80.0
            ),
            btn_back_color: Color::LIGHTGRAY,
        }
    }

    pub fn process_menu_controller(&mut self, rl: &mut RaylibHandle, game: &mut game::Game, player: &mut player::Player) {
        if game.get_state() == game::GameState::Menu {
            let mouse_pos = rl.get_mouse_position();

            if self.state == MenuState::Primary {
                if self.btn_start.check_collision_point_rec(mouse_pos) {
                    self.btn_start_color = Color::LIGHTGREEN;
                    if rl.is_mouse_button_released(MOUSE_BUTTON_LEFT) {
                        game.set_state(game::GameState::Game);
                        player.restart();
                    }
                } else {
                    self.btn_start_color = Color::LIGHTGRAY;
                }

                if self.btn_load.check_collision_point_rec(mouse_pos) {
                    self.btn_load_color = Color::LIGHTGREEN;
                    if rl.is_mouse_button_released(MOUSE_BUTTON_LEFT) {
                        game.set_state(game::GameState::Game);
                    }
                } else {
                    self.btn_load_color = Color::LIGHTGRAY;
                }

                if self.btn_settings.check_collision_point_rec(mouse_pos) {
                    self.btn_settings_color = Color::LIGHTGREEN;
                    if rl.is_mouse_button_released(MOUSE_BUTTON_LEFT) {
                        self.state = MenuState::Settings;
                    }
                } else {
                    self.btn_settings_color = Color::LIGHTGRAY;
                }

                if self.btn_exit.check_collision_point_rec(mouse_pos) {
                    self.btn_exit_color = Color::LIGHTGREEN;
                    if rl.is_mouse_button_released(MOUSE_BUTTON_LEFT) {
                        std::process::exit(0);
                    }
                } else {
                    self.btn_exit_color = Color::LIGHTGRAY;
                }
            } else if self.state == MenuState::Settings {
                if self.btn_fullscreen.check_collision_point_rec(mouse_pos) {
                    self.btn_fullscreen_color = Color::LIGHTGREEN;
                    if rl.is_mouse_button_released(MOUSE_BUTTON_LEFT) {
                        rl.toggle_fullscreen();
                        self.update_btn_positions(game);
                    }
                } else {
                    self.btn_fullscreen_color = Color::LIGHTGRAY;
                }

                if self.btn_back.check_collision_point_rec(mouse_pos) {
                    self.btn_back_color = Color::LIGHTGREEN;
                    if rl.is_mouse_button_released(MOUSE_BUTTON_LEFT) {
                        self.state = MenuState::Primary;
                    }
                } else {
                    self.btn_back_color = Color::LIGHTGRAY;
                }
            }
        }
    }

    pub fn draw_menu(&self, d: &mut RaylibDrawHandle, game: &game::Game) {
        if game.get_state() == game::GameState::Menu {
            if self.state == MenuState::Primary {
                self.draw_menu_button(d, &self.btn_start, BTN_START_TEXT, &self.btn_start_color);
                self.draw_menu_button(d, &self.btn_load, BTN_LOAD_TEXT, &self.btn_load_color);
                self.draw_menu_button(d, &self.btn_settings, BTN_SETTINGS_TEXT, &self.btn_settings_color);
                self.draw_menu_button(d, &self.btn_exit, BTN_EXIT_TEXT, &self.btn_exit_color);
            } else if self.state == MenuState::Settings {
                self.draw_menu_button(d, &self.btn_fullscreen, BTN_FULLSCREEN_TEXT, &self.btn_fullscreen_color);
                self.draw_menu_button(d, &self.btn_back, BTN_BACK_TEXT, &self.btn_back_color);
            }
        }
    }

    pub fn draw_menu_button(&self, d: &mut RaylibDrawHandle, btn: &Rectangle, btn_text: &str, btn_color: &Color) {
        d.draw_rectangle_rec(btn, btn_color);
        let btn_padding = Vector2::new(
            btn.x + (btn.width - d.measure_text(btn_text, 64) as f32) / 2.0, 
            btn.y + (btn.height - 64.0) / 2.0
        );
        d.draw_text(btn_text, btn_padding.x as i32, btn_padding.y as i32, 64, Color::BLACK);
    }

    pub fn update_btn_positions(&mut self, game: &game::Game) {
        self.btn_start.x = (game.get_window_width() as f32 - 400.0) / 2.0;
        self.btn_start.y = (game.get_window_height() as f32) / 2.0 - 220.0;

        self.btn_load.x = (game.get_window_width() as f32 - 400.0) / 2.0;
        self.btn_load.y = (game.get_window_height() as f32) / 2.0 - 100.0;

        self.btn_settings.x = (game.get_window_width() as f32 - 400.0) / 2.0;
        self.btn_settings.y = (game.get_window_height() as f32) / 2.0 + 20.0;

        self.btn_exit.x = (game.get_window_width() as f32 - 400.0) / 2.0;
        self.btn_exit.y = (game.get_window_height() as f32) / 2.0 + 140.0;

        self.btn_fullscreen.x = (game.get_window_width() as f32 - 400.0) / 2.0;
        self.btn_fullscreen.y = (game.get_window_height() as f32) / 2.0 - 100.0;

        self.btn_back.x = (game.get_window_width() as f32 - 400.0) / 2.0;
        self.btn_back.y = (game.get_window_height() as f32) / 2.0 + 20.0;
    }
}