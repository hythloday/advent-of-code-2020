use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;


fn main() {

    if let Ok(lines) = read_lines("./data/day9.txt") { 
        let list = lines.map(|line| {
            line.unwrap().parse::<u64>().unwrap()
        }).collect::<Vec<_>>();

        let preamble = 25;
        let elems = list.clone().into_iter().skip(preamble);
        let windows = list.windows(preamble).map(|xs| xs.iter().collect::<HashSet<_>>());

        let invalid = windows.zip(elems).filter(|(prev, next)| {
            let matches = prev.clone().into_iter().filter(move |x| prev.contains(&(next-*x)));
            matches.count() == 0
        }).nth(0).unwrap().1;

        // p1
        println!("{:?}", invalid);

        'outer: for window_size in 2..list.len() {
            for window in list.windows(window_size) {
                if window.iter().sum::<u64>() == invalid {
                    // p1
                    println!("{:?}", window.iter().min().unwrap() + window.iter().max().unwrap());
                    break 'outer; 
                }
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
