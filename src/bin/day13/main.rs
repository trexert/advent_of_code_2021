use itertools::Itertools;
use std::collections::HashSet;
use std::fmt::{Debug, Formatter};

fn main() {
    let mut input = include_str!("input.txt").lines();

    let mut dots: Vec<(usize, usize)> = vec![];
    while let Some(line) = input.next() {
        if line == "" {
            break;
        }

        let mut xy = line.split(",");
        dots.push((
            xy.next().unwrap().parse().unwrap(),
            xy.next().unwrap().parse().unwrap(),
        ));
    }
    let paper = DottedPaper::from_dots(dots);

    let instructions: Vec<_> = input
        .map(|s| {
            let mut instruction = s.strip_prefix("fold along ").unwrap().split("=");
            let dir = match instruction.next().unwrap() {
                "x" => Direction::X,
                "y" => Direction::Y,
                unexpected => panic!("unexpected input {}", unexpected),
            };
            let pos = instruction.next().unwrap().parse().unwrap();

            (dir, pos)
        })
        .collect();

    println!("part1 result: {}", part1(&paper, &instructions));
    println!("part2 result:\n{:?}", part2(&paper, &instructions));
}

fn part1(paper: &DottedPaper, instructions: &Vec<(Direction, usize)>) -> usize {
    let mut _paper = paper.clone();
    _paper.make_fold(instructions[0].0, instructions[0].1);
    _paper.paper.len()
}

fn part2(paper: &DottedPaper, instructions: &Vec<(Direction, usize)>) -> DottedPaper {
    let mut _paper = paper.clone();
    instructions
        .iter()
        .for_each(|&(dir, pos)| _paper.make_fold(dir, pos));
    _paper
}

#[derive(Clone)]
struct DottedPaper {
    paper: HashSet<(usize, usize)>,
}

impl DottedPaper {
    fn from_dots(dots: impl IntoIterator<Item = (usize, usize)>) -> Self {
        DottedPaper {
            paper: dots.into_iter().collect(),
        }
    }

    fn make_fold(&mut self, dir: Direction, pos: usize) {
        self.paper = self
            .paper
            .iter()
            .map(|&(x, y)| match dir {
                Direction::X => (if x > pos { 2 * pos - x } else { x }, y),
                Direction::Y => (x, if y > pos { 2 * pos - y } else { y }),
            })
            .collect()
    }
}

impl Debug for DottedPaper {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let max_x = self.paper.iter().max_by_key(|(x, _)| x).unwrap().0;
        let max_y = self.paper.iter().max_by_key(|(_, y)| y).unwrap().1;

        let to_print = (0..=max_y)
            .map(|y| {
                (0..=max_x)
                    .map(|x| {
                        if self.paper.contains(&(x, y)) {
                            "#"
                        } else {
                            "."
                        }
                    })
                    .join("")
            })
            .join("\n");

        f.write_str(&to_print)
    }
}

#[derive(Copy, Clone)]
enum Direction {
    X,
    Y,
}
