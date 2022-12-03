use crate::helpers::file::{read_lines};
use std::{collections::{HashSet, HashMap}};
fn ascii_to_point(letter: u8) -> u32 {
    match letter {
        65..=90 => {
            letter as u32 - 38 
        },
        97..=122 => {
            letter as u32 - 96
        },
        _ => unreachable!("Letter not in ASCII range: {}", letter)
    }
}
fn part1(lines: Vec<String>) {
    let mut part1_total_score = 0;
    for line in lines {
        let mut duplicate_sack1: HashSet<u8> =  HashSet::new();
        let mut duplicate_sack2: HashSet<u8> = HashSet::new();
        let line: Vec<u8>= line.bytes().collect();
        let half = line.len() / 2;

        let sack1 = &line[0..half];
        let sack2 = &line[half..line.len()];
        let mut samsies: Option<u8> = None;

        for i in 0..half {
            if sack1[i] == sack2[i] || duplicate_sack2.contains(&sack1[i]){
                samsies = Some(sack1[i]);
                break;
            } else if duplicate_sack1.contains(&sack2[i]) {
                samsies = Some(sack2[i]);
                break;
            } else {
                duplicate_sack1.insert(sack1[i]);
                duplicate_sack2.insert(sack2[i]);
            }
        }
        if let Some(samsies) = samsies {
            part1_total_score += ascii_to_point(samsies);
        } else {
            panic!("Duplicate not found");
        }
    }
    println!("Final Score 1 {:?}", part1_total_score); 
}

fn part2(mut lines: Vec<String>) {
    let mut part2_total_score = 0;
    
    while lines.len() > 0 {
        let group:Vec<Vec<u8>> = lines.drain(0..3).map(|s| s.bytes().collect()).collect();
        let mut running_index = 0;
        let mut running_dict: HashMap<u8, u8> = HashMap::new();
        let mut visited_set: Vec<HashSet<u8>> = vec![HashSet::new(), HashSet::new(), HashSet::new()];
        let mut final_num = 0;
        while running_index < group[0].len() || running_index < group[1].len() || running_index < group[2].len() {
            for i in 0..3 {
                if running_index < group[i].len() {
                    let current_char = group[i][running_index];
                    if !visited_set[i].contains(&current_char) {
                        visited_set[i].insert(group[i][running_index]);
                        let count = running_dict.entry(group[i][running_index]).or_insert(0);
                        *count += 1;
                        if *count == 3 {
                            final_num = current_char;
                            break;
                        }
                    }
                }

            }
            running_index = running_index + 1;
        }
        if final_num == 0 {
            println!("{:?}", group);
            println!("{:?}", running_dict);
        }
       
        part2_total_score += ascii_to_point(final_num);
    }
    println!("Final Score 2 {:?}", part2_total_score); 

}
pub fn run(filename: String) {
    let lines = match read_lines(filename) {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
    let lines: Vec<String> = lines.into_iter().map(|s| s.ok().unwrap()).collect();
    part1(lines.clone());
    part2(lines.clone());
}