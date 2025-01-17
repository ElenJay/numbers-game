use raylib::prelude::*;
use raylib::consts::MouseButton::*;

use crate::game;
use crate::level;

const DEFAULT_MENU_ITEM_WIDTH: f32 = 400.0;
const DEFAULT_MENU_ITEM_HEIGHT: f32 = 80.0;
const DEFAULT_MENU_ITEMS_DIFF: f32 = DEFAULT_MENU_ITEM_HEIGHT / 2.0;
const DEFAULT_MENU_ITEM_FONT_SIZE: f32 = 54.0;

struct HelpTextRow {
    content: String,
    font_size: u8,
    padding_bottom: u16,
}

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
            Self::Back => "Back",
            _ => "",
        }
    }

    fn description(&self) -> &str {
        match *self {
            Self::Difficulty => "Game difficulty",
            Self::Fullscreen => "Fullscreen mode",
            Self::ToggleFPS => "FPS counter on the screen",
            _ => "",
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
    title: MenuAllItems,
    color: Color,
    description: Option<String>,
    description_pos: Option<Vector2>,
}

pub struct Menu {
    state: MenuState,
    items: Vec<MenuItem>,
    settings_items: Vec<MenuItem>,
    help_rows: [HelpTextRow; 9],
}

impl Menu {
    const PRIMARY_ITEMS: [MenuAllItems; 4] = [
        MenuAllItems::Start,
        MenuAllItems::Settings, 
        MenuAllItems::Help,
        MenuAllItems::Exit, 
    ];
    const FULL_PRIMARY_ITEMS: [MenuAllItems; 5] = [
        MenuAllItems::Start,
        MenuAllItems::Continue,
        MenuAllItems::Settings, 
        MenuAllItems::Help,
        MenuAllItems::Exit, 
    ];
    const SETTINGS_ITEMS: [MenuAllItems; 4] = [
        MenuAllItems::Difficulty,
        MenuAllItems::Fullscreen, 
        MenuAllItems::ToggleFPS, 
        MenuAllItems::Back,
    ];

    fn construct_menu_items(menu_items: &[MenuAllItems], window_width: f32, window_height: f32) -> Vec<MenuItem> {
        let mut menu_item: MenuItem;
        let mut items: Vec<MenuItem> = Vec::with_capacity(menu_items.len());
        let all_items_height: f32 = menu_items.len() as f32 * DEFAULT_MENU_ITEM_HEIGHT + (menu_items.len() - 1) as f32 * DEFAULT_MENU_ITEMS_DIFF;
        
        for (index, item) in menu_items.iter().enumerate() {
            if item.description() == "" {
                menu_item = MenuItem {
                    btn: Rectangle {
                        x: (window_width - DEFAULT_MENU_ITEM_WIDTH) / 2.0, 
                        y: (window_height - all_items_height) / 2.0 + index as f32 * (DEFAULT_MENU_ITEM_HEIGHT + DEFAULT_MENU_ITEMS_DIFF), 
                        width: DEFAULT_MENU_ITEM_WIDTH, 
                        height: DEFAULT_MENU_ITEM_HEIGHT,
                    },
                    title: *item,
                    color: Color::LIGHTGRAY,
                    description: None,
                    description_pos: None
                };
            } else {
                menu_item = MenuItem {
                    btn: Rectangle {
                        x: window_width / 8.0 * 7.0 - DEFAULT_MENU_ITEM_WIDTH, 
                        y: (window_height - all_items_height) / 2.0 + index as f32 * (DEFAULT_MENU_ITEM_HEIGHT + DEFAULT_MENU_ITEMS_DIFF), 
                        width: DEFAULT_MENU_ITEM_WIDTH, 
                        height: DEFAULT_MENU_ITEM_HEIGHT,
                    },
                    title: *item,
                    color: Color::LIGHTGRAY,
                    description: Some(item.description().to_string()),
                    description_pos: Some(Vector2 {
                        x: window_width / 8.0,
                        y: (window_height as f32 - all_items_height) / 2.0 + index as f32 * (DEFAULT_MENU_ITEM_HEIGHT + DEFAULT_MENU_ITEMS_DIFF) + (DEFAULT_MENU_ITEM_HEIGHT - DEFAULT_MENU_ITEM_FONT_SIZE) / 2.0,
                    }),
                };
            }
            items.push(menu_item);
        }

        items
    }

    pub fn new(game: &game::Game) -> Self {
        let window_width: f32 = game.get_window_width();
        let window_height: f32 = game.get_window_height();

        Self {
            state: MenuState::Primary,
            items: Self::construct_menu_items(&Self::PRIMARY_ITEMS, window_width, window_height),
            settings_items: Self::construct_menu_items(&Self::SETTINGS_ITEMS, window_width, window_height),
            help_rows: [
                HelpTextRow { font_size: 32, padding_bottom: 64, content: "How to Play:".to_string(), },
                HelpTextRow { font_size: 24, padding_bottom: 36, content: "1. Click the numbers from 1 to 56 in order as fast as you can.".to_string(), },
                HelpTextRow { font_size: 24, padding_bottom: 36, content: "2. Time's ticking! The game ends when the timer runs out.".to_string(), },
                HelpTextRow { font_size: 24, padding_bottom: 64, content: "3. Aim for the highest score: Your score is based on the number of numbers you clicked correctly within the time limit.".to_string(), },
                HelpTextRow { font_size: 32, padding_bottom: 64, content: "Tips:".to_string(), },
                HelpTextRow { font_size: 24, padding_bottom: 36, content: "- Stay focused! Keep your eyes on the numbers and click quickly.".to_string(), },
                HelpTextRow { font_size: 24, padding_bottom: 36, content: "- Practice makes perfect! The more you play, the faster you'll get.".to_string(), },
                HelpTextRow { font_size: 24, padding_bottom: 100, content: "- Have fun! This is a challenging and addictive game.".to_string(), },
                HelpTextRow { font_size: 32, padding_bottom: 0, content: "Press Esc button to go back to the menu...".to_string(), },
            ],
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
            item.btn.y = (window_height - all_settings_items_height) / 2.0 + index as f32 * (DEFAULT_MENU_ITEM_HEIGHT + DEFAULT_MENU_ITEMS_DIFF);

            if item.title.value() == "" {
                item.btn.x = window_width / 8.0 * 7.0 - DEFAULT_MENU_ITEM_WIDTH;
                item.description_pos = Some(Vector2 {
                    x: window_width / 8.0,
                    y: item.btn.y + (DEFAULT_MENU_ITEM_HEIGHT - DEFAULT_MENU_ITEM_FONT_SIZE) / 2.0,
                });
            } else {
                item.btn.x = (window_width - DEFAULT_MENU_ITEM_WIDTH) / 2.0;
            }
        }
    }

    pub fn process_controller(&mut self, rl: &mut RaylibHandle, game: &mut game::Game, level: &mut level::Level) {
        if game.get_state() == game::GameState::Menu {
            let mouse_pos: Vector2 = rl.get_mouse_position();

            if self.state == MenuState::Primary {
                let mut has_primary_menu_to_be_updated: bool = false;
                let items_count: usize = self.items.len();

                if items_count == 5 && level.is_over() {
                    self.update_primary_menu(game, 4);
                }

                for item in self.items.iter_mut() {
                    if item.btn.check_collision_point_rec(mouse_pos) {
                        item.color = Color::LIGHTGREEN;
                        if rl.is_mouse_button_released(MOUSE_BUTTON_LEFT) {
                            match item.title {
                                MenuAllItems::Start => {
                                    game.set_state(game::GameState::Game);
                                    level.start(game);
                                    has_primary_menu_to_be_updated = items_count == 4;
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

                if has_primary_menu_to_be_updated {
                    self.update_primary_menu(game, 5);
                }
            } else if self.state == MenuState::Settings {
                let mut is_fullscreen_required: bool = false;

                for item in self.settings_items.iter_mut() {
                    if item.btn.check_collision_point_rec(mouse_pos) {
                        item.color = Color::LIGHTGREEN;
                        if rl.is_mouse_button_released(MOUSE_BUTTON_LEFT) {
                            match item.title {
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

    fn draw_menu_button(&self, d: &mut RaylibDrawHandle, game: &game::Game, menu_item: &MenuItem) {
        // Draw item button
        let game_difficulty: game::GameDifficulty = game.get_difficulty();
        let item_btn_title: &str = match &menu_item.title {
            MenuAllItems::Difficulty => game_difficulty.repr(),
            MenuAllItems::Fullscreen => if game.get_settings().is_fullscreen { "Disable" } else { "Enable" },
            MenuAllItems::ToggleFPS => if game.get_settings().is_fps_visible { "Disable" } else { "Enable" },
            _ => menu_item.title.value(),
        };

        d.draw_rectangle_rec(menu_item.btn, menu_item.color);
        let btn_text_sizes: Vector2 = game.get_font().measure_text(item_btn_title, DEFAULT_MENU_ITEM_FONT_SIZE, game.get_font_spacing());
        let btn_padding: Vector2 = Vector2 {
            x: menu_item.btn.x + (menu_item.btn.width - btn_text_sizes.x) / 2.0, 
            y: menu_item.btn.y + (menu_item.btn.height - btn_text_sizes.y) / 2.0
        };
        d.draw_text_ex(game.get_font(), item_btn_title, btn_padding, DEFAULT_MENU_ITEM_FONT_SIZE, game.get_font_spacing(), Color::BLACK);
        
        // Draw item description
        match &menu_item.description {
            Some(desc) => d.draw_text_ex(game.get_font(), desc.as_str(), menu_item.description_pos.unwrap(), DEFAULT_MENU_ITEM_FONT_SIZE, game.get_font_spacing(), Color::BLACK),
            _ => {},
        }
    }

    fn draw_help_menu(&self, d: &mut RaylibDrawHandle, game: &game::Game) {
        let text_sizes: Vector2 = game.get_font().measure_text(&self.help_rows[3].content, 20.0, game.get_font_spacing());
        let x: f32 = (game.get_window_width() - text_sizes.x) / 2.0;
        let mut y: f32 = (game.get_window_height() - (100.0 + 64.0 * 3.0 + 36.0 * 4.0 + 32.0)) / 2.0;

        for item in self.help_rows.iter() {
            d.draw_text_ex(game.get_font(), &item.content, Vector2 {x: x, y: y}, item.font_size as f32, game.get_font_spacing(), Color::BLACK);
            y += item.padding_bottom as f32;
        }
    }

    fn draw_menu(&self, d: &mut RaylibDrawHandle, game: &game::Game) {
        if game.get_state() == game::GameState::Menu {
            if self.state == MenuState::Primary {
                for item in self.items.iter() {
                    self.draw_menu_button(d, &game, &item);
                }
            } else if self.state == MenuState::Settings {
                for item in self.settings_items.iter() {
                    self.draw_menu_button(d, &game, &item);
                }
            } else if self.state == MenuState::Help {
                self.draw_help_menu(d, game);
            }
        }
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle, game: &game::Game) {
        self.draw_menu(d, game);
    }

    fn update_primary_menu(&mut self, game: &game::Game, items_count: i32) {
        if items_count == 4 {
            self.items = Self::construct_menu_items(&Self::PRIMARY_ITEMS, game.get_window_width(), game.get_window_height());
        } else if items_count == 5 {
            self.items = Self::construct_menu_items(&Self::FULL_PRIMARY_ITEMS, game.get_window_width(), game.get_window_height());
        }
    }
}