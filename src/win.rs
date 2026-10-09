use macroquad::prelude::*;


use crate::utils::preload_image::TextureManager;
use crate::ui::label::Label;
use macroquad::input::KeyCode;


pub async fn run(score: i32, tm: TextureManager) -> (String, i32, TextureManager) {

let mut lbl_win = Label::new("You win!", 350.0, 350.0, 90);
lbl_win.with_colors(BLACK, Some(WHITE));
let mut lbl_restart = Label::new("Press SPACE to restart", 250.0, 400.0, 50);
lbl_restart.with_colors(BLACK, Some(WHITE));

    loop {
        clear_background(YELLOW);
        lbl_win.draw();
        lbl_restart.draw();

        if is_key_pressed(KeyCode::Space) {
            return ("game".to_string(), score, tm);
        }

        next_frame().await;
    }
}