use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;

use itertools::Itertools;
use multiset::HashMultiSet;

use strum_macros::EnumString;
use std::str::FromStr;

#[derive(EnumString)]
#[strum(serialize_all = "snake_case")]
#[derive(Clone)]
enum Op {
    Nop,
    Jmp,
    Acc
}

#[derive(Clone)]
struct Instruction {
    op: Op,
    argument: i32
}

impl Instruction {
    pub fn new(op: &str, argument: i32) -> Instruction {
        Instruction { op: Op::from_str(op).unwrap(), argument: argument }
    }

    pub fn flip(self: &mut Self) {
        self.op = match self.op {
            Op::Nop => Op::Jmp,
            Op::Acc => Op::Acc,
            Op::Jmp => Op::Nop
        }
    }
}

fn main() {

    if let Ok(lines) = read_lines("./data/day8.txt") { 
        let mut program = lines.map(|line| {
            let line_str = line.unwrap();
            let parts = line_str.split(" ").collect::<Vec<_>>();
            Instruction::new(parts[0], parts[1].parse::<i32>().unwrap())
        }).collect::<Vec<_>>();

        let mut visited: HashSet<usize> = HashSet::new();
        let mut ip: usize = 0;

        let mut accum: i32 = 0;

        while visited.get(&ip).is_none() {
            visited.insert(ip);
            let instr = &program[ip];

            // op side-effect
            match &instr.op {
                Op::Acc => accum += &instr.argument,
                Op::Jmp => ip = (ip as i32 + &instr.argument) as usize,
                _ => ()
            }

            // next op
            match &instr.op {
                Op::Acc | Op::Nop => ip += 1,
                _ => ()
            }
        }

        // p1
        println!("{:?}", accum);

        // p2
        for line_no in 0..program.len() {
            let mut modified_program = program.clone();
            modified_program[line_no].flip();

            let mut visited: HashSet<usize> = HashSet::new();
            let mut ip: usize = 0;
    
            let mut accum: i32 = 0;
    
            while visited.get(&ip).is_none() && ip < modified_program.len() {
                visited.insert(ip);
                let instr = &modified_program[ip];
    
                // op side-effect
                match &instr.op {
                    Op::Acc => accum += &instr.argument,
                    Op::Jmp => ip = (ip as i32 + &instr.argument) as usize,
                    _ => ()
                }
    
                // next op
                match &instr.op {
                    Op::Acc | Op::Nop => ip += 1,
                    _ => ()
                }
            }

            if ip >= modified_program.len() {
                println!("{}", accum);
            }
        }
    }

}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
