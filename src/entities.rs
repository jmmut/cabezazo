use macroquad::prelude::*;
use crate::textures::Textures;

pub fn maybe_add_obstacles(
    runner_size: Vec2,
    obstacles: &mut Vec<Vec2>,
    frame_count: i32,
    seed: &mut i32,
) {
    if frame_count % 60 == 0 {
        obstacles.push(Vec2::new(
            (*seed % ((screen_width() - runner_size.x) as i32)) as f32,
            -runner_size.y,
        ));
        let microseconds = (get_frame_time() * 1000000.0) as i32;
        *seed = (*seed + microseconds) % 10000;
        *seed = *seed * *seed + microseconds % 16;
    }
}

pub fn update_obstacles_pos(obstacles: &mut Vec<Vec2>, bottom_limit: f32) -> usize {
    let mut to_remove = Vec::new();

    let delta = get_frame_time();
    for (i, obstacle) in &mut obstacles.iter_mut().enumerate() {
        obstacle.y += 200.0 * delta;
        if obstacle.y > bottom_limit {
            to_remove.push(i);
        }
    }
    let obstacles_passed = to_remove.len();
    for i in to_remove {
        obstacles.remove(i);
    }
    obstacles_passed
}

pub fn update_runner_pos(runner_pos: &mut Vec2, right_limit: f32, left_limit: f32) {
    let delta = get_frame_time();

    if is_key_down(KeyCode::Right) || touches_local().iter().any(|t| t.position.x >= 0.0) {
        runner_pos.x += 300.0 * delta;
    }
    if runner_pos.x > right_limit {
        runner_pos.x = right_limit;
    }

    if is_key_down(KeyCode::Left) || touches_local().iter().any(|t| t.position.x < 0.0) {
        runner_pos.x -= 300.0 * delta;
    }
    if runner_pos.x < left_limit {
        runner_pos.x = left_limit;
    }
}

pub fn draw_obstacles(obstacles: &Vec<Vec2>, texture: &Texture2D, frame_count: i32) {
    for obstacle in obstacles {
        let mut params = DrawTextureParams::default();
        let flipped = frame_count / 20 % 2 == 0;
        params.flip_x = flipped;
        draw_texture_ex(*texture, obstacle.x, obstacle.y, WHITE, params);
    }
}

pub fn draw_runner(runner_pos: &Vec2, textures: &Textures, frame_count: i32, collided: bool, runner_lives: i32) {
    let runner_color = if collided { RED } else { WHITE };
    let mut params = DrawTextureParams::default();
    let flipped = frame_count / 20 % 2 == 0;
    params.flip_x = flipped;
    let texture = match runner_lives {
        3 => &textures.runner,
        2 => &textures.runner_scratched,
        1 | 0 => &textures.runner_dying,
        _ => unreachable!(),
    };
    draw_texture_ex(*texture, runner_pos.x, runner_pos.y, runner_color, params);
}

pub fn did_collide(runner_pos: &Vec2, obstacles: &Vec<Vec2>, size: &Vec2) -> bool {
    let squared_diameter = size.x * size.x;
    for obstacle in obstacles {
        let distance_x = runner_pos.x - obstacle.x;
        let distance_y = runner_pos.y - obstacle.y;
        let squared_distance = distance_x * distance_x + distance_y * distance_y;
        if squared_distance < squared_diameter {
            return true;
        }
    }
    return false;
}
