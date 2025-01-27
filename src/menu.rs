use raylib::prelude::*;
use raylib::consts::MouseButton::*;

use crate::consts;
use crate::game;
use crate::level;
use crate::utils::draw_text_center;

const DEFAULT_MENU_ITEM_WIDTH: f32 = 400.0;
const DEFAULT_MENU_ITEM_HEIGHT: f32 = 80.0;
const DEFAULT_MENU_ITEMS_DIFF: f32 = DEFAULT_MENU_ITEM_HEIGHT / 2.0;
const DEFAULT_MENU_ITEM_FONT_SIZE: f32 = 54.0;

struct HelpTextRow {
    content: &'static str,
    font_size: u8,
    padding_bottom: u16,
}

const HELP_TEXT_ROWS: [HelpTextRow; 10] = [
    HelpTextRow { font_size: 32, padding_bottom: 64, content: consts::HELP_TITLE_1_STRING_NAME, },
    HelpTextRow { font_size: 24, padding_bottom: 36, content: consts::HELP_TEXT_1_STRING_NAME, },
    HelpTextRow { font_size: 24, padding_bottom: 36, content: consts::HELP_TEXT_2_STRING_NAME, },
    HelpTextRow { font_size: 24, padding_bottom: 50, content: consts::HELP_TEXT_3_1_STRING_NAME, },
    HelpTextRow { font_size: 24, padding_bottom: 64, content: consts::HELP_TEXT_3_2_STRING_NAME, },
    HelpTextRow { font_size: 32, padding_bottom: 64, content: consts::HELP_TITLE_2_STRING_NAME, },
    HelpTextRow { font_size: 24, padding_bottom: 36, content: consts::HELP_TEXT_4_STRING_NAME, },
    HelpTextRow { font_size: 24, padding_bottom: 36, content: consts::HELP_TEXT_5_STRING_NAME, },
    HelpTextRow { font_size: 24, padding_bottom: 100, content: consts::HELP_TEXT_6_STRING_NAME, },
    HelpTextRow { font_size: 32, padding_bottom: 0, content: consts::HELP_TITLE_3_STRING_NAME, },
];

#[derive(Clone, Copy, PartialEq)]
enum MenuAllItems {
    Start,
    Continue,
    Settings,
    Help,
    Exit,
    Difficulty,
    Language,
    Fullscreen,
    ToggleFPS,
    Back,
}

impl MenuAllItems {
    fn value(&self) -> &str {
        match *self {
            Self::Start => consts::START_MENU_STRING_NAME,
            Self::Continue => consts::CONTINUE_MENU_STRING_NAME,
            Self::Settings => consts::SETTINGS_MENU_STRING_NAME,
            Self::Help => consts::HELP_MENU_STRING_NAME,
            Self::Exit => consts::EXIT_MENU_STRING_NAME,
            Self::Back => consts::BACK_MENU_STRING_NAME,
            _ => "",
        }
    }

