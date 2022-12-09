use std::collections::HashSet;

use crate::helpers::file::read_lines;

pub fn run(filename: String) {
    let lines = match read_lines(filename) {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
    let commands: Vec<Command> = lines.into_iter().map(|s| Command::new(s.ok().unwrap())).collect();
    let mut visited_part1 = HashSet::<Coord>::new();
    let mut rope1 = vec![Coord::new(0,0);2];

    let mut visited_part2 = HashSet::<Coord>::new();
    let mut rope2 = vec![Coord::new(0,0);10];

    for command in commands {
        rope1 = command.apply(&mut rope1.clone(),&mut visited_part1);
        rope2 = command.apply(&mut rope2.clone(),&mut visited_part2);
    }
    println!("Visted p1: {}", visited_part1.len());
    println!("Visted p2: {}", visited_part2.len());
}
#[derive(Eq, Hash, PartialEq, Clone, Debug)]
struct Coord {
    x: i32,
    y: i32
}
impl Coord {
    fn new(x: i32, y: i32) -> Coord {
        Coord { x: x, y: y}
    }
    fn add(&mut self, new: Coord) {
        self.x += new.x;
        self.y += new.y;
    }
    fn is_touching(&self, other: &Coord) -> bool {
        (self.x - other.x).abs() <= 1 && (self.y - other.y).abs() <= 1
    }
    fn move_to(&mut self, next: &Coord) {
        let mut x_diff = next.x - self.x;
        let mut y_diff = next.y - self.y;
        if x_diff == 2 || x_diff == -2{
            x_diff = x_diff / 2;
        }
        if y_diff == 2 || y_diff == -2 {
            y_diff = y_diff / 2;
        }
        self.add(Coord::new(x_diff, y_diff));
    }
}

struct Command {
    direction: Dir,
    amount: i32
}
impl Command {
    fn new(str: String) -> Command {
        let split: Vec<&str> = str.split(" ").collect::<Vec<&str>>();
        Command {
            direction: Dir::new(split[0]),
            amount: split[1].parse::<i32>().unwrap()
        }
    }
    fn apply(&self, rope: &mut Vec<Coord>, visited:&mut HashSet<Coord>) -> Vec<Coord> {
        let size = rope.len();
        for _ in 0..self.amount {
            rope[0].add(self.direction.coord());
            for i in 1..(size) {
                let head = rope[i-1].clone();
                if !rope[i-1].is_touching(&rope[i]) {
                    rope[i].move_to(&head);
                }
            }
            visited.insert(rope[size-1].clone());
        }
        rope.clone()
    }
}

enum Dir {
    Up,
    Down,
    Left,
    Right
}
impl Dir {
    fn new(str: &str) -> Dir {
        match str {
            "U" => Dir::Up,
            "D" => Dir::Down,
            "L" => Dir::Left,
            "R" => Dir::Right,
            _ => unreachable!("Invalid Direction given: {}", str)
        }
    }
    fn coord(&self) -> Coord {
        match &self {
            Dir::Up => Coord::new(0, 1),
            Dir::Down => Coord::new(0,-1),
            Dir::Left => Coord::new(-1,0),
            Dir::Right => Coord::new(1,0)
        }
    }
}