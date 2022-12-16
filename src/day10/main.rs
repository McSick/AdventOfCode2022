use crate::helpers::file::read_lines;

pub fn run(filename: String) {
    let lines = match read_lines(filename) {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
    let mut commands: Vec<Instruction> = lines
        .into_iter()
        .map(|s| Instruction::new(s.ok().unwrap()))
        .collect();
    commands.reverse();
    let mut cpu = CPU::new();
    let mut sig_str = 0;
    let mut horiziontaonl_pos = 0;
    while commands.len() > 0 {
        if ((cpu.register - 1) <= horiziontaonl_pos) && horiziontaonl_pos <= (cpu.register + 1) {
            print!("#");
        } else {
            print!(" ");
        }
        match cpu.active_instruction {
            None => {
                let next_command = commands.pop();
                if let Some(command) = next_command {
                    cpu.add_instruction(command);
                }
            }, 
            _ => {}
        }
        cpu.cycle();
        match cpu.cycles {
            20 | 60 | 100 | 140 | 180 | 220 => {
                sig_str += cpu.cycles * cpu.register;
            },
            _ => {}
        }
        horiziontaonl_pos += 1;
        if horiziontaonl_pos > 39 {
            println!("");
            horiziontaonl_pos = 0;
        }

    }
    println!("");
    println!("Part1: {}", sig_str);
}
struct CPU {
    register: i32, 
    cycles: i32,
    work_for: u8,
    active_instruction: Option<Instruction>

}
impl  CPU {
    fn new() -> CPU {
        CPU { register: 1 ,cycles: 1, work_for: 0, active_instruction: None}
    }
    fn add_instruction(&mut self, instruction: Instruction) {
        self.work_for += instruction.op.num_cycles();
        self.active_instruction = Some(instruction);

    }
    fn apply_instruction(&mut self) {
        match &self.active_instruction {
            Some(instruction) => {
                if let Some(data) = instruction.data {
                    self.register += data;
                }
            },
            None => {}
        }
        self.active_instruction = None;
    }
    fn cycle(&mut self) {
        match self.work_for {
            0 => {},
            1 => { 
                self.apply_instruction();
                self.work_for = 0;
            },
            _ => { self.work_for -= 1}
        }
        self.cycles += 1;
    }
}
#[derive(Debug)]
struct Instruction {
    op: Operation,
    data: Option<i32>
}
impl Instruction {
    fn new(s: String) -> Instruction {
        let split: Vec<&str> = s.split(" ").collect::<Vec<&str>>();
        match split[0] {
            "noop" => Instruction { op : Operation::NOOP, data : None },
            _ => Instruction { op : Operation::ADDX, data: split[1].parse::<i32>().ok() },
        }
    }
}
#[derive(Debug)]
enum Operation {
    NOOP,
    ADDX
}
impl Operation {
    fn num_cycles(&self) -> u8 {
        match *self {
            Operation::NOOP => 1,
            Operation::ADDX => 2
        }
    }
 }
