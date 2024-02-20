mod engine;
mod circular_body;

use macroquad::input::KeyCode::Escape;
use macroquad::prelude::*;
use crate::engine::Engine;

const UPDATES_PER_SECOND: u32 = 30;
const TIME_PER_UPDATE: f32 = 1f32 / UPDATES_PER_SECOND as f32;
pub const MASS_FACTOR: f64 = 5_000_000f64; // weight of every voxel in kg

fn get_conf() -> Conf {
    Conf {
        window_title: "Boids".to_string(),
        window_width: 0,
        window_height: 0,
        high_dpi: false,
        fullscreen: true,
        sample_count: 0,
        window_resizable: false,
        icon: None,
        platform: Default::default(),
    }
}

#[macroquad::main(get_conf())]
async fn main() {

    let mut engine = Engine::new();
    engine.spawn_sample(300, 4f64, 20f32);
    engine.spawn_object(100f64, Vec2::new(screen_width()/2f32, screen_height()/2f32), Vec2::ZERO);

    let (mut pos, mut vel, mut radius) = (Vec2::ZERO, Vec2::ZERO, 0f64);
    let mut init_sequence: usize = 0;

    let mut lag: f32 = 0f32;
    loop {
        if is_key_down(Escape) { break; }
        if is_mouse_button_pressed(MouseButton::Left) {
            match init_sequence {
                0 => {
                    pos = Vec2::new(mouse_position().0, mouse_position().1);
                    init_sequence = 1;
                },
                1 => {
                    radius = pos.distance(Vec2::new(mouse_position().0, mouse_position().1)) as f64;
                    init_sequence = 2;
                },
                2 => {
                    vel = Vec2::new(mouse_position().0, mouse_position().1) - pos;
                    let magnitude = vel.length() - radius as f32;
                    if magnitude < 0f32 {
                        vel = Vec2::ZERO;
                    } else {
                        vel = vel * (magnitude/vel.length()/10f32);
                    }
                    // every pixel (1 m^3) is weighing 10^6 tons
                    engine.spawn_object(radius, pos, vel);
                    init_sequence = 0;
                }
                _ => {}
            }
        }
        if is_mouse_button_down(MouseButton::Right) {
            init_sequence = 0;
        }
        lag += get_frame_time();

        while lag >= TIME_PER_UPDATE {
            lag -= TIME_PER_UPDATE;

            engine.update_all();
        }
        engine.draw_all();

        match init_sequence {
            0 => (),
            1 => {
                let rad = pos.distance(Vec2::new(mouse_position().0, mouse_position().1));
                draw_circle(pos.x, pos.y, rad, WHITE);
            },
            2 => {
                let mouse_pos = Vec2::new(mouse_position().0, mouse_position().1);
                draw_line(pos.x, pos.y, mouse_pos.x, mouse_pos.y, 5f32, BLUE);
                draw_circle(pos.x, pos.y, radius as f32, WHITE);

            }
            _ => {}
        }

        next_frame().await;
    }
}
