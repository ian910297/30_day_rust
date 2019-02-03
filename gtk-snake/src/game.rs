pub struct Game {
    score: i32
}

impl Game {
    pub fn new() -> Self {
        Game {
            score: 0
        }
    }

    pub fn increase_score(&mut self) -> String {
        self.score += 1;
        format!("Score: {}", self.score)
    }
}