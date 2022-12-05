use crate::helpers::file::{read_lines};
pub fn run(filename: String) {
    let lines = match read_lines(filename) {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
    let lines: Vec<String> = lines.into_iter().map(|s| s.ok().unwrap()).collect();
    part1(lines.clone());
    println!("");
    part2(lines);

}

fn part1(lines: Vec<String>) {
    let mut stacks: Vec<Vec<char>> = Vec::new();
    let mut build_stacks = false;
    for mut line in lines { 
        if !build_stacks {
            let mut stacknum = 0;
            loop {
                let chars:String = line.drain(0..3).collect();
                if stacks.len() < (stacknum + 1) {
                    stacks.push(Vec::new());
                }
                let character = chars.chars().nth(1).unwrap();
                if character == '1' {
                    build_stacks = true;
                    break;
                }
                if character != ' ' {
                    stacks[stacknum].push(character);
                }

                stacknum += 1;

                if line.len() > 0 {
                    line.drain(0..1);
                } else {
                    break;
                }
            }
        } else {
            if line.len() > 0 {
                let commands: Vec<&str> = line.split(" ").collect();
                let iterations:usize = commands[1].parse().unwrap();
                let from:usize = commands[3].parse::<usize>().unwrap() - 1;
                let to:usize = commands[5].parse::<usize>().unwrap() - 1;
                for _ in 0..iterations {
                    if let Some(char) = stacks[from].pop() {
                        stacks[to].push(char);
                    }
                }
            } else {
                for i in 0..stacks.len() {
                    stacks[i].reverse();
                }
            }

        }
    }
    for i in 0..stacks.len() {
        if let Some(char) = stacks[i].pop() {
            print!("{}",char);
        }
    }
}
fn part2(lines: Vec<String>) {
    let mut stacks: Vec<Vec<char>> = Vec::new();
    let mut build_stacks = false;
    for mut line in lines {


  
        if !build_stacks {
            let mut stacknum = 0;
            loop {
                let chars:String = line.drain(0..3).collect();
                if stacks.len() < (stacknum + 1) {
                    stacks.push(Vec::new());
                }
                let character = chars.chars().nth(1).unwrap();
                if character == '1' {
                    build_stacks = true;
                    break;
                }
                if character != ' ' {
                    stacks[stacknum].push(character);
                }

                stacknum += 1;

                if line.len() > 0 {
                    line.drain(0..1);
                } else {
                    break;
                }
            }
        } else {
            if line.len() > 0 {
                let commands: Vec<&str> = line.split(" ").collect();
                let iterations:usize = commands[1].parse().unwrap();
                let from:usize = commands[3].parse::<usize>().unwrap() - 1;
                let to:usize = commands[5].parse::<usize>().unwrap() - 1;
                let min = stacks[from].len() - iterations;
                let max = stacks[from].len();
                let mut moving: Vec<char> = stacks[from].drain(min..max).collect();
                stacks[to].append(&mut moving);
            } else {
                for i in 0..stacks.len() {
                    stacks[i].reverse();
                }
            }
        }


    }
    for i in 0..stacks.len() {
        if let Some(char) = stacks[i].pop() {
            print!("{}",char);
        }
    }
}