#![feature(int_abs_diff)]

use array_init::array_init;

fn main() {
    let crabs: Vec<usize> = include_str!("input.txt")
        .trim()
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect();

    println!("part1 result: {}", part1(&crabs));
    println!("part2 result: {}", part2(&crabs));
}

fn part1(crabs: &Vec<usize>) -> usize {
    minimise(
        |x| calculate_cost(crabs, x, linear_metric),
        0,
        *crabs.iter().max().unwrap(),
    )
}

fn part2(crabs: &Vec<usize>) -> usize {
    minimise(
        |x| calculate_cost(crabs, x, quadratic_metric),
        0,
        *crabs.iter().max().unwrap(),
    )
}

fn calculate_cost<M>(crabs: &Vec<usize>, pos: usize, metric: M) -> usize
where
    M: Fn(usize, usize) -> usize,
{
    crabs.iter().map(|&crab| metric(crab, pos)).sum()
}

fn minimise<F>(f: F, start_min: usize, start_max: usize) -> usize
where
    F: Fn(usize) -> usize,
{
    let mut min = start_min;
    let mut max = start_max;

    while max - min > 5 {
        let pivots = [
            min,
            (3 * min + max) / 4,
            (min + max) / 2,
            (min + 3 * max) / 4,
            max,
        ];
        let values: [usize; 5] = array_init(|i| f(pivots[i]));
        let (min_index, _) = values
            .iter()
            .enumerate()
            .min_by_key(|(_, &value)| value)
            .unwrap();

        min = pivots[(min_index - 1).max(0)];
        max = pivots[(min_index + 2).min(4)];
    }

    (min..=max).map(|x| f(x)).min().unwrap()
}

fn linear_metric(a: usize, b: usize) -> usize {
    a.abs_diff(b)
}

fn quadratic_metric(a: usize, b: usize) -> usize {
    let dist = a.abs_diff(b);
    (dist * (dist + 1)) / 2
}
