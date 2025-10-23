use std::time::{Duration, Instant};

use macroquad::prelude::*;

struct Bullet {
    x: f32,
    y: f32,
}

#[macroquad::main("BasicShapes")]
async fn main() {
    let spaceship_image = load_image("./spaceship.png").await.unwrap();
    let spaceship_texture = Texture2D::from_image(&spaceship_image);

    let bullet_image = load_image("./bullet.png").await.unwrap();
    let bullet_texture = Texture2D::from_image(&bullet_image);

    let mut x = (screen_width() / 2.) - 32.;
    let mut y = screen_height() - 200.;

    let mut fire_delay = Instant::now();
    let mut bullets = vec![];

    loop {
        clear_background(BLACK);

        if is_key_down(KeyCode::Right) {
            if x < screen_width() - 90. {
                x += 10.0;
            }
        }
        if is_key_down(KeyCode::Left) {
            if x > 0. {
                x -= 10.0;
            }
        }
        if is_key_down(KeyCode::Down) {
            if y < screen_height() - 130. {
                y += 10.0;
            }
        }
        if is_key_down(KeyCode::Up) {
            if y > 0. {
                y -= 10.0;
            }
        }

        if is_key_down(KeyCode::Space) {
            if fire_delay.elapsed() > Duration::from_millis(100) {
                bullets.push(Bullet { x: x + 32., y });
                fire_delay = Instant::now();
            }
        }

        bullets.retain_mut(|bullet| {
            bullet.y -= 10.;
            bullet.y >= 0.
        });

        for bullet in bullets.iter() {
            draw_texture(&bullet_texture, bullet.x, bullet.y, WHITE);
        }

        draw_texture(&spaceship_texture, x, y, WHITE);

        next_frame().await
    }
}
