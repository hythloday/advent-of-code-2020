use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use std::collections::HashSet;

use itertools::Itertools;

fn main() {

    if let Ok(lines) = read_lines("./data/day5.txt") {

        let seats = lines.map(|line| {
            let bin_idx = line.unwrap().replace("F", "0").replace("B", "1").replace("L", "0").replace("R", "1");
            isize::from_str_radix(&bin_idx, 2).unwrap()
        }).collect::<HashSet<_>>();


        // p2
        let occupied = (0..1024)
            .group_by(|s| seats.contains(s))
            .into_iter()
            .filter(|(b, _s)| *b)
            .map(|(_b, s)| s.collect::<Vec<_>>())
            .map(|v| (*v.first().unwrap(), *v.last().unwrap()))
            .collect::<Vec<_>>();

        // p1
        println!("{}", seats.into_iter().max().unwrap());

        // p2
        println!("{:?}", occupied)
    }

}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
