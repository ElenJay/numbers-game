use raylib::prelude::*;
use raylib::core::texture::Image;
use std::env;

mod game;
mod menu;
mod player;
mod utils;


fn main() {
    let (mut rl, thread) = raylib::init()
        .size(game::WINDOW_WIDTH, game::WINDOW_HEIGHT)
        .title("Numbers Game")
        .build();

    let mut game = game::Game::new();
    if env::consts::OS != "macos" {
        // ToDo: in MacOS the app should be properly bundled to show icon
        rl.set_window_icon(Image::load_image("assets/icon.png").unwrap());
    }
    let mut menu = menu::Menu::new();
    let mut player = player::Player::new();

    while !rl.window_should_close() {
        game.process_game_controller(&mut rl);
        menu.process_menu_controller(&mut rl, &mut game);
        player.process_player_controller(&rl, &game);

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);

        if game.get_mode() == game::GameMode::Debug {
            d.draw_fps( 10, 10);
        }
        let welcome_text = format!("The highest score is {0} points.", 410);
        utils::draw_text_center(&mut d, welcome_text.as_str(), 12, 20, Color::GREEN);

        menu.draw_menu(&mut d, &game);
        player.draw(&mut d, &game);
    }
}