use crate::textures::Textures;
use macroquad::prelude::*;

const HITBOX_RADIUS: f32 = 0.90;
const DEBUG_HITBOXES: bool = false;

pub fn maybe_add_obstacles(
    runner_size: Vec2,
    obstacles: &mut Vec<Vec2>,
    frame_count: i32,
    seed: &mut i32,
    difficulty: i32,
) {
    if frame_count % difficulty == 0 {
        //eprintln!("added obstacle. current difficulty is {}", difficulty);
        obstacles.push(Vec2::new(
            (*seed % ((screen_width() + runner_size.x) as i32)) as f32 - runner_size.x,
            -runner_size.y,
        ));
        let microseconds = (get_frame_time() * 1_000_000.0) as i32;
        *seed *= 39;
        *seed += microseconds;
        if *seed > 1_000_000 {
            *seed = *seed % 1_000_000;
        }
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

fn is_click_right(pos_local: Vec2) -> bool {
    pos_local.y >= 0.0 && pos_local.x >= 0.0
}
fn is_click_left(pos_local: Vec2) -> bool {
    pos_local.y >= 0.0 && pos_local.x < 0.0
}

fn is_click_up(pos_local: Vec2) -> bool {
    pos_local.y < 0.0
}

pub fn update_runner_pos(runner_pos: &mut Vec2, right_limit: f32, left_limit: f32) {
    let delta = get_frame_time();

    if is_key_down(KeyCode::Right)
        || touches_local().iter().any(|t| is_click_right(t.position))
        || (is_mouse_button_down(MouseButton::Left) && is_click_right(mouse_position_local()))
    {
        runner_pos.x += 300.0 * delta;
    }
    if runner_pos.x > right_limit {
        runner_pos.x = right_limit;
    }

    if is_key_down(KeyCode::Left)
        || touches_local().iter().any(|t| is_click_left(t.position))
        || (is_mouse_button_down(MouseButton::Left) && is_click_left(mouse_position_local()))
    {
        runner_pos.x -= 300.0 * delta;
    }
    if runner_pos.x < left_limit {
        runner_pos.x = left_limit;
    }
}

pub fn should_headbutt(collided: bool) -> bool {
    !collided
        && (is_key_down(KeyCode::Up)
            || touches_local().iter().any(|t| is_click_up(t.position))
            || (is_mouse_button_down(MouseButton::Left) && is_click_up(mouse_position_local())))
}

pub fn draw_obstacles(obstacles: &Vec<Vec2>, texture: &Texture2D, frame_count: i32) {
    for obstacle in obstacles.iter().rev() {
        let mut params = DrawTextureParams::default();
        let flipped = frame_count / 20 % 2 == 0;
        params.flip_x = flipped;
        draw_texture_ex(*texture, obstacle.x, obstacle.y, WHITE, params);
        maybe_debug_hitbox(obstacle, texture)
    }
}

fn maybe_debug_hitbox(pos_top_left_corner: &Vec2, texture: &Texture2D) {
    if DEBUG_HITBOXES {
        draw_circle(
            pos_top_left_corner.x + texture.width() / 2.0,
            pos_top_left_corner.y + texture.height() / 2.0,
            texture.width() / 2.0 * HITBOX_RADIUS,
            Color::new(0.5, 0.5, 0.2, 0.5),
        );
    }
}

pub fn draw_runner(
    runner_pos: &Vec2,
    textures: &Textures,
    frame_count: i32,
    collided: bool,
    runner_lives: i32,
    stamina: f32,
) {
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

    draw_stamina(runner_pos, stamina, texture);
    maybe_debug_hitbox(runner_pos, texture);
}

fn draw_stamina(runner_pos: &Vec2, stamina: f32, texture: &Texture2D) {
    if stamina < 1.0 {
        let stamina_height: f32 = 20.0;
        draw_rectangle_lines(
            runner_pos.x,
            runner_pos.y - stamina_height * 2.0,
            texture.width(),
            stamina_height,
            2.0,
            BLACK,
        );
        draw_rectangle(
            runner_pos.x + 1.0,
            runner_pos.y - stamina_height * 2.0 + 1.0,
            (texture.width() - 2.0) * stamina,
            stamina_height - 2.0,
            VIOLET,
        );
    }
}

/// Use Pythagoras for a circle hitbox
pub fn did_collide(runner_pos: &Vec2, obstacles: &Vec<Vec2>, size: &Vec2) -> (bool, usize) {
    let squared_diameter = size.x * size.x * HITBOX_RADIUS * HITBOX_RADIUS;
    for (i, obstacle) in obstacles.iter().enumerate() {
        let distance_x = runner_pos.x - obstacle.x;
        let distance_y = runner_pos.y - obstacle.y;
        let squared_distance = distance_x * distance_x + distance_y * distance_y;
        if squared_distance < squared_diameter {
            return (true, i);
        }
    }
    return (false, 0);
}
