use std::fs;

use rand::Rng;

use crate::constants::flags::{DECODE_FLAG, ENCODE_FLAG};
use crate::models::config::Config;
use crate::services::processes_handler::run_processes;
use crate::services::string_manipulations::divide_words;

pub fn encode(filepath: &str, processes_options: Config) {
    let file_content = fs::read_to_string(filepath).expect("Failed to read file!");
    let words = file_content.split_whitespace().collect();
    let divided_words = divide_words(words, processes_options.processes_count as usize);

    let mut rng = rand::thread_rng();
    let seed: u64 = rng.gen();

    run_processes(
        processes_options,
        divided_words,
        ENCODE_FLAG,
        seed.to_string().as_str(),
    );
}

pub fn decode(filepath: &str, seed: &str, processes_options: Config) {
    let file_content = fs::read_to_string(filepath).expect("Failed to read file!");
    let words = file_content.split_whitespace().collect();
    let divided_words = divide_words(words, processes_options.processes_count as usize);

    run_processes(
        processes_options,
        divided_words,
        DECODE_FLAG,
        seed.to_string().as_str(),
    );
}
