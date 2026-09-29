/*
By: <Asen Doiron>
Date: 2026-09-21
Program Details: <Program Description Here>
*/

mod ui;
mod utils;

use macroquad::prelude::*;

use crate::ui::grid::draw_grid;
use crate::ui::image_button::ImageButton;
use crate::ui::still_image::StillImage;
use crate::utils::preload_image::TextureManager;
use crate::utils::preload_image::LoadingScreenOptions; // If you want to customize the loading screen
use crate::ui::label::Label;
use macroquad::input::KeyCode;
use crate::utils::collision::check_collision;

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

let mut verity = StillImage::from_preload(
    tm.get_preload("assets/Verity.png").unwrap(),
    screen_width() * 0.07,  // Width
    screen_height() * 0.08, // Height
    35.0,                   // X position
    15.0,                   // Y position
    true,                  // Enable stretching
    1.0,                   // Zoom level (100%)
);

let mut wall= StillImage::from_preload(
    tm.get_preload("assets/wall.png").unwrap(),
    screen_width() * 0.2,  // Width
    screen_height() * 0.1, // Height
    -200.0,                   // X position
    525.0,                   // Y position
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
lbl_win.set_visible(false);
let mut lbl_help = Label::new("Use WASD to move, reach the green\nsquare and avoid the red squares to win", 100.0, 300.0, 50);
lbl_help.with_colors(BLACK, Some(WHITE));


let collision = check_collision(&verity, &maze, 1); // 1 = pixel skip (for performance)

    loop {
        clear_background(WHITE);
        draw_grid(50.0, BLACK);
        win.draw();
        maze.draw();
        verity.draw();
        wall.draw();
        lbl_win.draw();
        lbl_help.draw();

        if wall.get_x() >= -200.0 && wall.get_x() <= 1050.0{
            wall.set_x(wall.get_x() + 2.0);
        }

        

        if wall.get_x() == 1050.0 {
            wall.set_x(-200.0);
        }



        // Assume `player` is your module
let mut x = verity.get_x();
let mut y = verity.get_y();

const MOVE_SPEED: f32 = 200.0;

 // Direction to move in
        let mut move_dir = vec2(0.0, 0.0);

        // Keyboard input
        if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) {
            move_dir.x += 6.0;
            lbl_help.set_visible(false);
        
        }
        if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) {
            move_dir.x -= 6.0;
            lbl_help.set_visible(false);
        
        }
        if is_key_down(KeyCode::S) || is_key_down(KeyCode::Down) {
            move_dir.y += 6.0;
            lbl_help.set_visible(false);
        }
        if is_key_down(KeyCode::W) || is_key_down(KeyCode::Up) {
            move_dir.y -= 6.0;
            lbl_help.set_visible(false);
        }

        // Normalize the movement to prevent faster diagonal movement
        if move_dir.length() > 0.0 {
            move_dir = move_dir.normalize();
        }

        // Apply movement based on frame time
        let movement = move_dir * MOVE_SPEED * get_frame_time();

        // Save old position in case of collision
        let old_pos = verity.pos();

        // Move X first
        if movement.x != 0.0 {
            verity.set_x(verity.get_x() + movement.x);
            if check_collision(&verity, &maze, 1) {
                verity.set_x(old_pos.x); // Undo if collision happens
            }
            
        }

        // Move Y next
        if movement.y != 0.0 {
            verity.set_y(verity.get_y() + movement.y);
            if check_collision(&verity, &maze, 1) {
                verity.set_y(old_pos.y); // Undo if collision happens
            }
            
        }
// Update the module's position

let collisionmaze = check_collision(&verity, &maze, 1);

if collisionmaze{
    verity.set_x(x - 2.0);
}

let collisionwall = check_collision(&verity, &wall, 1);

if collisionwall{
    verity.set_x(35.0);
    verity.set_y(15.0);
}

let collisionwin = check_collision(&verity, &win, 1);

if collisionwin{
    verity.set_x(925.0);
    verity.set_y(675.0);
    lbl_win.set_visible(true);
    
}

        next_frame().await;
}
}

