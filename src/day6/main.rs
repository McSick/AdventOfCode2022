use std::collections::HashSet;

pub fn run(stream: String) {
    let characters = stream.chars().collect::<Vec<char>>();
    print_result(start_of(&characters, 4));
    print_result(start_of(&characters, 14));
}
fn start_of(chars: &Vec<char>, distinct_characters: usize) -> Option<usize> {
    let mut current_set: HashSet<char> = HashSet::new();
    for i in 0..(chars.len()-distinct_characters) {
        current_set.clear();
        for j in 0..distinct_characters {
            current_set.insert(chars[i+j]);
        }
        if current_set.len() == distinct_characters {
            return Some(i+distinct_characters);
        }
    }
    None
}
fn print_result(result: Option<usize>) {
    match result {
        Some(i) => println!("Buffer found at: {}", i),
        None => println!("Start of buffer not found!")
    }
}
