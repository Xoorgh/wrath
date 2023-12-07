use macroquad::prelude::*;

struct Shape {
    size: f32,
    speed: f32,
    x: f32,
    y: f32,
    collided: bool,
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
    let mut gameover = false;
    let mut time_of_last_shot = 0.0;
    let mut fire_rate_multiplier = 1.0;

    rand::srand(get_time() as u64);

    const MOVEMENT_SPEED: f32 = 200.0;
    const CIRCLE_SIZE: f32 = 32.0;
    const FIRE_RATE: f32 = 0.25;
    const BASE_DAMAGE: f32 = 2.0;

    const SQUARE_MAX_SIZE: f32 = 64.0;
    const SQUARE_MIN_SIZE: f32 = 16.0;

    let mut squares = vec![];
    let mut bullets: Vec<Shape> = vec![];
    let mut circle = Shape {
        size: CIRCLE_SIZE,
        speed: MOVEMENT_SPEED,
        x: screen_width() / 2.0,
        y: screen_height() / 2.0,
        collided: false,
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

            // Determine fire rate multiplier, inverse of circle size to a max of 2.0
            fire_rate_multiplier = (CIRCLE_SIZE / circle.size).min(2.0);

            // Shoot bullets
            if is_key_pressed(KeyCode::Space) && get_time() - time_of_last_shot > (FIRE_RATE / fire_rate_multiplier).into() {
                bullets.push(Shape {
                    size: 4.0,
                    speed: MOVEMENT_SPEED * 2.0,
                    x: circle.x,
                    y: circle.y,
                    collided: false,
                });
                time_of_last_shot = get_time();
            }

            // Clamp the circle's position to the screen
            circle.x = circle.x.clamp(circle.size, screen_width() - circle.size);
            circle.y = circle.y.clamp(circle.size, screen_height() - circle.size);

            // Randomly generate squares
            if rand::gen_range(0, 99) >= 95 {
                let size = rand::gen_range(SQUARE_MIN_SIZE, SQUARE_MAX_SIZE);
                squares.push(Shape {
                    size,
                    speed: rand::gen_range(50.0, 150.0),
                    x: rand::gen_range(size / 2.0, screen_width() - size / 2.0),
                    y: - size,
                    collided: false,
                });
            }

            // Move the squares
            for square in &mut squares {
                // Move the square
                square.y += square.speed * delta_time;
            }

            // Move the bullets
            for bullet in &mut bullets {
                // Move the bullet
                bullet.y -= bullet.speed * delta_time;
            }

            // Check for collisions, remove square that collides, reduce circle size and check if circle is too small
            squares.retain(|square| {
                if circle.collides_with(square) {
                    // Reduce the circle's size
                    circle.size -= BASE_DAMAGE * square.size / 16.0;
                    // Check if the circle is too small
                    if circle.size <= 0.0 {
                        // Set gameover to true
                        gameover = true;
                    }
                    // Remove the square
                    false
                } else {
                    // Check if squares are outside the screen or have been hit by a bullet
                    square.y < ( screen_height() + square.size ) && !square.collided
                }
            });

            // Check for collisions, remove bullet that collides, remove square that collides
            bullets.retain(|bullet| {
                // Check if the bullet collides with a square
                for square in &mut squares {
                    if bullet.collides_with(square) {
                        // Set the collided variable to true
                        square.collided = true;
                        // Increase circle size to a max of CIRCLE_SIZE, inverse of square size
                        circle.size = (circle.size + (SQUARE_MAX_SIZE / square.size)).min(CIRCLE_SIZE);
                        // Remove the bullet
                        return false;
                    }
                }
                // Check if the bullet is outside the screen
                bullet.y > 0.0 - bullet.size / 2.0
            });
        }

        // Draw everything

        // Draw the bullets
        for bullet in &bullets {
            draw_rectangle(
                bullet.x,
                bullet.y,
                bullet.size,
                bullet.size,
                GREEN,
            );
        }

        // Draw the circle
        draw_circle(
            circle.x,
            circle.y,
            circle.size / 2.0,
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
            screen_width() * CIRCLE_SIZE / 100.0 + 2.0,
            12.0,
            12.0,
            BLACK,
        );
        // Draw the health bar background
        draw_rectangle(
            0.0,
            0.0,
            screen_width() * CIRCLE_SIZE / 100.0,
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
            // Clear the bullets
            bullets.clear();
            // Reset the circle
            circle.size = CIRCLE_SIZE;
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
