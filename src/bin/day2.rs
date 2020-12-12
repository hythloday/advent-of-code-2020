use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::iter::FromIterator;

use multiset::HashMultiSet;

fn main() {

    // p1
    if let Ok(lines) = read_lines("./data/day2.txt") {
        let passwords = lines.filter(|line| {
            let rule = line.as_ref().unwrap().split(" ").collect::<Vec<&str>>();
            let multiset: HashMultiSet<char> = FromIterator::from_iter(rule[2].chars());
            let needle = rule[1].chars().nth(0).unwrap();
            let minmax = rule[0].split("-").map(|c| c.parse::<usize>().unwrap()).collect::<Vec<_>>();
            let count = multiset.count_of(&needle);
            count <= minmax[1] && count >= minmax[0]
        }).count();
        println!("{}", passwords);
    }

    // p2
    if let Ok(lines) = read_lines("./data/day2.txt") {
        let passwords2 = lines.filter(|line| {
            let rule = line.as_ref().unwrap().split(" ").collect::<Vec<&str>>();
            let needle = rule[1].chars().nth(0).unwrap();
            let positions = rule[0].split("-").map(|c| c.parse::<usize>().unwrap()).collect::<Vec<_>>();
            (rule[2].chars().nth(positions[0]-1).unwrap() == needle) ^ (rule[2].chars().nth(positions[1]-1).unwrap() == needle)
        }).count();
        println!("{}", passwords2)
    }

}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
