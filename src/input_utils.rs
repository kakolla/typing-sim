pub mod input_utils {

    use macroquad::input::KeyCode;

    pub fn resolve_key<'a>(
        correct_key: &'a str,
        key: &KeyCode,
    ) -> Result<(bool, &'static str), String> {
        let k: &str = match key {
            KeyCode::A => "A",
            KeyCode::B => "B",
            KeyCode::C => "C",
            KeyCode::D => "D",
            KeyCode::E => "E",
            KeyCode::F => "F",
            KeyCode::G => "G",
            KeyCode::H => "H",
            KeyCode::I => "I",
            KeyCode::J => "J",
            KeyCode::K => "K",
            KeyCode::L => "L",
            KeyCode::M => "M",
            KeyCode::N => "N",
            KeyCode::O => "O",
            KeyCode::P => "P",
            KeyCode::Q => "Q",
            KeyCode::R => "R",
            KeyCode::S => "S",
            KeyCode::T => "T",
            KeyCode::U => "U",
            KeyCode::V => "V",
            KeyCode::W => "W",
            KeyCode::X => "X",
            KeyCode::Y => "Y",
            KeyCode::Z => "Z",
            KeyCode::Space => " ",
            _ => "",
        };
        if k == correct_key {
            return Ok((true, k));
        } else {
            return Ok((false, k));
        }
    }
}
