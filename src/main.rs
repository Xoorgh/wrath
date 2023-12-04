use macroquad::prelude::*;

struct Shape {
    size: f32,
    speed: f32,
    x: f32,
    y: f32,
}

impl Shape {
    fn collides_with(&self, other: &Self) -> bool {
        self.rect().overlaps(&other.rect())
    }

    fn rect(&self) -> Rect {
        Rect {
            x: self.x - self.size / 2.0,
            y: self.y - self.size / 2.0,
            w: self.size,
            h: self.size,
        }
    }
}

#[macroquad::main("wrath")]
async fn main() {
    // Declare gameover variable
    let mut gameover = false;

    // Seed the random number generator
    rand::srand(get_time() as u64);

    // Set the movement speed for the circle
    const MOVEMENT_SPEED: f32 = 200.0;

    let mut squares = vec![];
    let mut circle = Shape {
        size: 32.0,
        speed: MOVEMENT_SPEED,
        x: screen_width() / 2.0,
        y: screen_height() / 2.0,
    };

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
            circle.x += MOVEMENT_SPEED * delta_time;
        }
        if is_key_down(KeyCode::Left) {
            circle.x -= MOVEMENT_SPEED * delta_time;
        }
        if is_key_down(KeyCode::Down) {
            circle.y += MOVEMENT_SPEED * delta_time;
        }
        if is_key_down(KeyCode::Up) {
            circle.y -= MOVEMENT_SPEED * delta_time;
        }

        // Clamp the circle's position to the screen
        x = x.clamp(circle.size / 2.0, screen_width() - circle.size / 2.0);
        y = y.clamp(circle.size / 2.0, screen_height() - circle.size / 2.0);

        // Randomly generate squares
        if rand::gen_range(0, 99) >= 95 {
            let size = rand::gen_range(16.0, 64.0);
            squares.push(Shape {
                size,
                speed: rand::gen_range(50.0, 150.0),
                x: rand::gen_range(size / 2.0, screen_width() - size / 2.0),
                y: - size,
            });
        }

        // Move the squares
        for square in &mut squares {
            // Move the square
            square.y += square.speed * delta_time;
        }

        // Check if squares are outside the screen and remove them
        squares.retain(|square| square.y < screen_height() + square.size);

        // Draw everything
        draw_circle(circle.x, circle.y, circle.size, YELLOW);
        for square in &squares {
            draw_rectangle(
                square.x - square.size / 2.0,
                square.y - square.size / 2.0,
                square.size,
                square.size,
                RED,
            );
        }

        // Wait for the next frame
        next_frame().await
    }
}
