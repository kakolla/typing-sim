use macroquad::input;
use macroquad::prelude::*;

mod input_utils;
mod textbox;
use input_utils::input_utils::resolve_key;
use textbox::textbox::TextBox;

#[macroquad::main("Type")]
async fn main() {
    request_new_screen_size(1920.0, 1080.0);
    let mut height;
    let mut width;

    // read text
    let text: String = include_str!("texts/first.txt").to_string();
    let curr_index: usize = 0;
    let mut current_letter: &str;

    loop {
        // get current pos
        // current_letter_byte = &text[curr_index as usize];
        current_letter = &text[curr_index..curr_index + 1];
        // deref the byte, cast to char, and borrothw the reference

        // get input
        if let Some(key) = input::get_last_key_pressed() {
            println!(
                "{} was pressed, which is correct: {}, the correct is: {}",
                resolve_key(current_letter, &key).unwrap().1,
                resolve_key(current_letter, &key).unwrap().0,
                current_letter
            );
        }

        height = screen_height();
        width = screen_width();
        clear_background(BLACK);
        // println!("{} {} ", width, height);

        // draw_line(40.0, 40.0, 100.0, 150.0, 60.3, BLUE);
        draw_text(&text, width / 2.0, height / 2.0, 48.0, WHITE);
        next_frame().await;

        #[warn(unused)]
        let _t = TextBox::new(400, 48);
    }
}
