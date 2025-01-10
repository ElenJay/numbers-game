#![allow(dead_code)]

use raylib::prelude::*;
use raylib::consts::KeyboardKey::*;

use crate::menu::{ Menu, MenuState };
use crate::level::Level;

#[derive(Clone, Copy, PartialEq)]
pub enum GameMode {
    Debug,
    Release,
}

#[derive(Clone, Copy, PartialEq)]
pub enum GameState {
    Menu,
    Game,
    Win,
    Lose,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GameDifficulty {
    Easy,
    Medium,
    Hard,
}

impl std::fmt::Display for GameDifficulty {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Clone, Copy, PartialEq)]
pub struct GameSettings {
    is_fullscreen: bool,
    is_vsync: bool,
    is_fps_visible: bool,
}

pub struct GameFont {
    font: Font, 
    spacing: f32,
}

pub struct Game {
    mode: GameMode,
    state: GameState,
    difficulty: GameDifficulty,
    settings: GameSettings,
    game_font: GameFont,
    window_width: f32,
    window_height: f32,
    fullscreen_width: i32,
    fullscreen_height: i32,
}

impl Game {
    pub const DEFAULT_WINDOW_WIDTH: i32 = 1600;
    pub const DEFAULT_WINDOW_HEIGHT: i32 = 900;
    pub const CUSTOM_FONT_PATH: &str = "assets/fonts/Arimo-Regular.ttf";

    pub fn new(rl: &mut RaylibHandle, thread: &RaylibThread, mode: GameMode) -> Self {
        let english_alphabet: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
        let cyrillic_alphabet: &str = "абвгдеєжзиіЇйклмнопрстуфхцчшщьюяАБВГДЕЄЖЗИІЇЙКЛМНОПРСТУФХЦЧШЩЬЮЯ";
        let ascii_symbols: &str = "1234567890 !@#$%^&*()_+-=[]{};':\",.<>/?`~";
        let alphabet: String = format!("{}{}{}", ascii_symbols, english_alphabet, cyrillic_alphabet);

        let spacing: f32 = match std::fs::File::open(Self::CUSTOM_FONT_PATH) {
            Ok(_) => 1.0,
            Err(_) => 5.0,
        };

        Self {
            mode: mode,
            state: GameState::Menu,
            difficulty: GameDifficulty::Easy,
            settings: GameSettings {
                is_fullscreen: true,
                is_vsync: true,
                is_fps_visible: false,
            },
            game_font: GameFont {
                font: rl.load_font_ex(&thread, Self::CUSTOM_FONT_PATH, 200, Some(alphabet.as_str())).unwrap(), 
                spacing: spacing 
            },
            window_width: Self::DEFAULT_WINDOW_WIDTH as f32,
            window_height: Self::DEFAULT_WINDOW_HEIGHT as f32,
            fullscreen_width: 0,
            fullscreen_height: 0,
        }
    }

    pub fn get_mode(&self) -> GameMode {
        self.mode
    }

    pub fn get_state(&self) -> GameState {
        self.state
    }

    pub fn set_state(&mut self, state: GameState) {
        self.state = state;
    }

    pub fn get_difficulty(&self) -> GameDifficulty {
        self.difficulty
    }

    pub fn change_difficulty(&mut self, difficulty: GameDifficulty) {
        match difficulty {
            GameDifficulty::Easy => self.difficulty = GameDifficulty::Medium,
            GameDifficulty::Medium => self.difficulty = GameDifficulty::Hard,
            GameDifficulty::Hard => self.difficulty = GameDifficulty::Easy,
        }
    }

    pub fn get_font(&self) -> &Font {
        &self.game_font.font
    }

    pub fn get_font_spacing(&self) -> f32 {
        self.game_font.spacing
    }

    pub fn get_window_width(&self) -> f32 {
        self.window_width
    }

    pub fn get_window_height(&self) -> f32 {
        self.window_height
    }

    pub fn set_window_sizes(&mut self, width: i32, height: i32) {
        self.window_width = width as f32;
        self.window_height = height as f32;
    }

    pub fn set_fullscreen_sizes(&mut self, width: i32, height: i32) {
        self.fullscreen_width = width;
        self.fullscreen_height = height;
    }

    pub fn toggle_fps_monitor(&mut self) {
        self.settings.is_fps_visible = !self.settings.is_fps_visible;
    }

    pub fn toggle_fullscreen(&mut self, rl: &mut RaylibHandle, menu: &mut Menu, level: &mut Level) {
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
        self.set_window_sizes(rl.get_screen_width(), rl.get_screen_height());

        // Recalculate menu buttons positions
        menu.update_btn_positions(self);
        level.update_btn_positions(self);
    }

    pub fn process_controller(&mut self, rl: &mut RaylibHandle, menu: &mut Menu, level: &mut Level) {
        rl.set_exit_key(None);

        if rl.is_window_resized() {
            self.set_window_sizes(rl.get_screen_width(), rl.get_screen_height());
            menu.update_btn_positions(self);
            level.update_btn_positions(self);
        }

        if rl.is_key_released(KEY_F1) {
            self.toggle_fullscreen(rl, menu, level);
        }

        if rl.is_key_released(KEY_ESCAPE) {
            if menu.get_state() == MenuState::Help {
                menu.set_state(MenuState::Primary);
            } else {
                match self.state {
                    GameState::Game | GameState::Win | GameState::Lose => self.state = GameState::Menu,
                    GameState::Menu => std::process::exit(0),
                }
            }
        }
    }

    fn draw_fps(&self, d: &mut RaylibDrawHandle) {
        if self.settings.is_fps_visible {
            d.draw_fps( 10, 10);
        }
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        self.draw_fps(d);
    }
}