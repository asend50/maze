/*
By: <Asen Doiron>
Date: 2026-09-21
Program Details: <Program Description Here>
*/

mod ui;
mod utils;
mod custom;

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

    let mut start_x = 35.0;
    let mut start_y = 15.0;

let tm = TextureManager::new();


    tm.preload_with_loading_screen(&["assets/Verity.png","assets/maze.png","assets/wall.png","assets/win.png"], None, None).await;

    let maze = StillImage::from_preload(
    tm.get_preload("assets/maze.png").unwrap(),
    screen_width(),  // Width
    screen_height(), // Height
    0.0,             // X position
    0.0,             // Y position
    true,            // Enable stretching
    1.0,             // Zoom level (100%)
);

let mut player = Player::new(
    35.0,                   // X position
    15.0,                   // Y position
    screen_width() * 0.07,  // Width
    screen_height() * 0.08, // Height
    "assets/Verity.png",    // Image name
    true,                   // Enable stretching
    1.0,                    // Zoom level (100%)
    3.0,                    // Speed
    vec2(35.0, 15.0),       // Old position
).await;

let mut wall1= StillImage::from_preload(
    tm.get_preload("assets/wall.png").unwrap(),
    screen_width() * 0.2,  // Width
    screen_height() * 0.1, // Height
    -200.0,                   // X position
    525.0,                   // Y position
    true,                  // Enable stretching
    1.0,                   // Zoom level (100%)
);

let mut wall2= StillImage::from_preload(
    tm.get_preload("assets/wall.png").unwrap(),
    screen_width() * 0.07,  // Width
    screen_height() * 0.23, // Height
    500.0,                   // X position
    -200.0,                   // Y position
    true,                  // Enable stretching
    1.0,                   // Zoom level (100%)
);

let mut win = StillImage::from_preload(
    tm.get_preload("assets/win.png").unwrap(),
    screen_width() * 0.12,  // Width
    screen_height() * 0.15, // Height
    900.0,                   // X position
    650.0,                   // Y position
    true,                  // Enable stretching
    1.0,                   // Zoom level (100%)
);

let mut lbl_win = Label::new("You win!", 350.0, 350.0, 90);
lbl_win.with_colors(BLACK, Some(WHITE));
let mut lbl_restart = Label::new("Press SPACE to restart", 250.0, 400.0, 50);
lbl_restart.with_colors(BLACK, Some(WHITE));
lbl_win.set_visible(false);
lbl_restart.set_visible(false);
let mut lbl_help = Label::new("Use WASD to move, reach the green\nsquare and avoid the red squares to win", 100.0, 300.0, 50);
lbl_help.with_colors(BLACK, Some(WHITE));


/* let collision = check_collision(&verity, &maze, 1); // 1 = pixel skip (for performance)
*/

    loop {
        clear_background(WHITE);
        win.draw();
        maze.draw();
        player.get_image().draw();
        wall1.draw();
        wall2.draw();
        lbl_win.draw();
        lbl_restart.draw();
        lbl_help.draw();

        if wall1.get_x() >= -200.0 && wall1.get_x() <= 1050.0{
            wall1.set_x(wall1.get_x() + 2.0);
        }

        if wall1.get_x() == 1050.0 {
            wall1.set_x(-200.0);
        }

        if wall2.get_y() >= -200.0 && wall2.get_y() <= 900.0{
            wall2.set_y(wall2.get_y() + 1.0);
        }

        if wall2.get_y() == 900.0{
            wall2.set_y(-200.0);
        }


        player.keypress();
        player.move_player();

        if player.move_dir.x != 0.0 {
            lbl_help.set_visible(false);
            if check_collision(player.get_image(), &maze, 1) {
                player.move_back_x();
            }
        }

        if player.move_dir.y != 0.0 {
            lbl_help.set_visible(false);
            if check_collision(player.get_image(), &maze, 1) {
                player.move_back_y();
            }
        }

        if player.move_dir.x != 0.0 {
            lbl_help.set_visible(false);
            if check_collision(player.get_image(), &wall1, 1) {
                player.move_to_start();
            }
        }

        if player.move_dir.y != 0.0 {
            if check_collision(player.get_image(), &wall1, 1) {
                player.move_to_start();
            }
        }

        if player.move_dir.x != 0.0 {
            if check_collision(player.get_image(), &wall2, 1) {
                player.move_to_start();
            }
        }

        if player.move_dir.y != 0.0 {
            if check_collision(player.get_image(), &wall2, 1) {
                player.move_to_start();
            }
        }
        if player.move_dir.x != 0.0 {
            if check_collision(player.get_image(), &win, 1) {
                player.move_to_win();
                lbl_win.set_visible(true);
            }
        }

        if player.move_dir.y != 0.0 {
            if check_collision(player.get_image(), &win, 1) {
                player.move_to_win();
                lbl_win.set_visible(true);
                lbl_restart.set_visible(true);
            }
        }

        if lbl_restart.is_visible() && is_key_down(KeyCode::Space) {
            player.move_to_start();
            lbl_win.set_visible(false);
            lbl_restart.set_visible(false);
        }

        next_frame().await;
}
}

