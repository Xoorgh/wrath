use macroquad::prelude::*;

struct Shape {
    size: f32,
    speed: f32,
    x: f32,
    y: f32,
}

#[macroquad::main("wrath")]
async fn main() {
    // Seed the random number generator
    rand::srand(get_time() as u64);

    // Set the movement speed for the circle
    const MOVEMENT_SPEED: f32 = 200.0;

    // Set initial circle position
    let mut x = screen_width() / 2.0;
    let mut y = screen_height() / 2.0;

    loop {
        // Get the time since the last frame
        let delta_time = get_frame_time();

        // Clear the screen and set the background color
        clear_background(BLUE);

        // Move the circle
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

        // Clamp the circle's position to the screen
        x = x.clamp(16.0, screen_width() - 16.0);
        y = y.clamp(16.0, screen_height() - 16.0);

        // Draw the circle
        draw_circle(x, y, 16.0, YELLOW);

        // Wait for the next frame
        next_frame().await
    }
}
