#![feature(hash_drain_filter)]

use itertools::Itertools;
use ndarray::Array2;
use rusttype::Point;
use std::cmp::min;
use std::collections::{HashMap, HashSet};

fn main() {
    let starting_grid = Octogrid::from_input(include_str!("input.txt"));

    println!("part1 result: {}", part1(&starting_grid));
    println!("part2 result: {}", part2(&starting_grid));
}

fn part1(starting_grid: &Octogrid) -> usize {
    let mut grid = starting_grid.clone();
    let mut result = 0;

    for _ in 0..100 {
        result += grid.step();
        // println!("{:?}", grid);
    }

    result
}

fn part2(starting_grid: &Octogrid) -> usize {
    let mut grid = starting_grid.clone();
    let mut result = 1;

    while grid.step() < SIZE * SIZE {
        result += 1;
    }

    result
}

#[derive(Clone, Debug)]
struct Octogrid {
    grid: Array2<u8>,
}

impl Octogrid {
    fn from_input(input: &str) -> Self {
        let grid = Array2::from_shape_vec(
            [SIZE, SIZE],
            input
                .chars()
                .filter(|c| ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'].contains(c))
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect_vec(),
        )
        .unwrap();

        Octogrid { grid }
    }

    fn step(&mut self) -> usize {
        let mut flashees: HashMap<Point<usize>, usize> = HashMap::new();
        let mut flashers: HashSet<Point<usize>> = HashSet::new();

        self.grid.iter_mut().for_each(|oct| *oct += 1);

        loop {
            for y in 0..SIZE {
                for x in 0..SIZE {
                    if self.grid[[y, x]] > 9 {
                        assert!(!flashers.contains(&Point { x, y }));
                        get_flashees(Point { x, y }).into_iter().for_each(|pos| {
                            *flashees.entry(pos).or_default() += 1;
                        });
                        flashers.insert(Point { x, y });
                        self.grid[[y, x]] = 0;
                    }
                }
            }

            flashees.drain_filter(|pos, _| flashers.contains(pos));

            if flashees.is_empty() {
                break;
            }

            flashees
                .drain()
                .for_each(|(pos, count)| self.grid[[pos.y, pos.x]] += count as u8);
        }

        flashers.len()
    }
}

fn get_flashees(flashed: Point<usize>) -> Vec<Point<usize>> {
    (flashed.y.checked_sub(1).unwrap_or(0)..min(SIZE, flashed.y + 2))
        .flat_map(|y| {
            (flashed.x.checked_sub(1).unwrap_or(0)..min(SIZE, flashed.x + 2))
                .map(move |x| Point { x, y })
        })
        .filter(|pos| *pos != flashed)
        .collect()
}

const SIZE: usize = 10;
