use raylib::prelude::*;
use raylib::consts::KeyboardKey::*;

const WINDOW_WIDTH: i32 = 1600;
const WINDOW_HEIGHT: i32 = 900;

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
    window_width: i32,
    window_height: i32,
}

impl Game {
    pub fn new() -> Self {
        Self {
            mode: GameMode::Debug,
            state: GameState::Menu,
            window_width: WINDOW_WIDTH,
            window_height: WINDOW_HEIGHT,
        }
    }

    pub fn process_game_controller(&mut self, rl: &mut RaylibHandle) {
        rl.set_exit_key(None);

        if rl.is_key_released(KEY_F1) {
            rl.toggle_fullscreen();
        }
        if self.state == GameState::Game && rl.is_key_released(KEY_ESCAPE) {
            self.state = GameState::Menu;
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
}