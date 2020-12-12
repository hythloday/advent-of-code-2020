use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;

fn main() {
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./data/day1.txt") {
        // Consumes the iterator, returns an (Optional) String
        let entries: HashSet<i32> = lines.map(|line|
            line.unwrap().parse::<i32>().unwrap()
        ).collect::<>();

        // p1
        entries.iter().for_each(|e|
            if entries.contains(&(2020-e)) {
                println!("{}", e*(2020-e));
            }
        );

        // p2
        entries.iter().for_each(|e1|
            entries.iter().filter(|e2| e2 < &&(2020 - e1)).for_each(|e2|
                if entries.contains(&(2020-e1-e2)) {
                    println!("{}", e1*e2*(2020-e1-e2));
                } 
            )
        )
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
