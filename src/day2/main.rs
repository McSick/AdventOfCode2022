use crate::helpers::file::{read_lines};
use crate::day2::hand::HandShape;
use crate::day2::game::GameResult;
pub fn run(filename: String) {
    let lines = match read_lines(filename) {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    let mut part1_total_score = 0;
    let mut part2_total_score = 0;

    for some_line in lines.into_iter() {
        if let Ok(line) = some_line {
            let split = line.split(" ").collect::<Vec<&str>>();
            let elf_choice = HandShape::new(split[0]);
            let my_choice = HandShape::new(split[1]);
            let game_result_wanted = GameResult::new(split[1]);
            
            match (my_choice, elf_choice) {
                (Some(me), Some(them)) => {
                    let game_result = me.compare(them.clone());
                    part1_total_score = part1_total_score + game_result.score() + me.score();

                    let me2 = them.get_game_result(game_result_wanted);
                    part2_total_score = part2_total_score + me2.score() + me2.compare(them).score();
                },
                (_, _) => panic!("Invalid Input Detected!")
            }
        }
    }
    println!("Final Score 1 {:?}", part1_total_score); 
    println!("Final Score 2 {:?}", part2_total_score); 
}