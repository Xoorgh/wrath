use macroquad::prelude::*;
use std::fs;

const FRAGMENT_SHADER: &str = include_str!("starfield-shader.glsl");

const VERTEX_SHADER: &str = "#version 100
attribute vec3 position;
attribute vec2 texcoord;
attribute vec4 color0;
varying float iTime;

uniform mat4 Model;
uniform mat4 Projection;
uniform vec4 _Time;

void main() {
    gl_Position = Projection * Model * vec4(position, 1);
    iTime = _Time.x;
}";

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

enum GameState {
    MainMenu,
    Playing,
    Paused,
    GameOver,
}

#[macroquad::main("wrath")]
async fn main() {
    // Shader stuff
    let mut direction_modifier: f32 = 0.0;
    let render_target = render_target(screen_width() as u32, screen_height() as u32);
    render_target.texture.set_filter(FilterMode::Nearest);
    let material = load_material(
        ShaderSource::Glsl {
            vertex: VERTEX_SHADER,
            fragment: FRAGMENT_SHADER,
        },
        MaterialParams {
            uniforms: vec![
                ("iResolution".to_owned(), UniformType::Float2),
                ("direction_modifier".to_owned(), UniformType::Float1),
            ],
            ..Default::default()
        },
    )
    .unwrap();

    let mut score: u32 = 0;
    let mut high_score: u32 = fs::read_to_string("high_score.dat")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(0);
    let mut game_state = GameState::MainMenu;
    let mut new_high_score = false;
    let mut time_of_last_shot = 0.0;
    let mut fire_rate_multiplier = 1.0;

    rand::srand(get_time() as u64);

    const MOVEMENT_SPEED: f32 = 200.0;
    const CIRCLE_SIZE: f32 = 32.0;
    const FIRE_RATE: f32 = 0.25;
    const BASE_DAMAGE: f32 = 2.0;
    const BASE_SCORE: f32 = 10.0;

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
        // Clear the screen and set the background color
        clear_background(BLACK);

        // Draw the starfield
        material.set_uniform("iResolution", (screen_width(), screen_height()));
        material.set_uniform("direction_modifier", direction_modifier);
        gl_use_material(&material);
        draw_texture_ex(
            &render_target.texture,
            0.,
            0.,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(screen_width(), screen_height())),
                ..Default::default()
            },
        );
        gl_use_default_material();

        match game_state {
            GameState::MainMenu => {
                if is_key_pressed(KeyCode::Escape) {
                    std::process::exit(0);
                }
                if is_key_pressed(KeyCode::Space) {
                    squares.clear();
                    bullets.clear();
                    circle.size = CIRCLE_SIZE;
                    circle.x = screen_width() / 2.0;
                    circle.y = screen_height() / 2.0;
                    score = 0;
                    new_high_score = false;
                    time_of_last_shot = 0.0;
                    game_state = GameState::Playing;
                }
                let title_text = "WRATH";
                let title_text_dimensions = measure_text(title_text, None, 100, 1.0);
                draw_text(
                    title_text,
                    screen_width() / 2.0 - title_text_dimensions.width / 2.0,
                    screen_height() / 2.0 - title_text_dimensions.height / 2.0 - 50.0,
                    100.0,
                    WHITE,
                );
                let begin_playing_text = "Press Space to Begin";
                let begin_playing_text_dimensions = measure_text(begin_playing_text, None, 50, 1.0);
                draw_text(
                    begin_playing_text,
                    screen_width() / 2.0 - begin_playing_text_dimensions.width / 2.0,
                    screen_height() / 2.0 - begin_playing_text_dimensions.height / 2.0,
                    50.0,
                    WHITE,
                );
                let current_high_score_text = &format!("High Score: {}", high_score);
                let current_high_score_text_dimensions = measure_text(current_high_score_text, None, 25, 1.0);
                draw_text(
                    current_high_score_text,
                    screen_width() / 2.0 - current_high_score_text_dimensions.width / 2.0,
                    screen_height() / 2.0 - current_high_score_text_dimensions.height / 2.0 + 50.0,
                    25.0,
                    WHITE,
                );
                let quit_text = "Press Escape to Quit";
                let quit_text_dimensions = measure_text(quit_text, None, 25, 1.0);
                draw_text(
                    quit_text,
                    screen_width() / 2.0 - quit_text_dimensions.width / 2.0,
                    screen_height() / 2.0 - quit_text_dimensions.height / 2.0 + 75.0,
                    25.0,
                    WHITE,
                );
            },
            GameState::Playing => {
                // Get the time since the last frame
                let delta_time = get_frame_time();

                // Go to paused state if escape is pressed
                if is_key_pressed(KeyCode::Escape) {
                    game_state = GameState::Paused;
                }

                // Move the circle
                if is_key_down(KeyCode::Right) {
                    circle.x += MOVEMENT_SPEED * delta_time;
                    direction_modifier += 0.05 * delta_time;
                }
                if is_key_down(KeyCode::Left) {
                    circle.x -= MOVEMENT_SPEED * delta_time;
                    direction_modifier -= 0.05 * delta_time;
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
                    square.y += square.speed * delta_time;
                }

                // Move the bullets
                for bullet in &mut bullets {
                    bullet.y -= bullet.speed * delta_time;
                }

                // Check for collisions, remove square that collides, reduce circle size and check if circle is too small
                squares.retain(|square| {
                    if circle.collides_with(square) {
                        // Reduce the circle's size, not below 0.0
                        circle.size = (circle.size - (BASE_DAMAGE * square.size / 16.0)).round().max(0.0);
                        // Reduce the score, not below 0
                        score = score.saturating_sub((BASE_SCORE * square.size / 16.0).round() as u32);
                        // Check if the circle is too small
                        if circle.size <= 0.0 {
                            // check if current score is a high score
                            if score > high_score {
                                new_high_score = true;
                                // Save the high score
                                fs::write("high_score.dat", score.to_string()).ok();
                            }
                            // Move to gameover state
                            game_state = GameState::GameOver;
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
                            // Increase the score inversely to the square size
                            score += (BASE_SCORE * SQUARE_MAX_SIZE / square.size).round() as u32;
                            // Remove the bullet
                            return false;
                        }
                    }
                    // Check if the bullet is outside the screen
                    bullet.y > 0.0 - bullet.size / 2.0
                });

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

                // Draw HUD outline
                draw_rectangle_lines(
                    0.0,
                    0.0,
                    screen_width(),
                    26.0,
                    12.0,
                    BLACK,
                );
                // Draw HUD background
                draw_rectangle(
                    0.0,
                    0.0,
                    screen_width(),
                    24.0,
                    BROWN,
                );
                // Draw health bar outline, middle of screen
                let health_bar_x = screen_width() / 2.0 - screen_width() * CIRCLE_SIZE / 200.0;
                draw_rectangle_lines(
                    health_bar_x - 2.0,
                    0.0,
                    screen_width() * CIRCLE_SIZE / 100.0 + 4.0,
                    12.0,
                    12.0,
                    BLACK,
                );
                // Draw the health bar background
                draw_rectangle(
                    health_bar_x,
                    0.0,
                    screen_width() * CIRCLE_SIZE / 100.0,
                    10.0,
                    DARKGRAY,
                );
                // Draw health bar
                draw_rectangle(
                    health_bar_x,
                    0.0,
                    screen_width() * (circle.size / 100.0),
                    10.0,
                    if circle.size <= 16.0 { RED } else { GREEN },
                );
                // Draw health bar text
                draw_text(
                    &format!("Health: {:.0}%", circle.size / CIRCLE_SIZE * 100.0),
                    health_bar_x + 2.0,
                    8.0,
                    12.0,
                    BLACK,
                );
                // Draw score text
                draw_text(
                    &format!("Score: {}", score).as_str(),
                    2.0,
                    16.0,
                    24.0,
                    WHITE,
                );
                // Draw high score text
                let score_to_show = if score > high_score { score } else { high_score };
                let high_score_text = &format!("High Score: {}", score_to_show);
                let high_score_font_size = 24.0;
                let high_score_text_dimensions = measure_text(high_score_text, None, high_score_font_size as u16, 1.0);
                draw_text(
                    high_score_text.as_str(),
                    screen_width() - high_score_text_dimensions.width - 2.0,
                    16.0,
                    high_score_font_size,
                    WHITE,
                );
            },
            GameState::Paused => {
                // Return to playing state if escape is pressed
                if is_key_pressed(KeyCode::Escape) {
                    game_state = GameState::Playing;
                }
                let paused_text = "Paused";
                let paused_text_dimensions = measure_text(paused_text, None, 50, 1.0);
                draw_text(
                    paused_text,
                    screen_width() / 2.0 - paused_text_dimensions.width / 2.0,
                    screen_height() / 2.0 - paused_text_dimensions.height / 2.0,
                    50.0,
                    WHITE,
                );
                let resume_text = "Press Escape to Resume";
                let resume_text_dimensions = measure_text(resume_text, None, 25, 1.0);
                draw_text(
                    resume_text,
                    screen_width() / 2.0 - resume_text_dimensions.width / 2.0,
                    screen_height() / 2.0 - resume_text_dimensions.height / 2.0 + 25.0,
                    25.0,
                    WHITE,
                );
            },
            GameState::GameOver => {
                // Return to main menu if escape is pressed
                if is_key_pressed(KeyCode::Escape) {
                    game_state = GameState::MainMenu;
                }
                let gameover_text = "Game Over!";
                let restart_text = "Press Escape to Return to Main Menu";
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
                if new_high_score {
                    let new_high_score_text = "New High Score!";
                    let new_high_score_text_dimensions = measure_text(new_high_score_text, None, 25, 1.0);
                    draw_text(
                        new_high_score_text,
                        screen_width() / 2.0 - new_high_score_text_dimensions.width / 2.0,
                        screen_height() / 2.0 - new_high_score_text_dimensions.height / 2.0 - 50.0,
                        25.0,
                        WHITE,
                    );
                }
            },
        }
        // Wait for the next frame
        next_frame().await
    }
}
