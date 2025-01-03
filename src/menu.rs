use raylib::prelude::*;
use raylib::consts::MouseButton::*;

use crate::game;
use crate::level;
use crate::utils::draw_text_center;

const DEFAULT_MENU_ITEM_WIDTH: f32 = 400.0;
const DEFAULT_MENU_ITEM_HEIGHT: f32 = 80.0;
const DEFAULT_MENU_ITEMS_DIFF: f32 = DEFAULT_MENU_ITEM_HEIGHT / 2.0;
const DEFAULT_MENU_ITEM_FONT_SIZE: i32 = 54;

const BTN_START_TEXT: &str = "Start";
const BTN_CONTINUE_TEXT: &str = "Continue";
const BTN_SETTINGS_TEXT: &str = "Settings";
const BTN_HELP_TEXT: &str = "Help";
const BTN_EXIT_TEXT: &str = "Exit";
const BTN_DIFFICULTY_TEXT: &str = "Difficulty";
const BTN_FULLSCREEN_TEXT: &str = "Fullscreen";
const BTN_TOGGLE_FPS_TEXT: &str = "FPS counter";
const BTN_BACK_TEXT: &str = "Back";

const HELP_HOW_TO_PLAY_TEXT: &str = "How to Play:";
const HELP_HOW_TO_PLAY_TEXT_1: &str = "1. Click the numbers from 1 to 56 in order as fast as you can.";
const HELP_HOW_TO_PLAY_TEXT_2: &str = "2. Time's ticking! The game ends when the timer runs out.";
const HELP_HOW_TO_PLAY_TEXT_3: &str = "3. Aim for the highest score: Your score is based on the number of numbers you clicked correctly within the time limit.";
const HELP_TIPS_TEXT: &str = "Tips:";
const HELP_TIPS_TEXT_1: &str = "- Stay focused! Keep your eyes on the numbers and click quickly.";
const HELP_TIPS_TEXT_2: &str = "- Practice makes perfect! The more you play, the faster you'll get.";
const HELP_TIPS_TEXT_3: &str = "- Have fun! This is a challenging and addictive game.";
const HELP_BACK_TEXT: &str = "Press Esc button to go back to the menu...";

// ToDo: separate menu items and implement enum iterator
// enum MENU_ITEMS_TEXT {
//     BTN_START_TEXT,
//     BTN_CONTINUE_TEXT,
//     BTN_SETTINGS_TEXT,
//     BTN_HELP_TEXT,
//     BTN_EXIT_TEXT,
// }

// enum MENU_SETTINGS_ITEMS_TEXT {
//     BTN_DIFFICULTY_TEXT,
//     BTN_FULLSCREEN_TEXT,
//     BTN_BACK_TEXT,
// }

#[derive(Clone, Copy, PartialEq)]
pub enum MenuState {
    Primary,
    Settings,
    Help,
}

pub struct MenuItem {
    btn: Rectangle,
    content: String,
    color: Color,
    is_disabled: bool,
}

pub struct Menu {
    state: MenuState,
    items: Vec<MenuItem>,
    settings_items: Vec<MenuItem>,
}

impl Menu {
    fn construct_menu_items(titles: Vec<String>, window_width: f32, window_height: f32) -> Vec<MenuItem> {
        let mut items: Vec<MenuItem> = Vec::with_capacity(titles.len());
        let all_items_height: f32 = titles.len() as f32 * DEFAULT_MENU_ITEM_HEIGHT + (titles.len() - 1) as f32 * DEFAULT_MENU_ITEMS_DIFF;
        for (index, title) in titles.iter().enumerate() {
            items.push(MenuItem {
                btn: Rectangle {
                    x: (window_width as f32 - DEFAULT_MENU_ITEM_WIDTH) / 2.0, 
                    y: (window_height as f32 - all_items_height) / 2.0 + index as f32 * (DEFAULT_MENU_ITEM_HEIGHT + DEFAULT_MENU_ITEMS_DIFF), 
                    width: DEFAULT_MENU_ITEM_WIDTH, 
                    height: DEFAULT_MENU_ITEM_HEIGHT,
                },
                content: title.clone(),
                color: Color::LIGHTGRAY,
                is_disabled: title == BTN_CONTINUE_TEXT,
            });
        }
        items
    }

