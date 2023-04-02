pub mod entities;
mod headbutt;
pub mod textures;

use crate::headbutt::{Headbutt, HeadbuttStage};
use crate::textures::load_textures;
use entities::*;
use macroquad::prelude::*;
use macroquad::ui::root_ui;
use macroquad::ui::widgets::{Button, Label, Window};

const DEFAULT_WINDOW_TITLE: &'static str = "Cabezazo";
const DEFAULT_WINDOW_WIDTH: i32 = 480;
const DEFAULT_WINDOW_HEIGHT: i32 = 640;

const INITIAL_DIFFICULTY: i32 = 30;

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
    let mut headbutt = Headbutt::new();
    let mut difficulty = INITIAL_DIFFICULTY;
    let mut difficulty_progress = 0;
    let mut previous_restart = false;
    loop {
        if runner_lives > 0 {
            increase_frame(&mut frame_count, &mut difficulty, &mut difficulty_progress);
            maybe_add_obstacles(
                runner_size,
                &mut obstacles,
                frame_count,
                &mut seed,
                difficulty,
            );
            if should_headbutt(previous_collided, previous_restart) {
                headbutt.start();
            }
            headbutt.update(get_frame_time());
            runner_pos.y = screen_height() - headbutt.pos() - runner_size.y;
            update_runner_pos(&mut runner_pos, right_limit, left_limit);
            obstacles_passed += update_obstacles_pos(&mut obstacles, bottom_limit);
            if is_mouse_button_released(MouseButton::Left) {
                previous_restart = false;
            }
        } else {
            if draw_game_over(runner_lives, obstacles_passed) {
                runner_lives = 3;
                previous_collided = false;
                obstacles_passed = 0;
                obstacles = Vec::new();
                headbutt = Headbutt::new();
                difficulty = INITIAL_DIFFICULTY;
                difficulty_progress = 0;
                previous_restart = true;
            }
        }
        clear_background(LIGHTGRAY);
        if is_key_down(KeyCode::Escape) {
            break;
        }

        draw_obstacles(&mut obstacles, &textures.obstacle, frame_count);
        let (mut collided, obstacle_idx) = did_collide(&runner_pos, &obstacles, &runner_size);
        if headbutt.stage == HeadbuttStage::Hitting && collided {
            obstacles.remove(obstacle_idx);
            collided = false;
        }
        if !collided && previous_collided && headbutt.stage != HeadbuttStage::Hitting {
            runner_lives -= 1; // decrease life after finishing collision, so that users sees the collision
        }
        previous_collided = collided;
        draw_runner(
            &runner_pos,
            &textures,
            frame_count,
            collided,
            runner_lives,
            headbutt.stamina(),
        );
        draw_button_overlay();
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

fn increase_frame(frame_count: &mut i32, difficulty: &mut i32, difficulty_progress: &mut i32) {
    *frame_count += 1;
    const MAX_FRAME: i32 = 10000;
    *difficulty_progress += 1;
    let period = 200 - *difficulty * 2;
    if *difficulty_progress > period {
        *difficulty_progress = 0;
        let new_difficulty = 1.max(*difficulty - 1);
        // eprintln!("increasing difficulty from {difficulty} to {new_difficulty} at frame {frame_count}. using period {period}");
        *difficulty = new_difficulty;
    }
    if *frame_count > MAX_FRAME {
        *frame_count -= MAX_FRAME;
    }
}

fn draw_game_over(runner_lives: i32, obstacles_passed: usize) -> bool {
    let mut restart = false;
    if runner_lives == 0 {
        Window::new(
            1,
            Vec2::new(screen_width() / 4.0, screen_height() / 4.0),
            Vec2::new(screen_width() / 2.0, screen_height() / 4.0),
        )
        .label("Game Over")
        .ui(&mut root_ui(), |ui| {
            Label::new(format!("You survived {} obstacles!", obstacles_passed)).ui(ui);
            restart = Button::new("Restart").ui(ui)
        });
    }
    restart
}

fn draw_button_overlay() {
    let width = screen_width();
    let height = screen_height();
    let color = Color::new(0.2, 0.2, 0.2, 0.2);
    let radius = (width / 16.0).min(height / 16.0);

    draw_move_button(
        width,
        height,
        color,
        radius,
        width * 6.0 / 8.0,
        height * 6.0 / 8.0,
        0.0,
    );
    draw_move_button(
        width,
        height,
        color,
        radius,
        width * 2.0 / 8.0,
        height * 6.0 / 8.0,
        180.0,
    );
    draw_move_button(
        width,
        height,
        color,
        radius,
        width * 4.0 / 8.0,
        height * 2.0 / 8.0,
        270.0,
    );
}

fn draw_move_button(
    width: f32,
    height: f32,
    color: Color,
    radius: f32,
    x: f32,
    y: f32,
    rotation: f32,
) {
    draw_poly(x, y, 3, radius, rotation, /* 10.0 ,*/ color);

    draw_rectangle_lines(
        x - width * 1.0 / 8.0,
        y - height * 1.0 / 8.0,
        width / 4.0,
        height / 4.0,
        10.0,
        color,
    );
}
