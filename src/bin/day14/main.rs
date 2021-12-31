use itertools::Itertools;
use std::collections::HashMap;

fn main() {
    let polymer = Polymer::from_input(include_str!("input.txt"));

    println!("part1 result: {}", part1(&polymer));
    println!("part2 result: {}", part2(&polymer));
}

fn part1(polymer: &Polymer) -> usize {
    polymer.clone().perform_steps(10)
}

fn part2(polymer: &Polymer) -> usize {
    polymer.clone().perform_steps(40)
}

#[derive(Clone)]
struct Polymer {
    state: HashMap<(char, char), usize>,
    template: HashMap<(char, char), char>,
    first: char,
    last: char,
}
impl Polymer {
    fn from_input(input: &str) -> Self {
        let mut lines = input.lines();
        let mut state = HashMap::new();
        let initial_polymer = lines.next().unwrap();
        initial_polymer
            .chars()
            .tuple_windows()
            .for_each(|(a, b)| *state.entry((a, b)).or_default() += 1);
        // Skip empty line
        lines.next();
        let template: HashMap<(char, char), char> = lines
            .map(|line| {
                let mut instruction = line.split(" -> ");
                let from: (char, char) =
                    instruction.next().unwrap().chars().collect_tuple().unwrap();
                let to: char = instruction.next().unwrap().chars().next().unwrap();
                (from, to)
            })
            .collect();
        Polymer {
            state,
            template,
            first: initial_polymer.chars().next().unwrap(),
            last: initial_polymer.chars().last().unwrap(),
        }
    }

    fn perform_step(&mut self) {
        let mut new_state: HashMap<(char, char), usize> = HashMap::new();
        for (&(a, b), &count) in self.state.iter() {
            if let Some(&to_insert) = self.template.get(&(a, b)) {
                *new_state.entry((a, to_insert)).or_default() += count;
                *new_state.entry((to_insert, b)).or_default() += count;
            } else {
                println!("unknown pair");
                *new_state.entry((a, b)).or_default() += count;
            }
        }
        self.state = new_state;
    }

    fn perform_steps(&mut self, step_count: usize) -> usize {
        for _ in 0..step_count {
            self.perform_step();
        }
        let counts = self.count_elements();
        counts.values().max().unwrap() - counts.values().min().unwrap()
    }

    fn count_elements(&self) -> HashMap<char, usize> {
        let mut counts: HashMap<char, usize> = HashMap::new();
        self.state.iter().for_each(|((a, b), count)| {
            *counts.entry(*a).or_default() += count;
            *counts.entry(*b).or_default() += count;
        });
        *counts.entry(self.first).or_default() += 1;
        *counts.entry(self.last).or_default() += 1;
        counts
            .into_iter()
            .map(|(c, count)| (c, count / 2))
            .collect()
    }
}