    pub fn new(game: &game::Game) -> Self {
        let window_width: f32 = game.get_window_width() as f32;
        let window_height: f32 = game.get_window_height() as f32;

        let items_titles: Vec<String> = vec![
            BTN_START_TEXT.to_string(), 
            BTN_CONTINUE_TEXT.to_string(),
            BTN_SETTINGS_TEXT.to_string(), 
            BTN_HELP_TEXT.to_string(),
            BTN_EXIT_TEXT.to_string(), 
        ];
        let settings_items_titles: Vec<String> = vec![
            BTN_DIFFICULTY_TEXT.to_string(),
            BTN_FULLSCREEN_TEXT.to_string(), 
            BTN_TOGGLE_FPS_TEXT.to_string(), 
            BTN_BACK_TEXT.to_string(),
        ];

        Self {
            state: MenuState::Primary,
            items: Self::construct_menu_items(items_titles, window_width, window_height),
            settings_items: Self::construct_menu_items(settings_items_titles, window_width, window_height),
        }
    }

    pub fn get_state(&self) -> MenuState {
        self.state
    }

    pub fn set_state(&mut self, state: MenuState) {
        self.state = state;
    }

    pub fn update_btn_positions(&mut self, game: &game::Game) {
        let window_width: f32 = game.get_window_width() as f32;
        let window_height: f32 = game.get_window_height() as f32;

        let items_length: f32 = self.items.len() as f32;
        let settings_items_length: f32 = self.settings_items.len() as f32;

        let all_items_height: f32 = items_length * DEFAULT_MENU_ITEM_HEIGHT + (items_length - 1.0) * DEFAULT_MENU_ITEMS_DIFF;
        let all_settings_items_height: f32 = settings_items_length * DEFAULT_MENU_ITEM_HEIGHT + (settings_items_length - 1.0) * DEFAULT_MENU_ITEMS_DIFF;

        for (index, item) in self.items.iter_mut().enumerate() {
            item.btn.x = (window_width - DEFAULT_MENU_ITEM_WIDTH) / 2.0;
            item.btn.y = (window_height - all_items_height) / 2.0 + index as f32 * (DEFAULT_MENU_ITEM_HEIGHT + DEFAULT_MENU_ITEMS_DIFF);
        }

        for (index, item) in self.settings_items.iter_mut().enumerate() {
            item.btn.x = (window_width - DEFAULT_MENU_ITEM_WIDTH) / 2.0;
            item.btn.y = (window_height - all_settings_items_height) / 2.0 + index as f32 * (DEFAULT_MENU_ITEM_HEIGHT + DEFAULT_MENU_ITEMS_DIFF);
        }
    }

    pub fn process_controller(&mut self, rl: &mut RaylibHandle, game: &mut game::Game, level: &mut level::Level) {
        if game.get_state() == game::GameState::Menu {
            let mouse_pos = rl.get_mouse_position();

            if self.state == MenuState::Primary {
                for item in self.items.iter_mut() {
                    if item.is_disabled {
                        if item.content.as_str() == BTN_CONTINUE_TEXT && level.is_started() {
                            item.is_disabled = false;
                        } else {
                            continue;
                        }
                    }
                    
                    if item.btn.check_collision_point_rec(mouse_pos) {
                        item.color = Color::LIGHTGREEN;
                        if rl.is_mouse_button_released(MOUSE_BUTTON_LEFT) {
                            match item.content.as_str() {
                                BTN_START_TEXT => {
                                    game.set_state(game::GameState::Game);
                                    level.restart(game);
                                },
                                BTN_CONTINUE_TEXT => {
                                    level.resume(game);
                                },
                                BTN_SETTINGS_TEXT => {
                                    self.state = MenuState::Settings;
                                },
                                BTN_HELP_TEXT => {
                                    self.state = MenuState::Help;
                                },
                                BTN_EXIT_TEXT => {
                                    std::process::exit(0);
                                },
                                _ => {},
                            }
                        }
                    } else {
                        item.color = Color::LIGHTGRAY;
                    }
                }
            } else if self.state == MenuState::Settings {
                let mut is_fullscreen_required: bool = false;

                for item in self.settings_items.iter_mut() {
                    if item.btn.check_collision_point_rec(mouse_pos) {
                        item.color = Color::LIGHTGREEN;
                        if rl.is_mouse_button_released(MOUSE_BUTTON_LEFT) {
                            match item.content.as_str() {
                                BTN_DIFFICULTY_TEXT => {
                                    let difficulty: game::GameDifficulty = game.get_difficulty();
                                    if difficulty == game::GameDifficulty::Easy {
                                        game.set_difficulty(game::GameDifficulty::Hard);
                                    } else {
                                        game.set_difficulty(game::GameDifficulty::Easy);
                                    }
                                },
                                BTN_FULLSCREEN_TEXT => {
                                    is_fullscreen_required = true;
                                },
                                BTN_BACK_TEXT => {
                                    self.state = MenuState::Primary;
                                },
                                BTN_TOGGLE_FPS_TEXT => {
                                    game.toggle_fps_monitor();
                                },
                                _ => {},
                            }
                        }
                    } else {
                        item.color = Color::LIGHTGRAY;
                    }
                }
                if is_fullscreen_required {
                    game.toggle_fullscreen(rl, self, level);
                }
            }
        }
    }

