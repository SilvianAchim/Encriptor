use crate::services::logic::start;
use crate::startup::startup::read_config;

mod models;
mod startup;
mod services;

fn main() {
    let config = read_config();
    println!("Processes: {}", config.processes);

    let file_path = "src/input.txt";

    start(file_path, config.processes);

    println!("Hello world1!");
}

