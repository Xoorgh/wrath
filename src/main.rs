use macroquad::prelude::*;

#[macroquad::main("wrath")]
async fn main() {
    loop {
        clear_background(DARKPURPLE);
        next_frame().await
    }
}
