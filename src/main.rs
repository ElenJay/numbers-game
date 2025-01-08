#![windows_subsystem = "windows"]

use raylib::prelude::*;
use raylib::core::texture::Image;
use std::env;

mod game;
mod menu;
mod level;
mod utils;
mod timer;

use game::Game;
use menu::Menu;
use level::Level;

fn main() {
    // Init window
    let (mut rl, thread) = raylib::init()
        .size(Game::DEFAULT_WINDOW_WIDTH, Game::DEFAULT_WINDOW_HEIGHT)
        .title("Numbers Game")
        .resizable()
        .vsync()
        .build();

    let mut game: Game = Game::new(&mut rl, &thread, game::GameMode::Release);

    if env::consts::OS != "macos" {
        // ToDo: in MacOS the app should be properly bundled to show icon
        rl.set_window_icon(Image::load_image("assets/images/icon.png").unwrap());
    }

    update_window_sizes(&mut rl, &mut game);

    let mut menu: Menu = Menu::new(&game);
    let mut level: Level = Level::new(&game);

    while !rl.window_should_close() {
        // Processing controllers
        game.process_controller(&mut rl, &mut menu, &mut level);
        menu.process_controller(&mut rl, &mut game, &mut level);
        level.process_controller(&rl, &mut game);

        // Drawing
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);

        game.draw(&mut d);
        menu.draw(&mut d, &game);
        level.draw(&mut d, &game);
    }
}

fn update_window_sizes(rl: &mut RaylibHandle, game: &mut Game) {
    let monitor_index: i32 = get_current_monitor_index();
    
    // ToDo: need to check if it requires on Windows and Linux
    rl.toggle_borderless_windowed();
    rl.toggle_fullscreen();
    
    let window_width: i32 = get_monitor_width(monitor_index);
    let window_height: i32 = get_monitor_height(monitor_index);

    game.set_fullscreen_sizes(window_width, window_height);
    game.set_window_sizes(window_width, window_height);

    if env::consts::OS == "macos" {
        // Mac OS requires to get window size already in fullscreen mode, and only then set it, before enabling fullscreen mode
        rl.toggle_fullscreen();
        rl.set_window_size(window_width, window_height);
        rl.toggle_fullscreen();
        rl.set_window_focused();
    }
}