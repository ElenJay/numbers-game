use raylib::prelude::*;
use raylib::consts::KeyboardKey::*;

pub const WINDOW_WIDTH: i32 = 1280;
pub const WINDOW_HEIGHT: i32 = 720;

#[derive(Clone, Copy, PartialEq)]
pub enum GameMode {
    Debug,
    // Release,
}

#[derive(Clone, Copy, PartialEq)]
pub enum GameState {
    Menu,
    Game,
}

pub struct Game {
    mode: GameMode,
    state: GameState,
    is_vsync_enabled: bool,
}

impl Game {
    pub fn new() -> Game {
        Game {
            mode: GameMode::Debug,
            state: GameState::Menu,
            is_vsync_enabled: false,
        }
    }

    pub fn process_game_controller(&self, rl: &mut RaylibHandle) {
        // Base hotkeys
        if rl.is_key_released(KEY_F1) {
            rl.toggle_fullscreen();
        }
    }

    pub fn get_state(&self) -> GameState {
        self.state
    }

    pub fn set_state(&mut self, state: GameState) {
        self.state = state;
    }

    pub fn get_mode(&self) -> GameMode {
        self.mode
    }

    pub fn get_vsync_enabled(&self) -> bool {
        self.is_vsync_enabled
    }

    pub fn toggle_vsync_enabled(&mut self) {
        self.is_vsync_enabled = !self.is_vsync_enabled;
    }
}