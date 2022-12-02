use crate::day2::game::GameResult;
#[derive(Debug, Clone)]
pub enum HandShape {
    Rock,
    Paper,
    Scissors
}

impl HandShape {
    pub fn new(letter: &str) -> Option<HandShape> {
        match letter {
            "A" | "X" => Some(HandShape::Rock),
            "B" | "Y" => Some(HandShape::Paper),
            "C" | "Z" => Some(HandShape::Scissors),
            _ => None
        }
    }
    pub fn compare(&self, opponent: HandShape) -> GameResult {
        match (&self, opponent) {
            (HandShape::Rock, HandShape::Paper) => GameResult::Lose,
            (HandShape::Rock, HandShape::Scissors) => GameResult::Win,
            (HandShape::Paper, HandShape::Rock) => GameResult::Win,
            (HandShape::Paper, HandShape::Scissors) => GameResult::Lose,
            (HandShape::Scissors, HandShape::Rock) => GameResult::Lose,
            (HandShape::Scissors, HandShape::Paper) => GameResult::Win,
            (_,_) => GameResult::Draw
        }
    }
    fn wins(&self) -> HandShape {
        match &self {
            HandShape::Rock => HandShape::Scissors,
            HandShape::Scissors => HandShape::Paper,
            HandShape::Paper => HandShape::Rock
        }
    }
    fn loses(&self) -> HandShape {
        match &self {
            HandShape::Rock => HandShape::Paper,
            HandShape::Scissors => HandShape::Rock,
            HandShape::Paper => HandShape::Scissors
        }
    }
    pub fn get_game_result(&self, result: GameResult) -> HandShape {
        match result {
            GameResult::Draw => self.clone(),
            GameResult::Win => self.wins(),
            GameResult::Lose => self.loses(),
        }
    }
    pub fn score(&self) -> u32 {
        match &self {
            HandShape::Rock => 1,
            HandShape::Paper => 2,
            HandShape::Scissors => 3
        }
    }
}