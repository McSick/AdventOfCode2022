use crate::helpers::file::read_lines;
use std::collections::HashMap;
pub fn run(filename: String) {
    let lines = match read_lines(filename) {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
    let lines: Vec<String> = lines.into_iter().map(|s| s.ok().unwrap()).collect();
    let mut current_path: String = "root".to_string();
    let mut directories: HashMap<String, u32> = HashMap::new();
    directories.insert(current_path.clone(), 0);
    let mut add_to_current_dir: bool = false;

    for line in lines {
        let split: Vec<&str> = line.split(' ').collect();
        match split[0] {
            "$" => {
                add_to_current_dir = false;
                match split[1] {
                    "ls" => {
                        add_to_current_dir = true;
                    }
                    _ => {
                        match split[2] {
                            ".." => {
                                let copy = current_path.clone();
                                let up = copy.rfind('/');
                                if let Some(index) = up {
                                    let (left, _) = copy.split_at(index);
                                    current_path = left.to_string();
                                }
                            }
                            "/" => {
                                current_path = "root".to_string();
                            }
                            _ => {
                                let dirname = split[2];
                                current_path.push('/');
                                current_path.push_str(dirname);
                            }
                        }
                    }
                }
            }
            "dir" => {}
            _ => {
                if add_to_current_dir {
                    let filesize: u32 = split[0].parse().unwrap();
                    let mut copy_of_path = current_path.clone();
                    loop {
                        let size = directories.entry(copy_of_path.clone()).or_insert(0);
                        *size += filesize;
                        let up = copy_of_path.rfind('/');
                        if let Some(index) = up {
                            let (left, _) = copy_of_path.split_at(index);
                            copy_of_path = left.to_string();
                        } else {
                            break;
                        }
                    }
                }
            }
        }
    }

    //part 1
    let mut sum = 0;
    let mut values: Vec<u32> = directories.into_values().collect();
    for val in values.clone() {
        if val <= 100000 {
            sum = sum + val;
        }
    }

    //part 2
    const TOTALSPACE:u32 = 70000000;
                           
    const UNUSEDSPACE:u32 = 30000000;
    values.sort_by(|a,b| a.cmp(b));
    let used_space = values[values.len() - 1];
    let mut smallest_space = 0;
    for val in values {
        if TOTALSPACE -( used_space - val) >= UNUSEDSPACE {
            smallest_space = val;
            break;
        }
    }

    println!("Sum: {:?}", sum);
    println!("Smallest Allowed: {:?}", smallest_space);
}

