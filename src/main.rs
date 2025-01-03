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
    let mut game = Game::new();

    // Init window
    let (mut rl, thread) = raylib::init()
        .size(game.get_window_width(), game.get_window_height())
        .title("Numbers Game")
        .resizable()
        .vsync()
        .build();

    // Toggle fullscreen
    let monitor_index = get_current_monitor_index();
    
    // ToDo: need to check if it requires on Windows and Linux
    rl.toggle_borderless_windowed();
    rl.toggle_fullscreen();
    
    let window_width = get_monitor_width(monitor_index);
    let window_height = get_monitor_height(monitor_index);

    game.set_fullscreen_sizes(window_width, window_height);
    game.set_window_width(window_width);
    game.set_window_height(window_height);

    if env::consts::OS == "macos" {
        // Mac OS requires to get window size already in fullscreen mode, and only then set it, before enabling fullscreen mode
        rl.toggle_fullscreen();
        rl.set_window_size(window_width, window_height);
        rl.toggle_fullscreen();
        rl.set_window_focused();
    }

    let mut menu = Menu::new(&game);
    let mut level = Level::new();

    if env::consts::OS != "macos" {
        // ToDo: in MacOS the app should be properly bundled to show icon
        rl.set_window_icon(Image::load_image("assets/icon.png").unwrap());
    }

    while !rl.window_should_close() {
        if rl.is_window_resized() {
            game.set_window_width(rl.get_screen_width());
            game.set_window_height(rl.get_screen_height());
            menu.update_btn_positions(&game);
        }

        // Process control
        game.process_game_controller(&mut rl, &mut menu);
        menu.process_menu_controller(&mut rl, &mut game, &mut level);
        level.process_level_controller(&rl, &mut game);

        // Drawing
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);

        game.draw_fps(&mut d);
        menu.draw_menu(&mut d, &game);
        level.draw_score(&mut d, &game);
        level.draw(&mut d, &game);
    }
}