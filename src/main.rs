use macroquad::prelude::*;

#[macroquad::main("Type")]
async fn main() {
    let game_done: bool = false;
    while !game_done {
        clear_background(BLACK);

        draw_line(40.0, 40.0, 100.0, 150.0, 60.3, BLUE);
        draw_text("welcome", 300.0, 800.0, 16.0, WHITE);
        next_frame().await;
    }
}
