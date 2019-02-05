#[derive(PartialEq)]
pub enum State {
    Init,
    Running,
    Pause,
    End
}

#[derive(PartialEq, Debug)]
pub enum Direction {
    Up,
    Down,
    Right,
    Left
}

pub struct Game {
    score: i32,
    time: i32,
    pub speedup: bool,
    pub direction: Direction,
    pub state: State
}

impl Game {
    pub fn new() -> Self {
        Game {
            score: 0,
            time: 100,
            speedup: false,
            direction: Direction::Down,
            state: State::Init
        }
    }

    pub fn run(&mut self) {
        self.state = State::Running
    }

    pub fn pause(&mut self) {
        self.state = State::Pause;
    }

    pub fn reset(&mut self) {
        self.score = 0;
        self.time = 100;
        self.state = State::Init;
    }
    
    pub fn update(&mut self) -> (String, String) {
        self.score += 1;
        self.time -= 1;

        (format!("Score: {}", self.score), format!("Time: {}", self.time))
    }
}