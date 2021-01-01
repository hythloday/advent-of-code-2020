use std::{collections::HashMap, collections::HashSet, fs::File};
use std::{
    io::{self, BufRead},
    ops::RangeInclusive,
};

fn parse_range(r: &str) -> RangeInclusive<u32> {
    let range = r
        .split("-")
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<_>>();
    range[0]..=range[1]
}

fn main() {
    if let Ok(mut lines) = read_day_input() {
        let fields: Vec<(String, Vec<RangeInclusive<u32>>)> = lines
            .by_ref()
            .map(|l| l.unwrap())
            .take_while(|l| l != &"")
            .map(|f| f.split(": ").map(|s| s.to_string()).collect::<Vec<_>>())
            .map(|f| {
                (
                    f[0].clone(),
                    f[1].split(" or ").map(parse_range).collect::<Vec<_>>(),
                )
            })
            .collect::<Vec<_>>();

        let ticket: Vec<_> = lines
            .by_ref()
            .take_while(move |l| l.as_deref().unwrap() != "")
            .skip(1)
            .flat_map(|t| {
                t.unwrap()
                    .split(",")
                    .map(|s| s.parse::<u32>().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect();

        let nearby_tickets: Vec<_> = lines
            .skip(1)
            .map(|t| t.unwrap())
            .map(|t| {
                t.split(",")
                    .map(|s| s.parse::<u32>().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect();

        let valid_values = fields
            .iter()
            .flat_map(|x| x.1.iter().flat_map(move |y| y.clone()))
            .collect::<HashSet<u32>>();

        // p1
        println!(
            "{:?}",
            nearby_tickets
                .iter()
                .flatten()
                .filter(|n| !valid_values.contains(n))
                .sum::<u32>()
        );

        // p2
        let valid_tickets = nearby_tickets
            .into_iter()
            .filter(|t| t.iter().all(|val| valid_values.contains(val)));

        let ticket_columns = transpose(valid_tickets.collect());

        let mapping = ticket_columns.iter().enumerate().map(|(_i, col)| {
            fields.iter().filter(move |(_name, ranges)| {
                col.iter().all(|v| ranges.iter().any(|r| r.contains(v)))
            })
        });

        let mut possibilities: HashMap<usize, HashSet<&String>> = mapping
            .map(|x| x.map(|(name, _range)| name).collect::<HashSet<_>>())
            .enumerate()
            .collect::<HashMap<_, _>>();

        let mut solved: HashMap<usize, String> = HashMap::new();
        loop {
            if possibilities.values().all(|vs| vs.len() == 0) {
                break;
            }
            let singleton = possibilities.iter().filter(|(_, v)| v.len() == 1);
            let to_remove = *singleton.clone().map(|x| x.1).flatten().nth(0).unwrap();
            let col = *singleton.map(|x| x.0).nth(0).unwrap();
            for (_, vs) in &mut possibilities {
                vs.remove(to_remove);
            }
            solved.insert(col, to_remove.to_string());
        }

        println!(
            "{:?}",
            solved
                .iter()
                .filter(|(_, v)| v.contains("departure"))
                .map(|(k, _)| ticket[*k] as u64)
                .product::<u64>()
        );
    }
}

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    assert!(!v.is_empty());
    (0..v[0].len())
        .map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<T>>())
        .collect()
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
