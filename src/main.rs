use raylib::prelude::*;
use raylib::core::texture::Image;
use std::env;

mod game;
mod menu;
mod player;
mod utils;
mod timer;

fn main() {
    let mut game = game::Game::new();
    println!("{0}, {1}", game.get_window_width(), game.get_window_height());

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
    let mut menu = menu::Menu::new(&game);
    let mut player = player::Player::new();
    let mut timer = timer::Timer::new(2 * 60);

    while !rl.window_should_close() {
        if rl.is_window_resized() {
            game.set_window_width(rl.get_screen_width());
            game.set_window_height(rl.get_screen_height());
            menu.update_btn_positions(&game);
        }
        game.process_game_controller(&mut rl);
        menu.process_menu_controller(&mut rl, &mut game, &mut player, &mut timer);
        player.process_player_controller(&rl, &game);

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);

        if game.get_mode() == game::GameMode::Debug {
            d.draw_fps( 10, 10);
        }
        let welcome_text = format!("Your score is {0} points.", player.get_score());
        utils::draw_text_center(&mut d, welcome_text.as_str(), 12, 36, Color::GREEN, &game);

        menu.draw_menu(&mut d, &game);
        if game.get_state() == game::GameState::Game {
            timer.draw(&mut d, &game);
        }
        player.draw(&mut d, &game);
    }
}