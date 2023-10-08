pub struct Config {
    address: String,
    port: u16,
}

impl Config {
    pub fn new(address: str, port: u16) -> Self {
        Config { address, port }
    }
}
