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
    let text1: String = include_str!("texts/first.txt").to_string();
    let mut curr_index: usize = 0;
    let mut current_letter: &str;

    let text2: String = include_str!("texts/second.txt").to_string();
    let texts: [String; 2] = [text1, text2];

    let mut i: usize = 0;
    let mut text: &String;

    loop {
        // revolve between texts
        text = &texts[i % 2];

        // get current pos
        // current_letter_byte = &text[curr_index as usize];
        current_letter = &text[curr_index..curr_index + 1];
        // deref the byte, cast to char, and borrothw the reference

        // get input
        if let Some(key) = input::get_last_key_pressed() {
            let _res = resolve_key(current_letter, &key, &mut curr_index).unwrap();

            // println!(
            //     "{} was pressed, which is correct: {}, the correct is: {}",
            //     _res.1, _res.0, current_letter
            // );
        }

        height = screen_height();
        width = screen_width();
        clear_background(BLACK);
        // println!("{} {} ", width, height);

        // draw_line(40.0, 40.0, 100.0, 150.0, 60.3, BLUE);
        // draw_text(&text, width / 2.0, height / 2.0, 48.0, WHITE);
        draw_text(&text[curr_index..], width / 2.0, height / 2.0, 48.0, WHITE);
        next_frame().await;

        #[warn(unused)]
        let _t = TextBox::new(400, 48);
        // check win
        if check_win(&text, curr_index) {
            i += 1;
            curr_index = 0;
        }
    }
}

/// check if all letters are typed, return true
fn check_win(text: &String, curr_index: usize) -> bool {
    if text.len() == curr_index + 1 {
        return true;
    } else {
        return false;
    }
}
