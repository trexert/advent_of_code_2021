use itertools::Itertools;
use linked_hash_map::LinkedHashMap;
use std::collections::{BTreeMap, HashMap};

fn main() {
    let map = CaveMap::from_input(include_str!("input.txt"));

    println!("part1 result: {}", part1(&map));
    println!("part2 result: {}", part2(&map));
}

struct CaveMap {
    connections: HashMap<String, Vec<String>>,
}

fn part1(map: &CaveMap) -> usize {
    map.count_paths(
        |cave, path| cave.chars().all(|c| c.is_uppercase()) || !path.contains_key(cave),
        |cave| cave == "end",
    )
}

fn part2(map: &CaveMap) -> usize {
    map.count_paths(
        |cave, path| {
            cave != "start"
                && (cave.chars().all(|c| c.is_uppercase())
                    || path.values().all(|x| *x < 2)
                    || !path.contains_key(cave))
        },
        |cave| cave == "end",
    )
}

impl CaveMap {
    fn from_input(input: &str) -> Self {
        let mut connections: HashMap<String, Vec<String>> = HashMap::new();

        for (a, b) in input
            .lines()
            .map(|l| l.split("-").collect_tuple::<(&str, &str)>().unwrap())
        {
            connections
                .entry(a.to_string())
                .or_default()
                .push(b.to_string());
            connections
                .entry(b.to_string())
                .or_default()
                .push(a.to_string());
        }

        CaveMap { connections }
    }

    fn count_paths<Pv, Pf>(&self, valid_move: Pv, final_move: Pf) -> usize
    where
        Pv: Fn(&String, &BTreeMap<String, u8>) -> bool,
        Pf: Fn(&String) -> bool,
    {
        let mut path_count = 0;
        let mut paths = LinkedHashMap::new();
        paths.insert(
            (
                BTreeMap::from([("start".to_string(), 1u8)]),
                "start".to_string(),
            ),
            1usize,
        );

        while !paths.is_empty() {
            let (current_path, count) = paths.pop_front().unwrap();
            for cave in &self.connections[&current_path.1] {
                if final_move(cave) {
                    path_count += count;
                } else if valid_move(cave, &current_path.0) {
                    let mut continued_path = current_path.clone();
                    if cave.chars().all(|c| c.is_lowercase()) {
                        *continued_path.0.entry(cave.to_string()).or_default() += 1;
                    }
                    continued_path.1 = cave.clone();
                    if paths.contains_key(&continued_path) {
                        paths[&continued_path] += count;
                    } else {
                        paths.insert(continued_path, count);
                    }
                }
            }
        }

        path_count
    }
}