    fn description(&self) -> &str {
        match *self {
            Self::Difficulty => consts::DIFFICULTY_MENU_STRING_NAME,
            Self::Language => consts::LANGUAGE_MENU_STRING_NAME,
            Self::Fullscreen => consts::FULLSCREEN_MENU_STRING_NAME,
            Self::ToggleFPS => consts::TOGGLE_FPS_MENU_STRING_NAME,
            _ => "",
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum MenuState {
    LanguageSelect,
    Primary,
    Settings,
    Help,
}

struct MenuItem {
    btn: Rectangle,
    title: MenuAllItems,
    color: Color,
}

struct LocaleItem {
    btn: Rectangle,
    color: Color,
}

pub struct Menu {
    state: MenuState,
    items: Vec<MenuItem>,
    settings_items: Vec<MenuItem>,
    locale_items: Vec<LocaleItem>,
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
    const SETTINGS_ITEMS: [MenuAllItems; 5] = [
        MenuAllItems::Difficulty,
        MenuAllItems::Language,
        MenuAllItems::Fullscreen, 
        MenuAllItems::ToggleFPS, 
        MenuAllItems::Back,
    ];

    fn construct_menu_items(menu_items: &[MenuAllItems], game: &game::Game) -> Vec<MenuItem> {
        let mut menu_item: MenuItem;
        let mut items: Vec<MenuItem> = Vec::with_capacity(menu_items.len());
        let all_items_height: f32 = menu_items.len() as f32 * DEFAULT_MENU_ITEM_HEIGHT + (menu_items.len() - 1) as f32 * DEFAULT_MENU_ITEMS_DIFF;
        
        for (index, item) in menu_items.iter().enumerate() {
            if item.description() == "" {
                menu_item = MenuItem {
                    btn: Rectangle {
                        x: (game.get_window_width() - DEFAULT_MENU_ITEM_WIDTH) / 2.0, 
                        y: (game.get_window_height() - all_items_height) / 2.0 + index as f32 * (DEFAULT_MENU_ITEM_HEIGHT + DEFAULT_MENU_ITEMS_DIFF), 
                        width: DEFAULT_MENU_ITEM_WIDTH, 
                        height: DEFAULT_MENU_ITEM_HEIGHT,
                    },
                    title: *item,
                    color: Color::LIGHTGRAY,
                };
            } else {
                menu_item = MenuItem {
                    btn: Rectangle {
                        x: game.get_window_width() / 8.0 * 7.0 - DEFAULT_MENU_ITEM_WIDTH, 
                        y: (game.get_window_height() - all_items_height) / 2.0 + index as f32 * (DEFAULT_MENU_ITEM_HEIGHT + DEFAULT_MENU_ITEMS_DIFF), 
                        width: DEFAULT_MENU_ITEM_WIDTH, 
                        height: DEFAULT_MENU_ITEM_HEIGHT,
                    },
                    title: *item,
                    color: Color::LIGHTGRAY,
                };
            }
            items.push(menu_item);
        }

        items
    }

    fn construct_locale_items(menu_state: MenuState, game: &game::Game) -> Vec<LocaleItem> {
        let mut items: Vec<LocaleItem>;
        let locales_len: usize = game.get_all_locales().len();
        
        if menu_state == MenuState::LanguageSelect {
            let mut texture: &Texture2D;
            items = Vec::with_capacity(locales_len);

            for (index, locale) in game.get_all_locales().iter().enumerate() {
                texture = locale.get_texture().as_ref().unwrap();
                items.push(LocaleItem {
                    btn: Rectangle {
                        x: (game.get_window_width() - ((texture.width + 100) as f32 * locales_len as f32 - 100.0)) / 2.0 + (texture.width as f32 + 100.0) * index as f32, 
                        y: (game.get_window_height() - texture.height as f32) / 2.0, 
                        width: texture.width as f32, 
                        height: texture.height as f32,
                    },
                    color: Color::WHITE,
                });
            }
        } else {
            items = Vec::with_capacity(0);
        }

        items
    }

    pub fn new(game: &game::Game) -> Self {
        let menu_state: MenuState = match std::fs::File::open(game::Game::SAVE_CONFIG_PATH) {
            Err(_) => MenuState::LanguageSelect,
            Ok(_) => MenuState::Primary,
        };

        Self {
            state: menu_state,
            items: Self::construct_menu_items(&Self::PRIMARY_ITEMS, game),
            settings_items: Self::construct_menu_items(&Self::SETTINGS_ITEMS, game),
            locale_items: Self::construct_locale_items(menu_state, game),
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
            } else {
                item.btn.x = (window_width - DEFAULT_MENU_ITEM_WIDTH) / 2.0;
            }
        }
    }

    pub fn process_controller(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread, game: &mut game::Game, level: &mut level::Level) {
        if game.get_state() != game::GameState::Menu { return; }

        let mouse_pos: Vector2 = rl.get_mouse_position();

        if self.state == MenuState::Primary {
            self.process_primary_menu_controller(rl, &mouse_pos, game, level);
        } else if self.state == MenuState::Settings {
            self.process_settings_menu_controller(rl, &mouse_pos, game, level);
        } else if self.state == MenuState::LanguageSelect {
            self.process_language_selector_controller(rl, thread, &mouse_pos, game, level);
        }
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle, game: &game::Game) {
        if game.get_state() != game::GameState::Menu { return; }

        if self.state == MenuState::Primary {
            for item in self.items.iter() {
                self.draw_menu_button(d, game, item);
            }
        } else if self.state == MenuState::Settings {
            for item in self.settings_items.iter() {
                self.draw_menu_button(d, game, &item);
            }
        } else if self.state == MenuState::LanguageSelect {
            self.draw_language_selector(d, game);
        } else if self.state == MenuState::Help {
            self.draw_help_menu(d, game);
        }
    }

    fn process_primary_menu_controller(&mut self, rl: &mut RaylibHandle, mouse_pos: &Vector2, game: &mut game::Game, level: &mut level::Level) {
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
                        MenuAllItems::Continue => level.resume(game),
                        MenuAllItems::Settings => self.state = MenuState::Settings,
                        MenuAllItems::Help => self.state = MenuState::Help,
                        MenuAllItems::Exit => std::process::exit(0),
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
    }

    fn process_settings_menu_controller(&mut self, rl: &mut RaylibHandle, mouse_pos: &Vector2, game: &mut game::Game, level: &mut level::Level) {
        let mut is_fullscreen_required: bool = false;

        for item in self.settings_items.iter_mut() {
            if item.btn.check_collision_point_rec(mouse_pos) {
                item.color = Color::LIGHTGREEN;
                if rl.is_mouse_button_released(MOUSE_BUTTON_LEFT) {
                    match item.title {
                        MenuAllItems::Difficulty => game.change_difficulty(game.get_difficulty()),
                        MenuAllItems::Language => game.change_locale(level),
                        MenuAllItems::Fullscreen => is_fullscreen_required = true,
                        MenuAllItems::ToggleFPS => game.toggle_fps_monitor(),
                        MenuAllItems::Back => self.state = MenuState::Primary,
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

    fn process_language_selector_controller(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread, mouse_pos: &Vector2, game: &mut game::Game, level: &mut level::Level) {
        for (index, item) in self.locale_items.iter_mut().enumerate() {
            if item.btn.check_collision_point_rec(mouse_pos) {
                let locale_code: String = game.get_all_locales()[index].get_code().clone();
                game.set_locale(&locale_code, level);
                item.color = Color::LIGHTGREEN;
                if rl.is_mouse_button_released(MOUSE_BUTTON_LEFT) {
                    game.update_config_file();

                    for locale in game.get_all_locales_mut() {
                        locale.unload_texture(rl, thread);
                    }

                    self.state = MenuState::Primary;
                    self.locale_items = Vec::with_capacity(0);
                    break;
                    
                }
            } else if item.color != Color::WHITE {
                item.color = Color::WHITE;
            }
        }
    }

    fn draw_menu_button(&self, d: &mut RaylibDrawHandle, game: &game::Game, menu_item: &MenuItem) {
        // Draw item button
        let item_btn_title: &str = match &menu_item.title {
            MenuAllItems::Difficulty => game.get_locale().get(game.get_difficulty().repr()).unwrap(),
            MenuAllItems::Language => game.get_locale().get_language(),
            MenuAllItems::Fullscreen => game.get_locale().get(
                if game.get_settings().is_fullscreen {consts::DISABLE_BTN_STRING_NAME} else {consts::ENABLE_BTN_STRING_NAME}
            ).unwrap(),
            MenuAllItems::ToggleFPS => game.get_locale().get(
                if game.get_settings().is_fps_visible {consts::DISABLE_BTN_STRING_NAME} else {consts::ENABLE_BTN_STRING_NAME}
            ).unwrap(),
            _ => game.get_locale().get(menu_item.title.value()).unwrap(),
        };
        d.draw_rectangle_rec(menu_item.btn, menu_item.color);
        let btn_text_sizes: Vector2 = game.get_font().measure_text(item_btn_title, DEFAULT_MENU_ITEM_FONT_SIZE, game.get_font_spacing());
        let btn_padding: Vector2 = Vector2 {
            x: menu_item.btn.x + (menu_item.btn.width - btn_text_sizes.x) / 2.0, 
            y: menu_item.btn.y + (menu_item.btn.height - btn_text_sizes.y) / 2.0
        };
        d.draw_text_ex(game.get_font(), item_btn_title, btn_padding, DEFAULT_MENU_ITEM_FONT_SIZE, game.get_font_spacing(), Color::BLACK);
        
        // Draw item description
        match menu_item.title.description() {
            "" => {},
            x => d.draw_text_ex(game.get_font(), game.get_locale().get(x).unwrap(), Vector2 {
                x: game.get_window_width() / 8.0,
                y: btn_padding.y,
            }, DEFAULT_MENU_ITEM_FONT_SIZE, game.get_font_spacing(), Color::BLACK),
        }
    }

    fn draw_language_selector(&self, d: &mut RaylibDrawHandle, game: &game::Game) {
        // Draw language icons
        for (index, locale) in game.get_all_locales().iter().enumerate() {
            let texture: &Texture2D = locale.get_texture().as_ref().unwrap();
            d.draw_texture_v(texture, Vector2 { 
                x: self.locale_items[index].btn.x, 
                y: self.locale_items[index].btn.y, 
            }, Color::WHITE);
            d.draw_rectangle_lines_ex(self.locale_items[index].btn, 10.0, self.locale_items[index].color);
        }

        // Draw text
        let text: String = format!("{} {}", game.get_locale().get(consts::CHOOSE_LANGUAGE_STRING_NAME).unwrap(), game.get_locale().get_language());
        draw_text_center(d, text.as_str(), self.locale_items[0].btn.y - 80.0, 48.0, Color::GREEN, &game);
    }

    fn draw_help_menu(&self, d: &mut RaylibDrawHandle, game: &game::Game) {
        // Calculate max length row of help menu
        let mut width: usize;
        let mut max_width: usize = 0;
        let mut max_help_text_index: usize = 0;

        for (index, row) in HELP_TEXT_ROWS.iter().enumerate() {
            width = game.get_locale().get(row.content).unwrap().len();
            if width > max_width {
                max_width = width;
                max_help_text_index = index;
            }
        };

        // Calculate measures of help text on the screen
        let text_sizes: Vector2 = game.get_font().measure_text(game.get_locale().get(&HELP_TEXT_ROWS[max_help_text_index].content).unwrap(), 20.0, game.get_font_spacing());
        let x: f32 = (game.get_window_width() - text_sizes.x) / 2.0;
        let mut y: f32 = (game.get_window_height() - (100.0 + 64.0 * 3.0 + 36.0 * 4.0 + 32.0)) / 2.0;

        // Draw help text
        for item in HELP_TEXT_ROWS.iter() {
            d.draw_text_ex(game.get_font(), game.get_locale().get(&item.content).unwrap(), Vector2 {x: x, y: y}, item.font_size as f32, game.get_font_spacing(), Color::BLACK);
            y += item.padding_bottom as f32;
        }
    }

    fn update_primary_menu(&mut self, game: &game::Game, items_count: i32) {
        // Updates menu with or without "Continue" button (to continue game)
        if items_count == 4 {
            self.items = Self::construct_menu_items(&Self::PRIMARY_ITEMS, game);
        } else if items_count == 5 {
            self.items = Self::construct_menu_items(&Self::FULL_PRIMARY_ITEMS, game);
        }
    }
}