#[derive(Debug)]
pub enum GameResult {
    Win,
    Lose,
    Draw
}

impl GameResult {
    pub fn score(&self) -> u32 {
        match &self {
            GameResult::Win => 6,
            GameResult::Lose => 0,
            GameResult::Draw => 3
        }
    }
    pub fn new(letter: &str) -> GameResult {
        match letter {
            "X" => GameResult::Win,
            "Y" => GameResult::Draw,
            _ => GameResult::Lose
        }
    }
}