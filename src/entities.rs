use macroquad::prelude::*;

pub fn maybe_add_obstacles(
    runner_size: Vec2,
    obstacles: &mut Vec<Vec2>,
    frame_count: i32,
    seed: &mut i32,
) {
    if frame_count % 60 == 0 {
        obstacles.push(Vec2::new(
            (*seed % ((screen_width() - runner_size.x) as i32)) as f32,
            0.0,
        ));
        let microseconds = (get_frame_time() * 1000000.0) as i32;
        *seed = (*seed + microseconds) % 10000;
        *seed = *seed * *seed + microseconds % 16 ;
    }
}

pub fn update_obstacles_pos(obstacles: &mut Vec<Vec2>, bottom_limit: f32) {
    let mut to_remove = Vec::new();

    let delta = get_frame_time();
    for (i, obstacle) in &mut obstacles.iter_mut().enumerate() {
        obstacle.y += 200.0 * delta;
        if obstacle.y > bottom_limit {
            to_remove.push(i);
        }
    }
    for i in to_remove {
        obstacles.remove(i);
    }
}

pub fn update_runner_pos(runner_pos: &mut Vec2, right_limit: f32, left_limit: f32) {
    let delta = get_frame_time();

    if is_key_down(KeyCode::Right) {
        runner_pos.x += 300.0 * delta;
    }
    if runner_pos.x > right_limit {
        runner_pos.x = right_limit;
    }

    if is_key_down(KeyCode::Left) {
        runner_pos.x -= 300.0 * delta;
    }
    if runner_pos.x < left_limit {
        runner_pos.x = left_limit;
    }
}

pub fn draw_obstacles(runner_size: Vec2, obstacles: &Vec<Vec2>) {
    for obstacle in obstacles {
        draw_rectangle(obstacle.x, obstacle.y, runner_size.x, runner_size.y, RED);
    }
}

pub fn draw_runner(runner_size: &Vec2, runner_pos: &Vec2, obstacles: &Vec<Vec2>) {
    let runner_color = if collided(runner_pos, obstacles, runner_size) {
        PURPLE
    } else {
        DARKBLUE
    };
    draw_rectangle(
        runner_pos.x,
        runner_pos.y,
        runner_size.x,
        runner_size.y,
        runner_color,
    );
}

pub fn collided(runner_pos: &Vec2, obstacles: &Vec<Vec2>, size: &Vec2) -> bool {
    let runner_rect = Rect::new(runner_pos.x, runner_pos.y, size.x, size.y);
    for obstacle in obstacles {
        if runner_rect
            .intersect(Rect::new(obstacle.x, obstacle.y, size.x, size.y))
            .is_some()
        {
            return true;
        }
    }
    return false;
}
