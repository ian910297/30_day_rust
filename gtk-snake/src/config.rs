pub struct Config {
    pub width: i32,
    pub height: i32,
    pub title: &'static str
}

impl Config {
    pub fn new() -> Config {
        Config {
            width: 600,
            height: 800,
            title: "GTK-SNARK"
        }
    }
}