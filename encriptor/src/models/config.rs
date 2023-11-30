use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub processes_count: u8,
    pub output_filepath: String,
    pub process_filepath: String,
}
