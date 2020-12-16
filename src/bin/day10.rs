use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use multiset::HashMultiSet;

use itertools::Itertools;

fn main() {

    if let Ok(lines) = read_lines("./data/day10.txt") { 
        let mut list = lines.map(|line| {
            line.unwrap().parse::<u64>().unwrap()
        }).sorted().collect::<Vec<_>>();

        list.push(list[list.len()-1]+3);
        
        list.push(0);
        list.rotate_right(1);

        let differences = list.windows(2).map(|xs| xs[1]-xs[0]).collect::<HashMultiSet<_>>(); 
        // p1
        println!("{:?}", differences.count_of(&1) * differences.count_of(&3));

   }

}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
