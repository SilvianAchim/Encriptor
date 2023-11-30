use std::env;

use rand::{rngs::StdRng, SeedableRng, seq::SliceRandom};

fn encode_string(input: &str, seed: u64) -> String {
    let mut rng = StdRng::seed_from_u64(seed);
    let mut chars: Vec<char> = input.chars().collect();
    chars.shuffle(&mut rng);
    chars.into_iter().collect()
}

fn decode_string(input: &str, seed: u64) -> String {
    let mut rng = StdRng::seed_from_u64(seed);
    let mut chars: Vec<char> = input.chars().collect();
    let mut indices: Vec<usize> = (0..chars.len()).collect();

    indices.shuffle(&mut rng);

    let mut decoded_chars = vec![' '; chars.len()];
    for (new_pos, &original_pos) in indices.iter().enumerate() {
        decoded_chars[original_pos] = chars[new_pos];
    }

    return decoded_chars.into_iter().collect()
}


fn main() {
    //NOTE: Should be called with:
    // {String} {Flag} {Strings} -> {Seed} {Flag: {--decode || --encode} {Strings to be decoded},

    let args: Vec<String> = env::args().collect();

    if args.len() < 4 {
        panic!("Invalid number of arguments!");
    }

    let decode = args.iter().any(|arg| arg == "decode");
    let encode = args.iter().any(|arg| arg == "encode");

    if !decode && !encode {
        panic!("Invalid flags!");
    }

    let seed = args[2]
        .parse::<u64>()
        .expect("Failed to convert string to u64");

    for (i, _) in args.iter().enumerate().skip(3){
        if decode {
            print!("{} ", decode_string(args[i].as_str(), seed));
        }
        else{
            print!("{} ", encode_string(args[i].as_str(), seed));
        }
    }
}