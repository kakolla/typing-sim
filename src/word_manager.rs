pub mod word_manager {
    use std::fs::File;
    use std::io;
    use std::io::{BufRead, BufReader};

    // random trait
    use rand::seq::SliceRandom;
    // use rand::thread_rng;

    pub struct WordManager {
        /// words from the file
        known_words: Vec<String>,
        /// words for the current game
        words: Vec<String>,
    }

    impl WordManager {
        pub fn new() -> Self {
            Self {
                known_words: Vec::new(),
                words: Vec::new(),
            }
        }
        pub fn init_words(&mut self, file_name: String) -> io::Result<()> {
            // read from file
            // let file = File::open("texts/words.txt")?;
            let file = File::open(file_name)?;
            let reader = BufReader::new(file); // allows buffered reading
            //

            for word in reader.lines() {
                let word = word?;
                self.known_words.push(word);
            }
            Ok(())
        }

        pub fn assort_words(&mut self) -> () {
            let mut rng = rand::rng();
            // shuffle it
            self.known_words.shuffle(&mut rng);
            self.words = self.known_words[..25]
                .iter()
                .map(|s| s.to_string())
                .collect();
        }

        pub fn get_entire_word_text(&self) -> String {
            let mut s: String = String::from("");
            for word in &self.words {
                s.push_str(&word);
                s.push_str(" ");
            }
            s.pop(); // pop last space
            s
        }
    }
}
