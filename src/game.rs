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

pub async fn run(score: i32) -> (String, i32) {
    let mut current_score = score;
    let mut start_x = 35.0;
    let mut start_y = 15.0;

let tm = TextureManager::new();


    tm.preload_with_loading_screen(&["assets/Verity.png","assets/maze.png","assets/wall.png","assets/win.png", "assets/key.png", "assets/cellar.png"], None, None).await;

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
    tm.get_preload("assets/cellar.png").unwrap(),
    screen_width() * 0.12,  // Width
    screen_height() * 0.15, // Height
    900.0,                   // X position
    650.0,                   // Y position
    true,                  // Enable stretching
    1.0,                   // Zoom level (100%)
);

let mut key = StillImage::from_preload(
    tm.get_preload("assets/key.png").unwrap(),
    screen_width() * 0.10,  // Width
    screen_height() * 0.13, // Height
    450.0,                   // X position
    500.0,                   // Y position
    true,                  // Enable stretching
    1.0,                   // Zoom level (100%)
);

let mut keyheld = false;


/* let collision = check_collision(&verity, &maze, 1); // 1 = pixel skip (for performance)
*/

    loop {
        clear_background(WHITE);
        win.draw();
        maze.draw();
        player.get_image().draw();
        key.draw();
        wall1.draw();
        wall2.draw();

        if wall1.get_x() >= -200.0 && wall1.get_x() <= 1050.0{
            wall1.set_x(wall1.get_x() + 1.0);
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
            if check_collision(player.get_image(), &maze, 1) {
                player.move_back_x();
            }
        }

        if player.move_dir.y != 0.0 {
            if check_collision(player.get_image(), &maze, 1) {
                player.move_back_y();
            }
        }

        if player.move_dir.x != 0.0 {
            if check_collision(player.get_image(), &wall1, 1) {
                player.move_to_start();
                keyheld = false;
                key.set_x(450.0);
            }
        }

        if player.move_dir.y != 0.0 {
            if check_collision(player.get_image(), &wall1, 1) {
                player.move_to_start();
                keyheld = false;
                key.set_x(450.0);
            }
        }

        if player.move_dir.x != 0.0 {
            if check_collision(player.get_image(), &wall2, 1) {
                player.move_to_start();
                keyheld = false;
                key.set_x(450.0);
            }
        }

        if player.move_dir.y != 0.0 {
            if check_collision(player.get_image(), &wall2, 1) {
                player.move_to_start();
                keyheld = false;
                key.set_x(450.0);
            }
        }

        if player.move_dir.x != 0.0 {
            if check_collision(player.get_image(), &key, 1) {
                keyheld = true;
                key.set_x(-200.0);
            }
        }

        if player.move_dir.y != 0.0 {
            if check_collision(player.get_image(), &key, 1) {
                keyheld = true;
                key.set_x(-200.0);
            }
        }

        if player.move_dir.x != 0.0 {
            if check_collision(player.get_image(), &win, 1) && keyheld == true {
            }
        }

        if player.move_dir.y != 0.0 {
            if check_collision(player.get_image(), &win, 1) && keyheld == true {
            }
        }

        if is_key_pressed(KeyCode::Space) {
            current_score += 1;
            return ("win".to_string(), current_score);
        }

        

        next_frame().await;
    }
}
