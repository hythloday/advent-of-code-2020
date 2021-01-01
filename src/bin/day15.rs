use std::{collections::HashMap, fs::File};
use std::io::{self, BufRead};

#[derive(Clone)]
struct NumberGame {
    init: Vec<u32>,
    last_number: u32,
    last_spoken: HashMap<u32, u32>,
    tick: u32,
}

impl Iterator for NumberGame {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        self.tick += 1;

        let next_number = if !self.init.is_empty() {
            self.init.pop().unwrap()
        } else {
            // print!("seeing if {} has been spoken before...", self.last_number);
            if self.last_spoken.contains_key(&self.last_number) {
                let turn_said = self.last_spoken.get(&self.last_number).unwrap();
                // println!("yes, on turn {} - we're now on turn {} so saying {}", turn_said, self.tick, self.tick - turn_said);
                self.tick - turn_said
            } else {
                // println!("no, saying 0");
                0
            }
        };

        self.last_spoken.insert(self.last_number, self.tick);
        self.last_number = next_number;
        Some(self.last_number)
    }
}

fn main() {
    if let Ok(lines) = read_day_input() {
        let mut init = lines.map(|line| {
            let l = line.unwrap();
            l.split(",").map(|x| x.parse::<u32>().unwrap()).collect::<Vec<_>>()
        }).nth(0).unwrap();

        init.reverse();
        let game = NumberGame { init: init, last_number: 0, last_spoken: HashMap::new(), tick: 0 };

        // p1
        println!("{:?}", game.clone().nth(2020-1).unwrap());
        // p2
        println!("{:?}", game.clone().nth(30000000-1).unwrap());
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
