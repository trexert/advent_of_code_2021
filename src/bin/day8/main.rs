use itertools::Itertools;
use std::collections::HashSet;

fn main() {
    let displays: Vec<_> = include_str!("input.txt")
        .lines()
        .map(|line| Display::from_line(line))
        .collect();

    println!("part1 result: {}", part1(&displays));
    println!("part2 result: {}", part2(&displays));
}

fn part1(displays: &Vec<Display>) -> usize {
    displays
        .iter()
        .map(|d| {
            d.output
                .iter()
                .filter(|digit| [2, 3, 4, 7].contains(&digit.len()))
                .count()
        })
        .sum()
}

fn part2(displays: &Vec<Display>) -> usize {
    displays.iter().map(|d| d.compute_output()).sum()
}

struct Display {
    patterns: Vec<HashSet<char>>,
    output: Vec<HashSet<char>>,
}

impl Display {
    fn from_line(line: &str) -> Display {
        let mut split_line = line.split(" | ");
        Display {
            patterns: split_line
                .next()
                .unwrap()
                .split_whitespace()
                .sorted_by_key(|s| s.len())
                .map(|s| s.chars().collect())
                .collect(),
            output: split_line
                .next()
                .unwrap()
                .split_whitespace()
                .map(|s| s.chars().collect())
                .collect(),
        }
    }

    fn compute_output(&self) -> usize {
        let one = &self.patterns[0];
        let seven = &self.patterns[1];
        let four = &self.patterns[2];
        let eight = &self.patterns[9];
        let zerosixnine = &self.patterns[6..=8];
        let six = zerosixnine
            .iter()
            .filter(|&d| one.difference(d).count() > 0)
            .next()
            .unwrap();
        let zero = zerosixnine
            .iter()
            .filter(|&d| {
                four.difference(&(d.union(one).map(|&c| c).collect()))
                    .count()
                    > 0
            })
            .next()
            .unwrap();
        let nine = zerosixnine
            .iter()
            .filter(|&d| ![zero, six].contains(&d))
            .next()
            .unwrap();
        let twothreefive = &self.patterns[3..=5];
        let five = twothreefive
            .iter()
            .filter(|&d| d.difference(six).count() == 0)
            .next()
            .unwrap();
        let two = twothreefive
            .iter()
            .filter(|&d| d.difference(nine).count() > 0)
            .next()
            .unwrap();
        let three = twothreefive
            .iter()
            .filter(|&d| ![two, five].contains(&d))
            .next()
            .unwrap();

        self.output
            .iter()
            .map(|d| match d {
                x if x == zero => 0,
                x if x == one => 1,
                x if x == two => 2,
                x if x == three => 3,
                x if x == four => 4,
                x if x == five => 5,
                x if x == six => 6,
                x if x == seven => 7,
                x if x == eight => 8,
                x if x == nine => 9,
                _ => panic!("unknown pattern {:?}", d),
            })
            .join("")
            .parse()
            .unwrap()
    }
}
