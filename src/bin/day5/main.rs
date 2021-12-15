use crate::Orientation::{Diagonal, Horizontal, Vertical};
use itertools::Itertools;
use rusttype::Point;
use std::collections::HashMap;

fn main() {
    let input: Vec<_> = include_str!("input.txt")
        .lines()
        .map(|line| VentLine::from_input_line(line))
        .collect();

    println!("part1 result: {}", part1(&input));
    println!("part2 result: {}", part2(&input));
}

fn part1(ventlines: &Vec<VentLine>) -> usize {
    filtered_count(ventlines, |line| {
        [Horizontal, Vertical].contains(&line.orientation())
    })
}

fn part2(ventlines: &Vec<VentLine>) -> usize {
    filtered_count(ventlines, |_| true)
}

fn filtered_count<P>(ventlines: &Vec<VentLine>, p: P) -> usize
where
    P: Fn(&VentLine) -> bool,
{
    let mut grid: HashMap<Point<u16>, u8> = HashMap::new();

    for line in ventlines {
        if p(line) {
            for point in line.all_points() {
                let value = grid.entry(point).or_insert(0);
                *value = *value + 1;
            }
        }
    }

    grid.values().filter(|&&count| count >= 2).count()
}

#[derive(Debug)]
struct VentLine {
    start: Point<u16>,
    end: Point<u16>,
}

impl<'a> VentLine {
    fn from_input_line(input_line: &str) -> Self {
        let startend: Vec<Vec<_>> = input_line
            .split(" -> ")
            .map(|pos| {
                pos.split(",")
                    .map(|xory| xory.parse::<u16>().unwrap())
                    .collect()
            })
            .collect();
        VentLine {
            start: Point {
                x: startend[0][0],
                y: startend[0][1],
            },
            end: Point {
                x: startend[1][0],
                y: startend[1][1],
            },
        }
    }

    fn orientation(&self) -> Orientation {
        if self.start.y == self.end.y {
            Horizontal
        } else if self.start.x == self.end.x {
            Vertical
        } else {
            Diagonal
        }
    }

    fn all_points(&'a self) -> Box<dyn Iterator<Item = Point<u16>> + 'a> {
        let xs: Box<dyn Iterator<Item = u16>> = if self.start.x < self.end.x {
            Box::new(self.start.x..=self.end.x)
        } else {
            Box::new((self.end.x..=self.start.x).rev())
        };
        let ys: Box<dyn Iterator<Item = u16>> = if self.start.y < self.end.y {
            Box::new(self.start.y..=self.end.y)
        } else {
            Box::new((self.end.y..=self.start.y).rev())
        };

        match self.orientation() {
            Horizontal => Box::new(xs.map(|x| Point { x, y: self.start.y })),
            Vertical => Box::new(ys.map(|y| Point { x: self.start.x, y })),
            Diagonal => Box::new(xs.zip_eq(ys).map(|(x, y)| Point { x, y })),
        }
    }
}

#[derive(PartialEq)]
enum Orientation {
    Horizontal,
    Vertical,
    Diagonal,
}
