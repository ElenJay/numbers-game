use raylib::prelude::*;
use raylib::consts::MouseButton::*;

use crate::game;
use crate::level;
use crate::utils::draw_text_center;

const DEFAULT_MENU_ITEM_WIDTH: f32 = 400.0;
const DEFAULT_MENU_ITEM_HEIGHT: f32 = 80.0;
const DEFAULT_MENU_ITEMS_DIFF: f32 = DEFAULT_MENU_ITEM_HEIGHT / 2.0;
const DEFAULT_MENU_ITEM_FONT_SIZE: f32 = 54.0;

const HELP_HOW_TO_PLAY_TEXT: &str = "How to Play:";
const HELP_HOW_TO_PLAY_TEXT_1: &str = "1. Click the numbers from 1 to 56 in order as fast as you can.";
const HELP_HOW_TO_PLAY_TEXT_2: &str = "2. Time's ticking! The game ends when the timer runs out.";
const HELP_HOW_TO_PLAY_TEXT_3: &str = "3. Aim for the highest score: Your score is based on the number of numbers you clicked correctly within the time limit.";
const HELP_TIPS_TEXT: &str = "Tips:";
const HELP_TIPS_TEXT_1: &str = "- Stay focused! Keep your eyes on the numbers and click quickly.";
const HELP_TIPS_TEXT_2: &str = "- Practice makes perfect! The more you play, the faster you'll get.";
const HELP_TIPS_TEXT_3: &str = "- Have fun! This is a challenging and addictive game.";
const HELP_BACK_TEXT: &str = "Press Esc button to go back to the menu...";

#[derive(Clone, Copy, PartialEq)]
enum MenuAllItems {
    Start,
    Continue,
    Settings,
    Help,
    Exit,
    Difficulty,
    Fullscreen,
    ToggleFPS,
    Back,
}

impl MenuAllItems {
    fn value(&self) -> &str {
        match *self {
            Self::Start => "Start",
            Self::Continue => "Continue",
            Self::Settings => "Settings",
            Self::Help => "Help",
            Self::Exit => "Exit",
            Self::Difficulty => "Difficulty",
            Self::Fullscreen => "Fullscreen",
            Self::ToggleFPS => "FPS counter",
            Self::Back => "Back",
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum MenuState {
    Primary,
    Settings,
    Help,
}

pub struct MenuItem {
    btn: Rectangle,
    content: MenuAllItems,
    color: Color,
    is_disabled: bool,
}

pub struct Menu {
    state: MenuState,
    items: Vec<MenuItem>,
    settings_items: Vec<MenuItem>,
}

impl Menu {
    fn construct_menu_items(titles: Vec<MenuAllItems>, window_width: f32, window_height: f32) -> Vec<MenuItem> {
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
                content: *title,
                color: Color::LIGHTGRAY,
                is_disabled: *title == MenuAllItems::Continue,
            });
        }
        items
    }

    pub fn new(game: &game::Game) -> Self {
        let window_width: f32 = game.get_window_width();
        let window_height: f32 = game.get_window_height();

        let items_titles: Vec<MenuAllItems> = vec![
            MenuAllItems::Start, 
            MenuAllItems::Continue,
            MenuAllItems::Settings, 
            MenuAllItems::Help,
            MenuAllItems::Exit, 
        ];
        let settings_items_titles: Vec<MenuAllItems> = vec![
            MenuAllItems::Difficulty,
            MenuAllItems::Fullscreen, 
            MenuAllItems::ToggleFPS, 
            MenuAllItems::Back,
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
        let window_width: f32 = game.get_window_width();
        let window_height: f32 = game.get_window_height();

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
            let mouse_pos: Vector2 = rl.get_mouse_position();

            if self.state == MenuState::Primary {
                for item in self.items.iter_mut() {
                    if item.is_disabled {
                        if item.content == MenuAllItems::Continue && level.is_started() {
                            item.is_disabled = false;
                        } else {
                            continue;
                        }
                    }
                    
                    if item.btn.check_collision_point_rec(mouse_pos) {
                        item.color = Color::LIGHTGREEN;
                        if rl.is_mouse_button_released(MOUSE_BUTTON_LEFT) {
                            match item.content {
                                MenuAllItems::Start => {
                                    game.set_state(game::GameState::Game);
                                    level.start(game);
                                },
                                MenuAllItems::Continue => {
                                    level.resume(game);
                                },
                                MenuAllItems::Settings => {
                                    self.state = MenuState::Settings;
                                },
                                MenuAllItems::Help => {
                                    self.state = MenuState::Help;
                                },
                                MenuAllItems::Exit => {
                                    std::process::exit(0);
                                },
                                _ => {},
                            }
                        }
                    } else if item.color != Color::LIGHTGRAY {
                        item.color = Color::LIGHTGRAY;
                    }
                }
            } else if self.state == MenuState::Settings {
                let mut is_fullscreen_required: bool = false;

                for item in self.settings_items.iter_mut() {
                    if item.btn.check_collision_point_rec(mouse_pos) {
                        item.color = Color::LIGHTGREEN;
                        if rl.is_mouse_button_released(MOUSE_BUTTON_LEFT) {
                            match item.content {
                                MenuAllItems::Difficulty => {
                                    game.change_difficulty(game.get_difficulty());
                                },
                                MenuAllItems::Fullscreen => {
                                    is_fullscreen_required = true;
                                },
                                MenuAllItems::ToggleFPS => {
                                    game.toggle_fps_monitor();
                                },
                                MenuAllItems::Back => {
                                    self.state = MenuState::Primary;
                                },
                                _ => {},
                            }
                        }
                    } else if item.color != Color::LIGHTGRAY {
                        item.color = Color::LIGHTGRAY;
                    }
                }
                if is_fullscreen_required {
                    game.toggle_fullscreen(rl, self, level);
                }
            }
        }
    }

