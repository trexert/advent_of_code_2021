use itertools::Itertools;

fn main() {
    let report: Vec<u32> = include_str!("input.txt")
        .lines()
        .map(|s| s.parse().unwrap())
        .collect();

    println!("part1 result = {}", part1(&report));
    println!("part2 result = {}", part2(&report));
}

fn part1(report: &Vec<u32>) -> u32 {
    count_increases(report)
}

fn part2(report: &Vec<u32>) -> u32 {
    let window_sums: Vec<u32> = report
        .as_slice()
        .windows(3)
        .map(|window| window.iter().sum())
        .collect();
    count_increases(&window_sums)
}

fn count_increases(list_of_things: &Vec<u32>) -> u32 {
    list_of_things
        .iter()
        .tuple_windows()
        .map(|(prev, curr)| (curr > prev) as u32)
        .sum()
}
