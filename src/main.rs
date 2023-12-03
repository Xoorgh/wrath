use macroquad::prelude::*;

#[macroquad::main("wrath")]
async fn main() {
    const MOVEMENT_SPEED: f32 = 200.0;

    let mut x = screen_width() / 2.0;
    let mut y = screen_height() / 2.0;

    loop {
        let delta_time = get_frame_time();

        clear_background(BLUE);

        if is_key_down(KeyCode::Right) {
            x += MOVEMENT_SPEED * delta_time;
        }
        if is_key_down(KeyCode::Left) {
            x -= MOVEMENT_SPEED * delta_time;
        }
        if is_key_down(KeyCode::Down) {
            y += MOVEMENT_SPEED * delta_time;
        }
        if is_key_down(KeyCode::Up) {
            y -= MOVEMENT_SPEED * delta_time;
        }

        x = x.clamp(16.0, screen_width() - 16.0);
        y = y.clamp(16.0, screen_height() - 16.0);

        draw_circle(x, y, 16.0, YELLOW);

        next_frame().await
    }
}
