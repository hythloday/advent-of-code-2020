use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;

use itertools::Itertools;
use multiset::HashMultiSet;

fn main() {

    if let Ok(lines) = read_lines("./data/day6.txt") {
        let grouped_lines = lines
            .map(|line| line.unwrap())
            .into_iter()
            .group_by(|line| line!= "");

        let valid = grouped_lines
            .into_iter()
            .filter(|(b, _lines)| *b)
            .map(|(_b, mut lines)| lines.join("").chars().collect::<HashSet<_>>().len())
            .collect::<Vec<_>>();

        // p1
        println!("{:?}", valid.into_iter().sum::<usize>());
    }

    if let Ok(lines) = read_lines("./data/day6.txt") {
        let grouped_lines = lines
            .map(|line| line.unwrap())
            .into_iter()
            .group_by(|line| line!= "");

        let valid = grouped_lines
            .into_iter()
            .filter(|(b, _lines)| *b)
            .map(|(_b, lines)| {
                let vlines = lines.collect::<Vec<_>>();
                let line_count = vlines.len();
                let counts = vlines.into_iter().join("").chars().collect::<HashMultiSet<_>>();
                counts.distinct_elements().filter(|e| counts.count_of(e) == line_count).count()
            })
            .collect::<Vec<_>>();

        // p2
        println!("{:?}", valid.into_iter().sum::<usize>());
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
