use std::env;

use rand::{Rng, rngs::StdRng, SeedableRng, seq::SliceRandom};

fn shuffle_string(input: &str, seed: u64) -> String {
    let mut rng = StdRng::seed_from_u64(seed);

    return input
        .split_whitespace()
        .map(|word| {
            let mut chars: Vec<char> = word.chars().collect();
            chars.shuffle(&mut rng);
            chars.into_iter().collect::<String>()
        })
        .collect::<Vec<String>>()
        .join(" ")
}

fn decode_string(shuffled: &str, seed: u64) -> String {
    let mut rng = StdRng::seed_from_u64(seed);

    return shuffled.split_whitespace()
        .map(|word| {
            let mut chars: Vec<char> = word.chars().collect();
            let mut indices: Vec<usize> = (0..chars.len()).collect();

            indices.shuffle(&mut rng);

            let mut decoded_chars = vec![' '; chars.len()];
            for (new_pos, &original_pos) in indices.iter().enumerate() {
                decoded_chars[original_pos] = chars[new_pos];
            }

            decoded_chars.into_iter().collect::<String>()
        })
        .collect::<Vec<String>>()
        .join(" ")
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        0 | 1 => panic!("Not enough arguments!"),
        2 => {
            let mut rng = rand::thread_rng();
            let seed: u64 = rng.gen();
            println!("{} {}", shuffle_string(args[1].as_str(), seed), seed);
        }
        3 => {
            let seed = args[2]
                .parse::<u64>()
                .expect("Failed to convert string to u64");

            println!("{}", decode_string(args[1].as_str(), seed))
        }
        _ => panic!("Too Many arguments!"),
    }
}
