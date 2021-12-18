use itertools::Itertools;

fn main() {
    let input = include_str!("input.txt");

    println!("part1 result: {}", part1(input));
    println!("part2 result: {}", part2(input));
}

fn part1(input: &str) -> usize {
    input
        .lines()
        .filter_map(|line| {
            if let Err(score) = parse_line(line) {
                Some(score)
            } else {
                None
            }
        })
        .sum()
}

fn part2(input: &str) -> usize {
    let results: Vec<usize> = input
        .lines()
        .filter_map(|line| {
            if let Ok(remaining) = parse_line(line) {
                Some(calculate_leftover_score(remaining))
            } else {
                None
            }
        })
        .sorted()
        .collect();

    results[results.len() / 2]
}

fn parse_line(line: &str) -> Result<Vec<char>, usize> {
    let mut parse_stack = vec![];

    for b in line.chars() {
        match b {
            '(' | '[' | '{' | '<' => parse_stack.push(b),
            ')' => {
                if let Some('(') = parse_stack.pop() {
                } else {
                    return Err(3);
                }
            }
            ']' => {
                if let Some('[') = parse_stack.pop() {
                } else {
                    return Err(57);
                }
            }
            '}' => {
                if let Some('{') = parse_stack.pop() {
                } else {
                    return Err(1197);
                }
            }
            '>' => {
                if let Some('<') = parse_stack.pop() {
                } else {
                    return Err(25137);
                }
            }
            c => panic!("unexpected input: {}", c),
        }
    }

    Ok(parse_stack)
}

fn calculate_leftover_score(mut remainder: Vec<char>) -> usize {
    let mut score = 0;

    while let Some(b) = remainder.pop() {
        score *= 5;
        score += match b {
            '(' => 1,
            '[' => 2,
            '{' => 3,
            '<' => 4,
            c => panic!("unexpected character in remainder {}", c),
        }
    }

    score
}
