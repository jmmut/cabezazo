use macroquad::prelude::*;

const DEFAULT_WINDOW_TITLE : &'static str = "Cabezazo";
const DEFAULT_WINDOW_WIDTH : i32 = 640;
const DEFAULT_WINDOW_HEIGHT : i32 = 640;


#[macroquad::main(window_conf)]
async fn main() {

    let runner_size = Vec2::new(64.0, 64.0);
    let mut runner_pos = Vec2::new(screen_width()/2.0 - runner_size.x/2.0, screen_height() - runner_size.y);

    let right_limit = screen_width() - runner_size.x;
    let left_limit = 0.0;
    loop {
        clear_background(BEIGE);

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
        if is_key_down(KeyCode::Escape) {
            break;
        }

        draw_rectangle(runner_pos.x, runner_pos.y, runner_size.x, runner_size.y, DARKBLUE);

        next_frame().await
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
