pub mod input_utils {

    use macroquad::input::KeyCode;

    /// pass the correct key, key that was pressed, and index of current ind in the text
    pub fn resolve_key<'a>(
        correct_key: &'a str,
        key: &KeyCode,
        curr_index: &mut usize,
    ) -> Result<(bool, &'static str), String> {
        let k: &str = match key {
            KeyCode::A => "a",
            KeyCode::B => "b",
            KeyCode::C => "c",
            KeyCode::D => "d",
            KeyCode::E => "e",
            KeyCode::F => "f",
            KeyCode::G => "g",
            KeyCode::H => "h",
            KeyCode::I => "i",
            KeyCode::J => "j",
            KeyCode::K => "k",
            KeyCode::L => "l",
            KeyCode::M => "m",
            KeyCode::N => "n",
            KeyCode::O => "o",
            KeyCode::P => "p",
            KeyCode::Q => "q",
            KeyCode::R => "r",
            KeyCode::S => "s",
            KeyCode::T => "t",
            KeyCode::U => "u",
            KeyCode::V => "v",
            KeyCode::W => "w",
            KeyCode::X => "x",
            KeyCode::Y => "y",
            KeyCode::Z => "z",
            KeyCode::Space => " ",
            _ => "non_letter",
        };
        if k == correct_key {
            *curr_index += 1;
            return Ok((true, k));
        } else {
            return Ok((false, k));
        }
    }
}
