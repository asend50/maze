/*
By: <Asen Doiron>
Date: 2026-09-21
Program Details: <The purpose of this program is to allow the player to traverse through a maze while avoiding moving walls to collect a key and reach the cellar to win.>
*/

mod ui;
mod utils;
mod custom;

mod game;
mod win;
mod lose;
mod menu;

use macroquad::prelude::*;


use crate::utils::preload_image::TextureManager;


/// Set up window settings before the app runs
fn window_conf() -> Conf {
    Conf {
        window_title: "maze".to_string(),
        window_width: 1024,
        window_height: 768,
        fullscreen: false,
        high_dpi: true,
        window_resizable: true,
        sample_count: 4, // MSAA: makes shapes look smoother
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {

    let mut tm = TextureManager::new();


    tm.preload_with_loading_screen(&["assets/Verity.png","assets/maze.png","assets/wall.png","assets/win.png", "assets/key.png", "assets/cellar.png"], None, None).await;

    let mut current_screen = "menu".to_string();
    let mut score = 0;
    let mut last_switch = get_time() - 0.02;

    loop {
        if get_time() - last_switch > 0.01 {
            (current_screen, score, tm) = match current_screen.as_str() {
                "menu" => menu::run(score, tm).await,
                "win" => win::run(score, tm).await,
                "lose" => lose::run(score, tm).await,
                "game" => game::run(score, tm).await,
                _ => break,
            };
            last_switch = get_time();
        }
        next_frame().await;
    }
}

