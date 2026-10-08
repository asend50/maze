use macroquad::prelude::*;

use crate::ui::label::Label;
use macroquad::input::KeyCode;

pub async fn run(score: i32) -> (String, i32){

let mut lbl_help = Label::new("Use WASD to move. Get the key and reach the\ncellar while avoiding the red squares to win", 100.0, 300.0, 40);
lbl_help.with_colors(BLACK, Some(WHITE));
let mut lbl_start = Label::new("Press SPACE to start", 250.0, 400.0, 50);
lbl_start.with_colors(BLACK, Some(WHITE));

    loop {
        lbl_help.draw();
        lbl_start.draw();
        clear_background(WHITE);

        if is_key_pressed(KeyCode::Space) {
            return ("game".to_string(), score);
        }

        next_frame().await;
    }
}