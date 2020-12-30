use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::cmp;


#[derive(Clone, Copy, Debug, PartialEq)]
enum Pos {
    Floor,
    Empty,
    Occupied
}

fn main() {
    if let Ok(lines) = read_lines("./data/day11.txt") {
        let seats = lines
            .enumerate()
            .flat_map(|(line_no, line)| {
                let l = line.unwrap();
                l.chars()
                    .enumerate()
                    .map(move |(char_no, chr)| match chr {
                        'L' => ((line_no, char_no), Pos::Empty),
                        '.' => ((line_no, char_no), Pos::Floor),
                        '#' => ((line_no, char_no), Pos::Occupied),
                        _ => panic!(),
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<HashMap<_, _>>();

        let h = seats.iter().map(|(k, _v)| k.0).max().unwrap();
        let w = seats.iter().map(|(k, _v)| k.1).max().unwrap();

        fn n_neighbours(x: usize, y: usize, seats: &HashMap<(usize, usize), Pos>) -> usize {
            (-1..=1)
                .flat_map(|dx| {
                    (-1..=1).map(move |dy| {
                        seats
                            .get(&((y as i32 + dy) as usize, (x as i32 + dx) as usize))
                            .unwrap_or(&Pos::Floor)
                    })
                })
                .filter(|x| *x == &Pos::Occupied)
                .count()
                - (if seats.get(&(y, x)).unwrap_or(&Pos::Floor) == &Pos::Occupied { 1 } else { 0 })
        }

        let mut stop = false;
        let mut last = seats.clone();
        let mut next: HashMap<(usize, usize), Pos> = HashMap::new();
        while !stop {
            
            // for y in 0..=h {
            //     for x in 0..=w {
            //         print!("{}", match *last.get(&(y, x)).unwrap() {
            //             Pos::Floor => ".",
            //             Pos::Occupied => "#",
            //             Pos::Empty => "L"
            //         });
            //     }
            //     println!();
            // }    
            // println!();

            // for y in 0..=h {
            //     for x in 0..=w {
            //         print!("{}", n_neighbours(y, x, &last));
            //     }
            //     println!();
            // }    
            // println!();

            for y in 0..=h {
                for x in 0..=w {
                    let pos = *last.get(&(y, x)).unwrap();
                    let n = n_neighbours(x, y, &last);
                    if pos == Pos::Empty && n == 0 {
                        next.insert((y, x), Pos::Occupied);
                    } else if pos == Pos::Occupied && n >= 4 {
                        next.insert((y, x), Pos::Empty);
                    } else {
                        next.insert((y, x), pos);
                    }
                }
            }
            if last == next {
                stop = true;
            } else {
                last = next.clone();
            }
        }


        // p1
        println!("{}", next.iter().filter(|(_k, v)| **v == Pos::Occupied).count());

        fn n_visible_neighbours(x: usize, y: usize, h: usize, w: usize, seats: &HashMap<(usize, usize), Pos>) -> usize {

            (-1..=1)
                .flat_map(|dx| {
                    (-1..=1).map(move |dy| {
                        (1..=cmp::max(h, w) as i32)
                            .map(|d| (d, (dx*d, dy*d)))
                            .map(|(d, (dx,dy))| (d, ((y as i32 + dy) as usize, (x as i32 + dx) as usize)))
                            .filter(|(_d, p)| seats.get(&p).is_some())
                            .filter(|(_d, p)| seats.get(&p).unwrap() != &Pos::Floor)
                            .min_by(|(d1, _p1), (d2, _p2)| d1.cmp(d2))
                            .and_then(|(_d, p)| seats.get(&p))
                            .unwrap_or(&&Pos::Floor)

                    })
                })
                .filter(|x| *x == &Pos::Occupied)
                .count()
                - (if seats.get(&(y, x)).unwrap_or(&Pos::Floor) == &Pos::Occupied { 1 } else { 0 })
        }

        let mut stop = false;
        let mut last = seats.clone();
        let mut next: HashMap<(usize, usize), Pos> = HashMap::new();
        let mut loop_i = 0;
        while !stop {
            println!("{}", loop_i);
            // for y in 0..=h {
            //     for x in 0..=w {
            //         print!("{}", match *last.get(&(y, x)).unwrap() {
            //             Pos::Floor => ".",
            //             Pos::Occupied => "#",
            //             Pos::Empty => "L"
            //         });
            //     }
            //     println!();
            // }    
            // println!();

            // for y in 0..=h {
            //     for x in 0..=w {
            //         print!("{}", n_visible_neighbours(y, x, h, w, &last));
            //     }
            //     println!();
            // }    
            // println!();

            for y in 0..=h {
                for x in 0..=w {
                    let pos = *last.get(&(y, x)).unwrap();
                    let n = n_visible_neighbours(x, y, h, w, &last);
                    if pos == Pos::Empty && n == 0 {
                        next.insert((y, x), Pos::Occupied);
                    } else if pos == Pos::Occupied && n >= 5 {
                        next.insert((y, x), Pos::Empty);
                    } else {
                        next.insert((y, x), pos);
                    }
                }
            }
            if last == next {
                stop = true;
            } else {
                last = next.clone();
            }
            loop_i += 1;
        }

        // p2
        println!("{}", next.iter().filter(|(_k, v)| **v == Pos::Occupied).count());
    }
}


// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
