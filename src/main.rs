use macroquad::prelude::*;

#[macroquad::main("Cabezazo")]
async fn main() {

    let runner_size = Vec2::new(64.0, 64.0);
    let runner_pos = Vec2::new(screen_width()/2.0 - runner_size.x/2.0, screen_height() - runner_size.y);


    loop {
        clear_background(SKYBLUE);

        // let delta = get_frame_time();
        //
        // if is_key_down(KeyCode::Right) && platform_x < SCR_W - platform_width / 2. {
        //     platform_x += 3.0 * delta;
        // }
        // if is_key_down(KeyCode::Left) && platform_x > platform_width / 2. {
        //     platform_x -= 3.0 * delta;
        // }
        draw_rectangle(runner_pos.x, runner_pos.y, runner_size.x, runner_size.y, WHITE);

        next_frame().await
    }
}
