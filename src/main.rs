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

    loop {
        // Get the time since the last frame
        let delta_time = get_frame_time();

        // Clear the screen and set the background color
        clear_background(BLUE);

        if !gameover {
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
            circle.x = circle.x.clamp(circle.size, screen_width() - circle.size);
            circle.y = circle.y.clamp(circle.size, screen_height() - circle.size);

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

            // Check for collisions, remove square that collides, reduce circle size and check if circle is too small
            squares.retain(|square| {
                if circle.collides_with(square) {
                    // Reduce the circle's size
                    circle.size -= 2.0;
                    // Check if the circle is too small
                    if circle.size <= 0.0 {
                        // Set gameover to true
                        gameover = true;
                    }
                    // Remove the square
                    false
                } else {
                    // Check if squares are outside the screen
                    square.y < ( screen_height() + square.size )
                }
            });
        }

        // Draw everything

        // Draw the circle
        draw_circle(
            circle.x,
            circle.y,
            circle.size,
            YELLOW
        );

        // Draw the squares
        for square in &squares {
            draw_rectangle(
                square.x - square.size / 2.0,
                square.y - square.size / 2.0,
                square.size,
                square.size,
                RED,
            );
        }

        // Draw health bar outline
        draw_rectangle_lines(
            0.0,
            0.0,
            screen_width() * 0.32 + 2.0,
            12.0,
            12.0,
            BLACK,
        );
        // Draw the health bar background
        draw_rectangle(
            0.0,
            0.0,
            screen_width() * 0.32,
            10.0,
            DARKGRAY,
        );
        // Draw health bar
        draw_rectangle(
            0.0,
            0.0,
            screen_width() * (circle.size / 100.0),
            10.0,
            if circle.size <= 16.0 { RED } else { GREEN },
        );

        // Restart the game if space is pressed
        if gameover && is_key_pressed(KeyCode::Space) {
            // Clear the squares
            squares.clear();
            // Reset the circle
            circle.size = 32.0;
            circle.x = screen_width() / 2.0;
            circle.y = screen_height() / 2.0;
            // Reset the gameover variable
            gameover = false;
        }

        // Draw the gameover text
        if gameover {
            let gameover_text = "Game Over!";
            let restart_text = "Press Space to Restart";
            let gameover_text_dimensions = measure_text(gameover_text, None, 50, 1.0);
            let restart_text_dimensions = measure_text(restart_text, None, 25, 1.0);
            draw_text(
                gameover_text,
                screen_width() / 2.0 - gameover_text_dimensions.width / 2.0,
                screen_height() / 2.0 - gameover_text_dimensions.height / 2.0,
                50.0,
                WHITE,
            );
            draw_text(
                restart_text,
                screen_width() / 2.0 - restart_text_dimensions.width / 2.0,
                screen_height() / 2.0 - restart_text_dimensions.height / 2.0 + 25.0,
                25.0,
                WHITE,
            );
        }

        // Wait for the next frame
        next_frame().await
    }
}
