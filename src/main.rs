pub mod entities;
pub mod textures;

use macroquad::prelude::*;
use entities::*;
use crate::textures::load_textures;

const DEFAULT_WINDOW_TITLE: &'static str = "Cabezazo";
const DEFAULT_WINDOW_WIDTH: i32 = 640;
const DEFAULT_WINDOW_HEIGHT: i32 = 640;

#[macroquad::main(window_conf)]
async fn main() -> Result<(), FileError> {
    let textures = load_textures().await?;
    let runner_size = Vec2::new(64.0, 64.0);
    let mut runner_pos = Vec2::new(
        screen_width() / 2.0 - runner_size.x / 2.0,
        screen_height() - runner_size.y,
    );

    let mut obstacles = Vec::new();

    let bottom_limit = screen_height() - runner_size.y;
    let right_limit = screen_width() - runner_size.x;
    let left_limit = 0.0;
    let mut frame_count = 0;
    let mut seed = 831435;
    loop {
        increase_frame(&mut frame_count);
        clear_background(BEIGE);
        if is_key_down(KeyCode::Escape) {
            break;
        }

        maybe_add_obstacles(runner_size, &mut obstacles, frame_count, &mut seed);

        update_runner_pos(&mut runner_pos, right_limit, left_limit);
        update_obstacles_pos(&mut obstacles, bottom_limit);

        draw_obstacles(runner_size, &mut obstacles);
        draw_runner(&runner_size, &runner_pos, &mut obstacles, &textures.runner);

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
