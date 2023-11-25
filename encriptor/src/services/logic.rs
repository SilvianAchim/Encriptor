use crate::services::files_handler::open_file;
use crate::services::string_manipulations::divide_words;

pub fn start(filepath: &str, processes: u8) {
    let content = open_file(filepath);
    let words = content.split_whitespace().collect();
    let divided_words = divide_words(words, processes as usize);

    for (i, vector) in divided_words.iter().enumerate() {
        println!("Vector {}: {:?}", i + 1, vector);
    }
}