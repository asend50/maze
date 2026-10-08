use macroquad::prelude::*;

use crate::ui::label::Label;
use macroquad::input::KeyCode;

pub async fn run(score: i32) -> (String, i32) {
    loop {
        clear_background(DARKGRAY);

        if is_key_pressed(KeyCode::Space) {
            return ("game".to_string(), score);
        }

        next_frame().await;
    }
}