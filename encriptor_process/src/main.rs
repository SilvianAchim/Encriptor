use rand::{rngs::StdRng, Rng, SeedableRng, seq::SliceRandom};

fn shuffle_string(input: &str, seed: u64) -> String {
    let mut rng = StdRng::seed_from_u64(seed);

    let mut chars: Vec<char> = input.chars().collect();
    chars.shuffle(&mut rng);
    return chars.into_iter().collect()
}

fn decode_string(shuffled: &str, seed: u64) -> String {
    let mut rng = StdRng::seed_from_u64(seed);

    let mut chars: Vec<char> = shuffled.chars().collect();
    let mut indices: Vec<usize> = (0..chars.len()).collect();

    indices.shuffle(&mut rng);

    let mut decoded_chars = vec![' '; chars.len()];
    for (new_pos, &original_pos) in indices.iter().enumerate() {
        decoded_chars[original_pos] = chars[new_pos];
    }

    return decoded_chars.into_iter().collect()
}


fn main() {
    let original_string = "Hello, World!";

    let mut rng = rand::thread_rng();
    let seed: u64 = rng.gen();

    let shuffled_string = shuffle_string(original_string, seed);

    println!("Original: {}", original_string);
    println!("Shuffled: {}", shuffled_string);
    println!("Seed used: {}", seed);


    let test = decode_string(shuffled_string.as_str(), seed);
    println!("Output: {}",test);
}
