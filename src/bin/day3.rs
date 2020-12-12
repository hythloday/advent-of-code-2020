use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;

fn main() {

    // p1
    if let Ok(lines) = read_lines("./data/day3.txt") {
        let mut height = 0;
        let mut width = 0;
        let trees = lines.enumerate().flat_map(|(line_no, line)| {
            let real_line = line.unwrap();
            height = line_no;
            width = real_line.len();
            real_line.match_indices("#").map(move |(pos,_chr)| (pos, line_no)).collect::<Vec<_>>()
        }).collect::<HashSet<_>>();

        let collisions = (0..=height).filter(|y| {
            let x = y * 3;
            trees.contains(&(x % width, *y))
        }).count();

        println!("{}", collisions);

        // p2
        let courses: usize = vec!((1, 1), (3, 1), (5, 1), (7, 1), (1, 2)).into_iter().map(|(dx, dy)| {
            (0..=height).step_by(dy).filter(|y| {
                let x = y/dy * dx;
                trees.contains(&(x % width, *y))
            }).count()
        }).product();

        println!("{:?}", courses);
    }




}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
