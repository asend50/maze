use macroquad::prelude::*;


use crate::utils::preload_image::TextureManager;
use crate::ui::label::Label;
use macroquad::input::KeyCode;


pub async fn run(score: i32, tm: TextureManager) -> (String, i32, TextureManager){

let mut lbl_title = Label::new("Maze Escape", 250.0, 200.0, 100);
lbl_title.with_colors(BLACK, Some(WHITE));
let mut lbl_help = Label::new("Use WASD to move. Get the key and reach the\ncellar while avoiding the red squares to win", 125.0, 300.0, 40);
lbl_help.with_colors(BLACK, Some(WHITE));
let mut lbl_start = Label::new("Press SPACE to start", 275.0, 400.0, 50);
lbl_start.with_colors(BLACK, Some(WHITE));

    loop {
        clear_background(BLUE);
        lbl_title.draw();
        lbl_help.draw();
        lbl_start.draw();

        if is_key_pressed(KeyCode::Space) {
            return ("game".to_string(), score, tm);
        }

        next_frame().await;
    }
}