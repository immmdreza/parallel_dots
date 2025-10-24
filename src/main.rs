use std::time::{Duration, Instant};

use macroquad::{
    color::*,
    input::{KeyCode, is_key_down},
    shapes::draw_circle,
    texture::{Texture2D, draw_texture, load_image},
    window::{clear_background, next_frame, screen_height, screen_width},
};

mod world;

struct Bullet {
    x: f32,
    y: f32,
}

struct Enemy {
    x: f32,
    y: f32,
}

#[macroquad::main("Dots")]
async fn main() {
    let spaceship_image = load_image("./spaceship.png").await.unwrap();
    let spaceship_texture = Texture2D::from_image(&spaceship_image);

    let bullet_image = load_image("./bullet.png").await.unwrap();
    let bullet_texture = Texture2D::from_image(&bullet_image);

    let mut x = (screen_width() / 2.) - 32.;
    let mut y = screen_height() - 200.;

    let mut fire_delay = Instant::now();
    let mut bullets = vec![];
    let mut enemies = vec![];

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

        if enemies.len() < 10 && rand::random::<f32>() > 0.8 {
            enemies.push(Enemy {
                y: 0.,
                x: rand::random_range(0..screen_width() as u32) as f32,
            });
        }

        enemies.retain_mut(|enemy| {
            enemy.y += 5.;
            enemy.y < screen_height() - 50.
        });

        bullets.retain_mut(|bullet| {
            bullet.y -= 10.;
            bullet.y >= 0.
        });

        for bullet in bullets.iter() {
            draw_texture(&bullet_texture, bullet.x, bullet.y, WHITE);
        }

        for enemy in enemies.iter() {
            draw_circle(enemy.x, enemy.y, 10., RED);
        }

        draw_texture(&spaceship_texture, x, y, WHITE);

        next_frame().await
    }
}
