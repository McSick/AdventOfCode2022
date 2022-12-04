use crate::helpers::file::{read_lines};
pub fn run(filename: String) {
    let lines = match read_lines(filename) {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
    let lines: Vec<String> = lines.into_iter().map(|s| s.ok().unwrap()).collect();
    
    let mut part1_in_range_pairs = 0;
    let mut part2_overlap_pairs = 0;
    
    for line in lines {
        let lr: Vec<String> = line.split(",").map(|s| s.to_string()).collect();

        let left_range: Range = Range::from_string(lr[0].to_string());
        let right_range: Range = Range::from_string(lr[1].to_string());
        if left_range.in_bounds(&right_range) || right_range.in_bounds(&left_range) {
            part1_in_range_pairs += 1;
        }

        if left_range.overlap(&right_range) || right_range.overlap(&left_range) {
            part2_overlap_pairs += 1;
        }
    }
    println!("Number of fully contained pairs: {}", part1_in_range_pairs);
    println!("Number of overlapping pairs: {}", part2_overlap_pairs);
}

struct Range {
    min: u32,
    max: u32
}
impl Range {
    fn from_string(s: String) -> Range {
        let vec: Vec<u32> = s.split("-").map(|s| s.parse().unwrap()).collect();
        Range {
            min: vec[0],
            max: vec[1]
        }
    }
    fn in_bounds(&self, r2: &Range) -> bool {
        if self.min >= r2.min && self.max <= r2.max {
            true
        } else {
            false
        }
    }
    fn overlap(&self, r2: &Range) -> bool {
        if self.min <= r2.min && self.max >= r2.min {
            true
        } else {
            false
        }

    }
}