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

    let (mut rl, thread) = raylib::init()
        .size(game.get_window_width(), game.get_window_height())
        .title("Numbers Game")
        .resizable()
        .vsync()
        .build();

    if env::consts::OS != "macos" {
        // ToDo: in MacOS the app should be properly bundled to show icon
        rl.set_window_icon(Image::load_image("assets/icon.png").unwrap());
    }
    let mut menu = Menu::new(&game);
    let mut level = Level::new();

    while !rl.window_should_close() {
        if rl.is_window_resized() {
            game.set_window_width(rl.get_screen_width());
            game.set_window_height(rl.get_screen_height());
            menu.update_btn_positions(&game);
        }

        // Process control
        game.process_game_controller(&mut rl);
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