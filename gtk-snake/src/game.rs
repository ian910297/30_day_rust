pub struct State {
    pub run: bool
}

impl State {
    fn new() -> Self {
        State {
            run: false
        }
    }
}

pub struct Game {
    score: i32,
    time: i32,
    state: State
}

impl Game {
    pub fn new() -> Self {
        Game {
            score: 0,
            time: 100,
            state: State::new()
        }
    }

    pub fn is_running(&self) -> bool {
        if self.state.run == true {
            true
        } else {
            false
        }
    }

    pub fn run(&mut self) {
        self.state.run = true;
    }

    pub fn pause(&mut self) {
        self.state.run = false;
    }

    pub fn reset(&mut self) {
        self.score = 0;
        self.time = 100;
        self.state.run = true;
    }
    
    pub fn update(&mut self) -> (String, String) {
        self.score += 1;
        self.time -= 1;

        (format!("Score: {}", self.score), format!("Time: {}", self.time))
    }
}