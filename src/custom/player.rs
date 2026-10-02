use crate::ui::still_image::StillImage;
use macroquad::input::KeyCode;
use macroquad::prelude::*;

pub struct Player {
    pub image: StillImage,
    pub speed: f32,
    move_dir: Vec2,
}
impl Player {
    pub async fn new(
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        image_name: &str,
        stretch: bool,
        zoom: f32,
        speed: f32,
    ) -> Self {
        let image = StillImage::new(image_name, width, height, x, y, stretch, zoom).await;

        Player {
            image,
            speed,
            move_dir: vec2(0.0, 0.0),
        }
    }

    pub fn get_image(&self) -> &StillImage {
        &self.image
    }

    pub fn keypress(&mut self) {
        if is_key_down(KeyCode::W) {
            // Direction to move in
            self.move_dir = vec2(0.0, 0.0);

            // Keyboard input
            if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) {
                self.move_dir.x += 3.0;
            }

            if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) {
                self.move_dir.x -= 3.0;
            }

            if is_key_down(KeyCode::W) || is_key_down(KeyCode::Up) {
                self.move_dir.y += 3.0;
            }

            if is_key_down(KeyCode::S) || is_key_down(KeyCode::Down) {
                self.move_dir.y -= 3.0;
            }

            if self.move_dir.length() > 0.0 {
                self.move_dir = self.move_dir.normalize();
            }
        }
    }

    pub fn get_speed(&self) -> f32 {
        self.speed
    }
}
