use macroquad::prelude::*;

const DEFAULT_WINDOW_TITLE : &'static str = "Cabezazo";
const DEFAULT_WINDOW_WIDTH : i32 = 640;
const DEFAULT_WINDOW_HEIGHT : i32 = 640;


#[macroquad::main(window_conf)]
async fn main() {

    let runner_size = Vec2::new(64.0, 64.0);
    let mut runner_pos = Vec2::new(screen_width()/2.0 - runner_size.x/2.0, screen_height() - runner_size.y);

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

        if frame_count %60 ==0 {
            obstacles.push(Vec2::new((seed % ((screen_width() - runner_size.x) as i32)) as f32, 0.0));
            seed = (seed + (get_frame_time() * 1000000.0) as i32) % 10000;
            seed = seed * seed;
        }

        update_runner_pos(&mut runner_pos, right_limit, left_limit);

        draw_rectangle(runner_pos.x, runner_pos.y, runner_size.x, runner_size.y, DARKBLUE);
        let mut to_remove = Vec::new();

        let delta = get_frame_time();
        for (i,  obstacle ) in &mut obstacles.iter_mut().enumerate() {
            draw_rectangle(obstacle.x, obstacle.y, runner_size.x, runner_size.y, RED);
            obstacle.y += 200.0 * delta;
            if obstacle.y > bottom_limit {
                to_remove.push(i);
            }
        }
        for i in to_remove {
            obstacles.remove(i);
        }

        next_frame().await
    }
}

fn increase_frame(frame_count :&mut i32) {
    *frame_count += 1;
    const MAX_FRAME :i32 = 10000;
    if *frame_count > MAX_FRAME {
        *frame_count -= MAX_FRAME;
    }
}

fn update_runner_pos(runner_pos: &mut Vec2, right_limit: f32, left_limit: f32) {
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

fn window_conf() -> Conf {
    Conf {
        window_title: DEFAULT_WINDOW_TITLE.to_owned(),
        window_width: DEFAULT_WINDOW_WIDTH,
        window_height: DEFAULT_WINDOW_HEIGHT,
        ..Default::default()
    }
}
