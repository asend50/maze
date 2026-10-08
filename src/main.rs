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

use crate::custom::player;
use crate::ui::image_button::ImageButton;
use crate::ui::still_image::StillImage;
use crate::utils::preload_image::TextureManager;
use crate::utils::preload_image::LoadingScreenOptions; // If you want to customize the loading screen
use crate::ui::label::Label;
use macroquad::input::KeyCode;
use crate::utils::collision::check_collision;
use crate::custom::player::Player;

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
    let mut current_screen = "game".to_string();
    let mut score = 0;
    let mut last_switch = get_time() - 0.02;

    loop {
        if get_time() - last_switch > 0.01 {
            (current_screen, score) = match current_screen.as_str() {
                "screen1" => game::run(score).await,
                "screen2" => win::run(score).await,
                "screen3" => lose::run(score).await,
                "screen4" => menu::run(score).await,
                _ => break,
            };
            last_switch = get_time();
        }
        next_frame().await;
    }
}

