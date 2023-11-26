use std::fs;

use crate::services::processes_handler::run_processes;
use crate::services::string_manipulations::divide_words;

pub fn encode(filepath: &str, processes_count: u8) {
    let file_content = fs::read_to_string(filepath).expect("Failed to read file!");
    let words = file_content.split_whitespace().collect();
    let divided_words = divide_words(words, processes_count as usize);

    run_processes(
        "../encriptor_process/target/debug/encriptor_process.exe",
        processes_count,
        divided_words,
    );
}

pub fn decode(filepath: &str, seed: &str, processes: u8) {
    let file_content = fs::read_to_string(filepath).expect("Failed to read file!");
    let words = file_content.split_whitespace().collect();
    let divided_words = divide_words(words, processes as usize);

    for (i, vector) in divided_words.iter().enumerate() {
        println!("Vector {}: {:?}", i + 1, vector);
    }
}
