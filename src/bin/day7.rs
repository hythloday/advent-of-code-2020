use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use regex::Regex;

fn main() {

    if let Ok(lines) = read_lines("./data/day7.txt") {
        let bag_graph: HashMap<String, Vec<String>> = lines
            .map(|line| line.unwrap())
            .flat_map(|line| {
                let b = Regex::new(r"^(.*) bags contain").unwrap();
                let c = Regex::new(r"([\d]+) ([^,]+) bags?").unwrap();
                let bag_cap = b.captures(&line).unwrap()[1].to_string();
                c.captures_iter(&line).map(|c| (c[2].to_string(), bag_cap.to_string())).collect::<Vec<_>>()
            })
            .into_iter()
            .sorted()
            .group_by(|(k,_v)| k.to_string())
            .into_iter()
            .map(|(k,kvs)| (k.to_string(), kvs.map(|(_k,v)| v.to_string()).collect::<Vec<_>>()))
            .collect::<HashMap<_,_>>();

        // p1
        let mut seen_bags = HashSet::new();
        let mut unseen_bags = HashSet::new();
        unseen_bags.insert("shiny gold".to_string());

        while !unseen_bags.is_empty() {
            let next_bag = unseen_bags.iter().next().unwrap().to_string();
            unseen_bags.remove(&next_bag.to_string());
            if !seen_bags.contains(&next_bag) {
                seen_bags.insert(next_bag.to_string());
                let containers = bag_graph.get(&next_bag);
                if containers.is_some() {
                    for bag in containers.unwrap() {
                        unseen_bags.insert(bag.to_string());
                    }
                }
            }
        }

        seen_bags.remove(&"shiny gold".to_string());

        println!("{:?}", seen_bags.len());
    }

    if let Ok(lines) = read_lines("./data/day7.txt") {
        let bag_graph: HashMap<String, Vec<(u32, String)>> = lines
            .map(|line| line.unwrap())
            .flat_map(|line| {
                let b = Regex::new(r"^(.*) bags contain").unwrap();
                let c = Regex::new(r"([\d]+) ([^,]+) bags?").unwrap();
                let bag_cap = b.captures(&line).unwrap()[1].to_string();
                c.captures_iter(&line).map(|c| (bag_cap.to_string(), (c[1].parse::<u32>().unwrap(), c[2].to_string()))).collect::<Vec<_>>()
            })
            .into_iter()
            .sorted()
            .group_by(|(k,_v)| k.to_string())
            .into_iter()
            .map(|(k,kvs)| (k.to_string(), kvs.map(|(_k,v)| v).collect::<Vec<_>>()))
            .collect::<HashMap<_,_>>();

        let contained = count_bags(&bag_graph, &"shiny gold".to_string()) - 1;

        println!("{:?}", contained);
    }
}

fn count_bags(graph: &HashMap<String, Vec<(u32, String)>>, bag: &String) -> u32 {
    1 + match graph.get(bag) {
        Some(bags) => {
            bags.into_iter().map(|(count,bag_name)| count * count_bags(graph, bag_name)).sum()
        },
        None => 0
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
