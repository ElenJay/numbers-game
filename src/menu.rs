use raylib::prelude::*;
use raylib::consts::MouseButton::*;

use crate::game;

const BTN_START_TEXT: &str = "Start";
const BTN_SETTINGS_TEXT: &str = "Settings";
const BTN_EXIT_TEXT: &str = "Exit";
const BTN_FULLSCREEN_TEXT: &str = "Fullscreen";
const BTN_VSYNC_TEXT: &str = "Vsync";
const BTN_BACK_TEXT: &str = "Back";

#[derive(Clone, Copy, PartialEq)]
pub enum MenuState {
    None,
    Primary,
    Settings,
}

pub struct Menu {
    state: MenuState,
    btn_start: Rectangle,
    btn_start_color: Color,
    btn_settings: Rectangle,
    btn_settings_color: Color,
    btn_exit: Rectangle,
    btn_exit_color: Color,
    btn_fullscreen: Rectangle,
    btn_fullscreen_color: Color,
    btn_vsync: Rectangle,
    btn_vsync_color: Color,
    btn_back: Rectangle,
    btn_back_color: Color,
}

impl Menu {
    pub fn new(game: &game::Game) -> Self {
        Self {
            state: MenuState::Primary,
            btn_start: Rectangle::new(
                (game.get_window_width() as f32 - 400.0) / 2.0, 
                (game.get_window_height() as f32 - 320.0) / 2.0, 
                400.0, 
                80.0,
            ),
            btn_start_color: Color::LIGHTGRAY,
            btn_settings: Rectangle::new(
                (game.get_window_width() as f32 - 400.0) / 2.0, 
                (game.get_window_height() as f32 - 80.0) / 2.0, 
                400.0, 
                80.0
            ),
            btn_settings_color: Color::LIGHTGRAY,
            btn_exit: Rectangle::new(
                (game.get_window_width() as f32 - 400.0) / 2.0, 
                (game.get_window_height() as f32 + 160.0) / 2.0, 
                400.0, 
                80.0
            ),
            btn_exit_color: Color::LIGHTGRAY,
            btn_fullscreen: Rectangle::new(
                (game.get_window_width() as f32 - 400.0) / 2.0, 
                (game.get_window_height() as f32 - 320.0) / 2.0, 
                400.0, 
                80.0
            ),
            btn_fullscreen_color: Color::LIGHTGRAY,
            btn_vsync: Rectangle::new(
                (game.get_window_width() as f32 - 400.0) / 2.0, 
                (game.get_window_height() as f32 - 80.0) / 2.0, 
                400.0, 
                80.0
            ),
            btn_vsync_color: Color::LIGHTGRAY,
            btn_back: Rectangle::new(
                (game.get_window_width() as f32 - 400.0) / 2.0, 
                (game.get_window_height() as f32 + 160.0) / 2.0, 
                400.0, 
                80.0
            ),
            btn_back_color: Color::LIGHTGRAY,
        }
    }