    fn draw_menu_button(&self, d: &mut RaylibDrawHandle, game: &game::Game, btn: &Rectangle, btn_text: &str, btn_color: &Color) {
        d.draw_rectangle_rec(btn, btn_color);
        let btn_text_sizes: Vector2 = game.get_font().measure_text(btn_text, DEFAULT_MENU_ITEM_FONT_SIZE, game.get_font_spacing());
        let btn_padding: Vector2 = Vector2 {
            x: btn.x + (btn.width - btn_text_sizes.x) / 2.0, 
            y: btn.y + (btn.height - btn_text_sizes.y) / 2.0
        };
        d.draw_text_ex(game.get_font(), btn_text, btn_padding, DEFAULT_MENU_ITEM_FONT_SIZE, game.get_font_spacing(), Color::BLACK);
    }

    fn draw_menu(&self, d: &mut RaylibDrawHandle, game: &game::Game) {
        if game.get_state() == game::GameState::Menu {
            if self.state == MenuState::Primary {
                for item in self.items.iter() {
                    self.draw_menu_button(d, &game, &item.btn, item.content.value(), &item.color);
                }
            } else if self.state == MenuState::Settings {
                for item in self.settings_items.iter() {
                    self.draw_menu_button(d, &game, &item.btn, item.content.value(), &item.color);
                }
                let game_difficulty_text: String = format!("Your current game difficulty is: {}", game.get_difficulty());
                draw_text_center(d, game_difficulty_text.as_str(), game.get_window_height() - 60.0, 40.0, Color::GREEN, &game)
            } else if self.state == MenuState::Help {
                let text_sizes: Vector2 = game.get_font().measure_text(HELP_HOW_TO_PLAY_TEXT_3, 20.0, game.get_font_spacing());
                let x: f32 = (game.get_window_width() - text_sizes.x) / 2.0;
                let mut y: f32 = (game.get_window_height() - (100.0 + 64.0 * 3.0 + 36.0 * 4.0 + 32.0)) / 2.0;

                d.draw_text_ex(game.get_font(), HELP_HOW_TO_PLAY_TEXT, Vector2 {x: x, y: y}, 32.0, game.get_font_spacing(), Color::BLACK);
                y += 64.0;
                d.draw_text_ex(game.get_font(), HELP_HOW_TO_PLAY_TEXT_1, Vector2 {x: x, y: y}, 24.0, game.get_font_spacing(), Color::BLACK);
                y += 36.0;
                d.draw_text_ex(game.get_font(), HELP_HOW_TO_PLAY_TEXT_2, Vector2 {x: x, y: y}, 24.0, game.get_font_spacing(), Color::BLACK);
                y += 36.0;
                d.draw_text_ex(game.get_font(), HELP_HOW_TO_PLAY_TEXT_3, Vector2 {x: x, y: y}, 24.0, game.get_font_spacing(), Color::BLACK);
                y += 64.0;
                d.draw_text_ex(game.get_font(), HELP_TIPS_TEXT, Vector2 {x: x, y: y}, 32.0, game.get_font_spacing(), Color::BLACK);
                y += 64.0;
                d.draw_text_ex(game.get_font(), HELP_TIPS_TEXT_1, Vector2 {x: x, y: y}, 24.0, game.get_font_spacing(), Color::BLACK);
                y += 36.0;
                d.draw_text_ex(game.get_font(), HELP_TIPS_TEXT_2, Vector2 {x: x, y: y}, 24.0, game.get_font_spacing(), Color::BLACK);
                y += 36.0;
                d.draw_text_ex(game.get_font(), HELP_TIPS_TEXT_3, Vector2 {x: x, y: y}, 24.0, game.get_font_spacing(), Color::BLACK);
                y += 100.0;
                d.draw_text_ex(game.get_font(), HELP_BACK_TEXT, Vector2 {x: x, y: y}, 32.0, game.get_font_spacing(), Color::BLACK);
            }
        }
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle, game: &game::Game) {
        self.draw_menu(d, game);
    }
}