





// input textbox to type
pub mod textbox {

    pub struct TextBox {
        width: u32,
        height: u32
    }


    impl TextBox {
        pub fn new(width: u32, height: u32) -> Self {
            Self {
                width,
                height
            }
        }

    }

}
