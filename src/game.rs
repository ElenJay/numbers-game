use raylib::prelude::*;
use raylib::consts::KeyboardKey::*;

use crate::menu::Menu;

const WINDOW_WIDTH: i32 = 1600;
const WINDOW_HEIGHT: i32 = 900;

#[derive(Clone, Copy, PartialEq)]
pub enum GameMode {
    // Debug,
    Release,
}

#[derive(Clone, Copy, PartialEq)]
pub enum GameState {
    Menu,
    Game,
    Win,
    Lose,
}

#[derive(Clone, Copy, PartialEq)]
pub struct GameSettings {
    is_fullscreen: bool,
    is_vsync: bool,
    is_fps_visible: bool,
}

pub struct Game {
    mode: GameMode,
    state: GameState,
    settings: GameSettings,
    window_width: i32,
    window_height: i32,
    fullscreen_width: i32,
    fullscreen_height: i32,
}

impl Game {
    pub fn new() -> Self {
        Self {
            mode: GameMode::Release,
            state: GameState::Menu,
            settings: GameSettings {
                is_fullscreen: true,
                is_vsync: true,
                is_fps_visible: false,
            },
            window_width: WINDOW_WIDTH,
            window_height: WINDOW_HEIGHT,
            fullscreen_width: 0,
            fullscreen_height: 0,
        }
    }

    pub fn get_state(&self) -> GameState {
        self.state
    }

    pub fn set_state(&mut self, state: GameState) {
        self.state = state;
    }

    pub fn get_window_width(&self) -> i32 {
        self.window_width
    }

    pub fn set_window_width(&mut self, width: i32) {
        self.window_width = width;
    }

    pub fn get_window_height(&self) -> i32 {
        self.window_height
    }

    pub fn set_window_height(&mut self, height: i32) {
        self.window_height = height;
    }

    pub fn set_fullscreen_sizes(&mut self, width: i32, height: i32) {
        self.fullscreen_width = width;
        self.fullscreen_height = height;
    }

    pub fn toggle_fps_monitor(&mut self) {
        self.settings.is_fps_visible = !self.settings.is_fps_visible;
    }

    pub fn toggle_fullscreen(&mut self, rl: &mut RaylibHandle, menu: &mut Menu) {
        self.settings.is_fullscreen = !self.settings.is_fullscreen;

        // Toggling fullscreen with borderless window mode requires order
        if self.settings.is_fullscreen {
            rl.toggle_borderless_windowed();
            rl.set_window_size(self.fullscreen_width, self.fullscreen_height);
            rl.toggle_fullscreen();
        } else {
            rl.toggle_fullscreen();
            rl.toggle_borderless_windowed();
        }

        // Update window sizes in the Game object
        self.set_window_width(rl.get_screen_width());
        self.set_window_height(rl.get_screen_height());

        // Recalculate menu buttons positions
        menu.update_btn_positions(self);
    }

    pub fn process_game_controller(&mut self, rl: &mut RaylibHandle, menu: &mut Menu) {
        rl.set_exit_key(None);

        if rl.is_key_released(KEY_F1) {
            self.toggle_fullscreen(rl, menu);
        }

        if rl.is_key_released(KEY_ESCAPE) {
            match self.state {
                GameState::Game | GameState::Win | GameState::Lose => self.state = GameState::Menu,
                GameState::Menu => std::process::exit(0),
            }
        }
    }

    pub fn draw_fps(&self, d: &mut RaylibDrawHandle) {
        if self.settings.is_fps_visible {
            d.draw_fps( 10, 10);
        }
    }
}