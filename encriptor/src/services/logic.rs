use std::fs;
use crate::services::string_manipulations::divide_words;

pub fn start(filepath: &str, processes: u8) {
    let file_content = fs::read_to_string(filepath)
        .expect("Failed to read file!");
    let words = file_content.split_whitespace().collect();
    let divided_words = divide_words(words, processes as usize);

    for (i, vector) in divided_words.iter().enumerate() {
        println!("Vector {}: {:?}", i + 1, vector);
    }
}