    fn draw_menu_button(&self, d: &mut RaylibDrawHandle, btn: &Rectangle, btn_text: &str, btn_color: &Color) {
        d.draw_rectangle_rec(btn, btn_color);
        let btn_padding = Vector2::new(
            btn.x + (btn.width - d.measure_text(btn_text, DEFAULT_MENU_ITEM_FONT_SIZE) as f32) / 2.0, 
            btn.y + (btn.height - DEFAULT_MENU_ITEM_FONT_SIZE as f32) / 2.0
        );
        d.draw_text(btn_text, btn_padding.x as i32, btn_padding.y as i32, DEFAULT_MENU_ITEM_FONT_SIZE, Color::BLACK);
    }

    fn draw_menu(&self, d: &mut RaylibDrawHandle, game: &game::Game) {
        if game.get_state() == game::GameState::Menu {
            if self.state == MenuState::Primary {
                for item in self.items.iter() {
                    self.draw_menu_button(d, &item.btn, item.content.as_str(), &item.color);
                }
            } else if self.state == MenuState::Settings {
                for item in self.settings_items.iter() {
                    self.draw_menu_button(d, &item.btn, item.content.as_str(), &item.color);
                }
                let game_difficulty_text: String = format!("Your current game difficulty is: {}", game.get_difficulty());
                draw_text_center(d, game_difficulty_text.as_str(), game.get_window_height() - 60, 40, Color::GREEN, &game)
            } else if self.state == MenuState::Help {
                let text_length = d.measure_text(HELP_HOW_TO_PLAY_TEXT_3, 20);
                let x: i32 = (game.get_window_width() - text_length) / 2;
                let mut y: i32 = (game.get_window_height() - (100 + 64 * 3 + 36 * 4 + 32)) / 2;

                d.draw_text(HELP_HOW_TO_PLAY_TEXT, x, y, 32, Color::BLACK);
                y += 64;
                d.draw_text(HELP_HOW_TO_PLAY_TEXT_1, x, y, 24, Color::BLACK);
                y += 36;
                d.draw_text(HELP_HOW_TO_PLAY_TEXT_2, x, y, 24, Color::BLACK);
                y += 36;
                d.draw_text(HELP_HOW_TO_PLAY_TEXT_3, x, y, 24, Color::BLACK);
                y += 64;
                d.draw_text(HELP_TIPS_TEXT, x, y, 32, Color::BLACK);
                y += 64;
                d.draw_text(HELP_TIPS_TEXT_1, x, y, 24, Color::BLACK);
                y += 36;
                d.draw_text(HELP_TIPS_TEXT_2, x, y, 24, Color::BLACK);
                y += 36;
                d.draw_text(HELP_TIPS_TEXT_3, x, y, 24, Color::BLACK);
                y += 100;
                d.draw_text(HELP_BACK_TEXT, x, y, 32, Color::BLACK);
            }
        }
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle, game: &game::Game) {
        self.draw_menu(d, game);
    }
}