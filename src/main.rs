pub mod entities;
pub mod textures;

use crate::textures::load_textures;
use entities::*;
use macroquad::prelude::*;
use macroquad::ui::root_ui;
use macroquad::ui::widgets::{Button, Label, Window};

const DEFAULT_WINDOW_TITLE: &'static str = "Cabezazo";
const DEFAULT_WINDOW_WIDTH: i32 = 640;
const DEFAULT_WINDOW_HEIGHT: i32 = 640;

#[macroquad::main(window_conf)]
async fn main() -> Result<(), FileError> {
    let textures = load_textures().await?;
    let runner_size = Vec2::new(64.0, 64.0);
    let mut runner_lives = 3;
    let mut runner_pos = Vec2::new(
        screen_width() / 2.0 - runner_size.x / 2.0,
        screen_height() - runner_size.y,
    );

    let mut obstacles = Vec::new();

    let bottom_limit = screen_height();
    let right_limit = screen_width() - runner_size.x;
    let left_limit = 0.0;
    let mut frame_count = 0;
    let mut seed = 831435;
    let mut previous_collided = false;
    let mut obstacles_passed = 0;
    loop {
        if runner_lives > 0 {
            increase_frame(&mut frame_count);
            maybe_add_obstacles(runner_size, &mut obstacles, frame_count, &mut seed);
            update_runner_pos(&mut runner_pos, right_limit, left_limit);
            obstacles_passed += update_obstacles_pos(&mut obstacles, bottom_limit);
        } else {
            if draw_game_over(runner_lives, obstacles_passed) {
                runner_lives = 3;
                previous_collided = false;
                obstacles_passed = 0;
                obstacles = Vec::new();
            }
        }
        clear_background(LIGHTGRAY);
        if is_key_down(KeyCode::Escape) {
            break;
        }

        draw_obstacles(&mut obstacles, &textures.obstacle, frame_count);
        let collided = did_collide(&runner_pos, &obstacles, &runner_size);
        if !collided && previous_collided {
            runner_lives -= 1; // decrease life after finishing collision, so that users sees the collision
        }
        previous_collided = collided;
        draw_runner(&runner_pos, &textures, frame_count, collided, runner_lives);
        next_frame().await
    }
    Ok(())
}

fn window_conf() -> Conf {
    Conf {
        window_title: DEFAULT_WINDOW_TITLE.to_owned(),
        window_width: DEFAULT_WINDOW_WIDTH,
        window_height: DEFAULT_WINDOW_HEIGHT,
        ..Default::default()
    }
}

fn increase_frame(frame_count: &mut i32) {
    *frame_count += 1;
    const MAX_FRAME: i32 = 10000;
    if *frame_count > MAX_FRAME {
        *frame_count -= MAX_FRAME;
    }
}

fn draw_game_over(runner_lives: i32, obstacles_passed: usize) -> bool {
    let mut restart = false;
    if runner_lives == 0 {
        Window::new(1,
                    Vec2::new(screen_width() / 4.0, screen_height() / 4.0),
                    Vec2::new(screen_width() / 2.0, screen_height() / 4.0))
            .label("Game Over")
            .ui(&mut root_ui(), |ui| {
                Label::new(format!("You survived {} obstacles!", obstacles_passed)).ui(ui);
                restart = Button::new("Restart").ui(ui)
            });
    }
    restart
}
