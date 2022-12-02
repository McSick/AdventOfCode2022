#![allow(dead_code)]
#![allow(unused_variables)]
use std::cmp::Ordering;
#[derive(Eq, Debug)]
pub struct Elf {
    pocket: Vec<u32>,
    calories: u32,
    id: usize //Will maybe use later?
}

impl Elf {
    pub fn new(id: usize) -> Elf {
        Elf {
            pocket: Vec::new(),
            calories: 0,
            id:id
        }
    }
    pub fn add_calorie(&mut self, cal: u32) {
        self.pocket.push(cal);
        self.calories += cal;
    }
    pub fn calorie_total(&self) -> u32 {
        return self.calories;
    }
    //for debug purposes
    fn print(&self) {
        println!("Elf id: {}, Calorie Total: {}", self.id, self.calorie_total());
    }
}


impl Ord for Elf {
    fn cmp(&self, other: &Self) -> Ordering {
        self.calorie_total().cmp(&other.calorie_total())
    }
}

impl PartialOrd for Elf {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Elf {
    fn eq(&self, other: &Self) -> bool {
        self.calorie_total() == other.calorie_total()
    }
}