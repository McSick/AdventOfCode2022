use crate::helpers::file::read_lines;

pub fn run(filename: String) {
    let lines = match read_lines(filename) {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
    let lines: Vec<String> = lines.into_iter().map(|s| s.ok().unwrap()).collect();
    let forest = parse_input(lines);
    let size = forest.len();
    let mut visible = (2 * size) + (2 * (size - 2));
    let mut best_score = 0;
    for i in 1..(forest.len() - 1) {
        for j in 1..(forest[i].len() - 1) {
            let view = View {
                up: check_visibility(MoveDirection::Up, &forest, (i, j)),
                down: check_visibility(MoveDirection::Down, &forest, (i, j)),
                left: check_visibility(MoveDirection::Left, &forest, (i, j)),
                right: check_visibility(MoveDirection::Right, &forest, (i, j))
            };
            if view.up.0 || view.down.0 || view.right.0 || view.left.0
            {
                visible += 1;
            }
            let score = view.up.1 * view.down.1 * view.right.1 * view.left.1;
            if score > best_score {
                best_score = score;
            }
        }
    }
    println!("Visible: {:?}", visible);
    println!("Best Score: {:?}", best_score);
}
struct View {
    left: (bool, u32),
    right: (bool, u32),
    up: (bool, u32),
    down: (bool, u32),
}
fn check_visibility(
    direction: MoveDirection,
    forest: &Vec<Vec<u32>>,
    current_index: (usize, usize),
) -> (bool, u32) {
    let curr_height = forest[current_index.0][current_index.1];
    let mut viewing_distance = 1;
    match direction {
        MoveDirection::Up => {
            let mut check = current_index.0 - 1;
            loop {
                if forest[check][current_index.1] >= curr_height {
                    return (false, viewing_distance);
                }
                if check == 0 {
                    break;
                }
                viewing_distance +=1;
                check = check - 1;
            }
        }
        MoveDirection::Down => {
            let mut check = current_index.0 + 1;
            loop {
                if forest[check][current_index.1] >= curr_height {
                    return (false, viewing_distance);
                }
                check = check + 1;
                if check >= forest.len() {
                    break;
                }
                viewing_distance +=1;
            }
        }
        MoveDirection::Left => {
            let mut check = current_index.1 - 1;
            loop {
                if forest[current_index.0][check] >= curr_height {
                    return (false, viewing_distance);
                }
                if check == 0 {
                    break;
                }
                viewing_distance +=1;
                check = check - 1;
            }
        }
        MoveDirection::Right => {
            let mut check = current_index.1 + 1;
            loop {
                if forest[current_index.0][check] >= curr_height {
                    return (false, viewing_distance);
                }
                check = check + 1;
                if check >= forest.len() {
                    break;
                }
                viewing_distance +=1;
            }
        }
    }
    (true, viewing_distance)
}
enum MoveDirection {
    Up,
    Left,
    Right,
    Down,
}

fn parse_input(lines: Vec<String>) -> Vec<Vec<u32>> {
    let mut forest:Vec<Vec<u32>> = Vec::new();
    let mut i = 0;
    for line in lines {
        forest.push(Vec::<u32>::new());
        for l in line.chars() {
            forest[i].push(l.to_digit(10).unwrap());
        }
        i += 1;
    }
    forest
}