    pub fn process_menu_controller(&mut self, rl: &mut RaylibHandle, game: &mut game::Game) {
        if game.get_state() == game::GameState::Menu {
            let mouse_pos = rl.get_mouse_position();

            if self.state == MenuState::Primary {
                if self.btn_start.check_collision_point_rec(mouse_pos) {
                    self.btn_start_color = Color::LIGHTGREEN;
                    if rl.is_mouse_button_released(MOUSE_BUTTON_LEFT) {
                        game.set_state(game::GameState::Game);
                    }
                } else {
                    self.btn_start_color = Color::LIGHTGRAY;
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

                if self.btn_vsync.check_collision_point_rec(mouse_pos) {
                    self.btn_vsync_color = Color::LIGHTGREEN;
                    if rl.is_mouse_button_released(MOUSE_BUTTON_LEFT) {
                        game.toggle_vsync_enabled();
                        // // ToDo: cannot turn-off VSync mode! Need to research and fix
                        // // ToDo: fix bug with resetting other flags (fullscreen mode will be turned off)
                        rl.set_window_state(rl.get_window_state().set_vsync_hint(game.get_vsync_enabled()));
                    }
                } else {
                    self.btn_vsync_color = Color::LIGHTGRAY;
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
        } else {
            self.state = MenuState::None;
        }
    }

    pub fn draw_menu(&self, d: &mut RaylibDrawHandle, game: &game::Game) {
        if game.get_state() == game::GameState::Menu {
            if self.state == MenuState::Primary {
                d.draw_rectangle_rec(self.btn_start, self.btn_start_color);
                d.draw_rectangle_rec(self.btn_settings, self.btn_settings_color);
                d.draw_rectangle_rec(self.btn_exit, self.btn_exit_color);

                let btn_start_game_padding = Vector2::new(
                    self.btn_start.x + (self.btn_start.width - d.measure_text(BTN_START_TEXT, 64) as f32) / 2.0, 
                    self.btn_start.y + (self.btn_start.height - 64.0) / 2.0
                );
                let btn_settings_game_padding = Vector2::new(
                    self.btn_settings.x + (self.btn_settings.width - d.measure_text(BTN_SETTINGS_TEXT, 64) as f32) / 2.0, 
                    self.btn_settings.y + (self.btn_settings.height - 64.0) / 2.0
                );
                let btn_exit_game_padding = Vector2::new(
                    self.btn_exit.x + (self.btn_exit.width - d.measure_text(BTN_EXIT_TEXT, 64) as f32) / 2.0, 
                    self.btn_exit.y + (self.btn_exit.height - 64.0) / 2.0
                );
                d.draw_text(BTN_START_TEXT, btn_start_game_padding.x as i32, btn_start_game_padding.y as i32, 64, Color::BLACK);
                d.draw_text(BTN_SETTINGS_TEXT, btn_settings_game_padding.x as i32, btn_settings_game_padding.y as i32, 64, Color::BLACK);
                d.draw_text(BTN_EXIT_TEXT, btn_exit_game_padding.x as i32, btn_exit_game_padding.y as i32, 64, Color::BLACK);
            } else if self.state == MenuState::Settings {
                d.draw_rectangle_rec(self.btn_fullscreen, self.btn_fullscreen_color);
                d.draw_rectangle_rec(self.btn_vsync, self.btn_vsync_color);
                d.draw_rectangle_rec(self.btn_back, self.btn_back_color);

                let btn_fullscreen_padding = Vector2::new(
                    self.btn_fullscreen.x + (self.btn_fullscreen.width - d.measure_text(BTN_FULLSCREEN_TEXT, 64) as f32) / 2.0, 
                    self.btn_fullscreen.y + (self.btn_fullscreen.height - 64.0) / 2.0
                );
                let btn_vsync_padding = Vector2::new(
                    self.btn_vsync.x + (self.btn_vsync.width - d.measure_text(BTN_VSYNC_TEXT, 64) as f32) / 2.0, 
                    self.btn_vsync.y + (self.btn_vsync.height - 64.0) / 2.0
                );
                let btn_back_padding = Vector2::new(
                    self.btn_back.x + (self.btn_back.width - d.measure_text(BTN_BACK_TEXT, 64) as f32) / 2.0, 
                    self.btn_back.y + (self.btn_back.height - 64.0) / 2.0
                );
                d.draw_text(BTN_FULLSCREEN_TEXT, btn_fullscreen_padding.x as i32, btn_fullscreen_padding.y as i32, 64, Color::BLACK);
                d.draw_text(BTN_VSYNC_TEXT, btn_vsync_padding.x as i32, btn_vsync_padding.y as i32, 64, Color::BLACK);
                d.draw_text(BTN_BACK_TEXT, btn_back_padding.x as i32, btn_back_padding.y as i32, 64, Color::BLACK);
            }
        }
    }

    pub fn update_btn_positions(&mut self, game: &game::Game) {
        self.btn_start.x = (game.get_window_width() as f32 - 400.0) / 2.0;
        self.btn_start.y = (game.get_window_height() as f32 - 320.0) / 2.0;

        self.btn_settings.x = (game.get_window_width() as f32 - 400.0) / 2.0;
        self.btn_settings.y = (game.get_window_height() as f32 - 80.0) / 2.0;

        self.btn_exit.x = (game.get_window_width() as f32 - 400.0) / 2.0;
        self.btn_exit.y = (game.get_window_height() as f32 + 160.0) / 2.0;

        self.btn_fullscreen.x = (game.get_window_width() as f32 - 400.0) / 2.0;
        self.btn_fullscreen.y = (game.get_window_height() as f32 - 240.0) / 2.0;

        self.btn_back.x = (game.get_window_width() as f32 - 400.0) / 2.0;
        self.btn_back.y = (game.get_window_height() as f32) / 2.0;
    }
}