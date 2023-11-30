use std::fs::File;
use std::io::Write;

pub fn write_to_file(filepath: &str, data: &str) {
    let mut file = File::create(filepath).expect("Could not create file!");

    file.write_all(data.as_bytes())
        .expect("Could not write to file");
}
