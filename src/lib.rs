pub struct Config {
    pub verbose: bool,
}

impl Config {
    pub fn default() -> Config {
        Config {
            verbose: false,
        }
    }
}
