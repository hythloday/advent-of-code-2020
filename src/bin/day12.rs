use std::fs::File;
use std::io::{self, BufRead};

fn rotate(heading: (i32, i32), direction: char, amount: i32) -> (i32, i32) {
    match direction {
        'L' => (0..amount).fold(heading, |(hx, hy),_|(-hy,hx)),
        'R' => (0..amount).fold(heading, |(hx, hy),_|(hy,-hx)),
        _ => panic!()
    }
}

fn main() {

    if let Ok(lines) = read_day_input() {
        let dir = (1, 0);
        let pos = (0, 0);
        let (_end_dir, end_pos) = lines
            .map(|line| {
                let l = line.unwrap();
                (l.chars().nth(0).unwrap(), l[1..].parse::<i32>().unwrap())
            })
            .fold((dir, pos), |((dx, dy), (px, py)), (action, value)| {
                match action {
                    'N' => ((dx, dy), (px, py+value)),
                    'S' => ((dx, dy), (px, py-value)),
                    'E' => ((dx, dy), (px+value, py)),
                    'W' => ((dx, dy), (px-value, py)),
                    'L' => (rotate((dx, dy), 'L', value / 90), (px, py)),
                    'R' => (rotate((dx, dy), 'R', value / 90), (px, py)),
                    'F' => ((dx, dy), (px+dx*value, py+dy*value)),
                    _ => panic!()
                }
            });

        // p1
        println!("{:?}", end_pos.0.abs() + end_pos.1.abs());
    }

    if let Ok(lines) = read_day_input() {
        let wpt = (10,1);
        let pos = (0, 0);
        let (_end_wpt, end_pos) = lines
            .map(|line| {
                let l = line.unwrap();
                (l.chars().nth(0).unwrap(), l[1..].parse::<i32>().unwrap())
            })
            .fold((wpt, pos), |((wx, wy), (px, py)), (action, value)| {
                match action {
                    'N' => ((wx, wy+value), (px, py)),
                    'S' => ((wx, wy-value), (px, py)),
                    'E' => ((wx+value, wy), (px, py)),
                    'W' => ((wx-value, wy), (px, py)),
                    'L' => (rotate((wx, wy), 'L', value / 90), (px, py)),
                    'R' => (rotate((wx, wy), 'R', value / 90), (px, py)),
                    'F' => ((wx, wy), (px+wx*value, py+wy*value)),
                    _ => panic!()
                }
            });

        // p2
        println!("{:?}", end_pos.0.abs() + end_pos.1.abs());
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
