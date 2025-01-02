use raylib::prelude::*;
use raylib::consts::KeyboardKey::*;

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
}

impl Game {
    pub fn new() -> Self {
        Self {
            mode: GameMode::Release,
            state: GameState::Menu,
            settings: GameSettings {
                is_fullscreen: false,
                is_vsync: true,
                is_fps_visible: false,
            },
            window_width: WINDOW_WIDTH,
            window_height: WINDOW_HEIGHT,
        }
    }

    pub fn get_state(&self) -> GameState {
        self.state
    }

    pub fn set_state(&mut self, state: GameState) {
        self.state = state;
    }

    pub fn get_settings(&self) -> &GameSettings {
        &self.settings
    }

    pub fn toggle_fps_monitor(&mut self) {
        self.settings.is_fps_visible = !self.settings.is_fps_visible;
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

    pub fn process_game_controller(&mut self, rl: &mut RaylibHandle) {
        rl.set_exit_key(None);

        if rl.is_key_released(KEY_F1) {
            rl.toggle_fullscreen();
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