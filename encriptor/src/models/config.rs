use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub processes: u8,
}
