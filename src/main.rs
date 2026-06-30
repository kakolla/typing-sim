use macroquad::prelude::*;
use macroquad::input;
use macroquad::input::KeyCode;

#[macroquad::main("Type")]
async fn main() {
    request_new_screen_size(1920.0, 1080.0);
    let mut height;
    let mut width;
    request_new_screen_size(1920.0, 1080.0);

    // read text
    let text = include_str!("texts/first.txt");

    loop {
        // get input
        if let Some(key) = input::get_last_key_pressed() {
            println!("{} was pressed", resolve_key(&key));
        }

        height = screen_height();
        width = screen_width();
        clear_background(BLACK);
        // println!("{} {} ", width, height);

        // draw_line(40.0, 40.0, 100.0, 150.0, 60.3, BLUE);
        draw_text(&text, width / 2.0, height / 2.0, 48.0, WHITE);
        next_frame().await;
    }
}

pub fn resolve_key(key: &KeyCode) -> String {
    match key {
        KeyCode::A => {
            "A".to_string()
        },
        KeyCode::B => {
            "B".to_string()
        },
        _ => {
            "NA".to_string()
        }

    }




}
