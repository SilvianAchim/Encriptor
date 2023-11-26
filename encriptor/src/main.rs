use std::env;

use crate::services::logic::{decode, encode};
use crate::startup::startup::read_config;

mod models;
mod services;
mod startup;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = read_config();

    match args.len() {
        0 | 1 => panic!("Not enough arguments!"),
        2 => encode(args[1].as_str(), config.processes),
        3 => decode(args[1].as_str(), args[2].as_str(), config.processes),
        _ => panic!("Too Many arguments!"),
    }
}
