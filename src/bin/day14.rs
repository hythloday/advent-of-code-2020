use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

use itertools::Itertools;

#[derive(Clone, Debug)]
enum Instruction {
    Mask(u64, u64),
    Write(usize, u64),
    MaskV2(String),
}

fn main() {
    if let Ok(lines) = read_day_input() {
        let zero: (HashMap<usize, u64>, (u64, u64)) = (HashMap::new(), (0u64, 1u64));
        let (memory, _) = lines
            .map(|line| {
                let l = line.unwrap();
                let parts = l.split(" = ").collect::<Vec<_>>();
                if parts[0].starts_with("mask") {
                    let andmask = u64::from_str_radix(&parts[1].replace("X", &"0"), 2).unwrap();
                    let ormask = u64::from_str_radix(&parts[1].replace("X", &"1"), 2).unwrap();
                    Instruction::Mask(ormask, andmask)
                } else {
                    let addr = parts[0][4..parts[0].len() - 1].parse::<usize>().unwrap();
                    let val = parts[1].parse::<u64>().unwrap();
                    Instruction::Write(addr, val)
                }
            })
            .fold(zero, |(mut mem, (mask_and, mask_or)), instr| match instr {
                Instruction::Mask(and, or) => (mem, (and, or)),
                Instruction::Write(loc, val) => {
                    mem.insert(loc, val & mask_and | mask_or);
                    (mem, (mask_and, mask_or))
                }
                _ => panic!(),
            });

        // p1
        println!("{:?}", memory.values().sum::<u64>());
    }

    if let Ok(lines) = read_day_input() {
        let zero: (HashMap<usize, u64>, String) = (HashMap::new(), "".to_string());
        let program: Vec<_> = lines
            .map(|line| {
                let l = line.unwrap();
                let parts = l.split(" = ").collect::<Vec<_>>();
                if parts[0].starts_with("mask") {
                    Instruction::MaskV2(parts[1].to_string())
                } else {
                    let addr = parts[0][4..parts[0].len() - 1].parse::<usize>().unwrap();
                    let val = parts[1].parse::<u64>().unwrap();
                    Instruction::Write(addr, val)
                }
            })
            .collect();
        println!("{:?}", program.clone());
        let (memory, _) = program
            .into_iter()
            .fold(zero, |(mut mem, mask), instr| match instr {
                Instruction::MaskV2(new_mask) => (mem, new_mask),
                Instruction::Write(loc, val) => {
                    println!("{:?}", mask);
                    let loc_string = format!("{:036b}", loc);
                    println!("{:?}", loc_string);
                    let zip = mask.chars().rev().zip(loc_string.chars().rev());
                    println!("zip: {:?}", zip.clone().collect::<Vec<_>>());
                    let combs = zip.map(|(m, l)| match m {
                        '0' => vec![l],
                        '1' => vec![m],
                        'X' => vec!['0', '1'],
                        _ => panic!(),
                    });
                    println!("combs: {:?}", combs.clone().collect::<Vec<_>>());
                    combs
                        .multi_cartesian_product()
                        .map(|cs| cs.into_iter().rev().collect::<String>())
                        .map(|n| usize::from_str_radix(&n, 2).unwrap())
                        .sorted()
                        .for_each(|masked_addr| {
                            println!("writing {} to {}", val, masked_addr);
                            mem.insert(masked_addr, val);
                            ()
                        });
                    (mem, mask)
                }
                _ => panic!(),
            });

        // p2
        println!("{:?}", memory.values().sum::<u64>());
    }
}

fn read_day_input() -> io::Result<io::Lines<io::BufReader<File>>> {
    let mut path = std::env::current_exe()?.clone();
    let file_name = std::env::current_exe()?;
    &path.pop();
    &path.pop();
    &path.pop();
    &path.push("data");
    &path.push(file_name.file_name().unwrap());
    &path.set_extension("txt");

    let file = File::open(path)?;
    Ok(io::BufReader::new(file).lines())
}
