
use crate::helpers::file::{read_lines};
use crate::helpers::elf::{Elf};


pub fn run(filename: String) {
    let mut elfs: Vec<Elf> = Vec::new();
    let mut index: usize = 0;

    let lines = match read_lines(filename) {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    for some_line in lines.into_iter() {
        if let Ok(line) = some_line {
            if elfs.len() == 0 {
                elfs.push(Elf::new(index));
            }
            if line.len() > 0 {
                elfs[index].add_calorie(line.parse().ok().unwrap_or(0));
            } else {
                index = index + 1;
                elfs.push(Elf::new(index));
            }
        }
    } 
    
    elfs.sort();
    elfs.reverse();


    let top_1 = &elfs[..1];
    let top_3 = &elfs[..3];
    println!("Top 1: {}", top_1.iter().map(|s| s.calorie_total()).sum::<u32>());    
    println!("Top 3: {}", top_3.iter().map(|s| s.calorie_total()).sum::<u32>());

    
}


