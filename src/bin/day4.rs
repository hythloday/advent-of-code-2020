use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

use itertools::Itertools;
use regex::Regex;

fn main() {

    if let Ok(lines) = read_lines("./data/day4.txt") {
        let grouped_lines = lines
            .map(|line| line.unwrap())
            .into_iter()
            .group_by(|line| line!= "");

        let valid = grouped_lines
            .into_iter()
            .filter(|(b, _lines)| *b)
            .map(|(_b, mut lines)| lines.join(" "))
            .map(|fields| fields
                .split(" ")
                .map(|field| {
                    let mut v = field.split(":");
                    (v.next().unwrap().to_string(), v.next().unwrap().to_string())
                }).collect::<HashMap<_, _>>())
            .map(|mut m| { m.remove("cid"); m })
            .filter(|m| m.len() >= 7)
            .collect::<Vec<_>>();

        // p1
        println!("{}", valid.len());

        let very_valid = valid.into_iter()
            .filter(|m| m.get("byr").unwrap().parse::<usize>().unwrap() >= 1920)
            .filter(|m| m.get("byr").unwrap().parse::<usize>().unwrap() <= 2002)
            .filter(|m| m.get("iyr").unwrap().parse::<usize>().unwrap() >= 2010)
            .filter(|m| m.get("iyr").unwrap().parse::<usize>().unwrap() <= 2020)
            .filter(|m| m.get("eyr").unwrap().parse::<usize>().unwrap() >= 2020)
            .filter(|m| m.get("eyr").unwrap().parse::<usize>().unwrap() <= 2030)
            .filter(|m| { 
                let hgt = m.get("hgt").unwrap();
                if Regex::new(r"([\d]{2,3})(cm|in)").unwrap().is_match(hgt) {
                    let captures = &Regex::new(r"([\d]{2,3})(cm|in)").unwrap().captures(hgt).unwrap();
                    if &captures[2] == "cm" {
                        (150..=193).contains(&captures[1].parse::<usize>().unwrap())
                    } else if &captures[2] == "in" {
                        (59..=76).contains(&captures[1].parse::<usize>().unwrap())
                    } else {
                        None.unwrap()
                    }
                }
                else { false }
            })
            .filter(|m| Regex::new(r"#[a-f0-9]{6}").unwrap().is_match(m.get("hcl").unwrap()))
            .filter(|m| Regex::new(r"amb|blu|brn|gry|grn|hzl|oth").unwrap().is_match(m.get("ecl").unwrap()))
            .filter(|m| Regex::new(r"^[\d]{9}$").unwrap().is_match(m.get("pid").unwrap()))
            ;
           
        // p2
        println!("{:?}", very_valid.count());

    }

}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
