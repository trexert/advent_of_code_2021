fn main() {
    let report: Vec<&str> = include_str!("input.txt").lines().collect();

    println!("part1 result = {}", part1(&report));
    println!("part2 result = {}", part2(&report));
}

fn part1(report: &Vec<&str>) -> u32 {
    let mut counts = vec![0; report[0].chars().count()];
    report.iter().for_each(|&line| {
        line.chars()
            .enumerate()
            .for_each(|(pos, x)| counts[pos] += (x == '1') as usize)
    });

    let breakpoint = report.len() / 2;

    let gamma: String = counts
        .iter()
        .map(|&count| ((count > breakpoint) as u32).to_string())
        .collect();
    let epsilon: String = counts
        .iter()
        .map(|&count| ((count < breakpoint) as u32).to_string())
        .collect();

    u32::from_str_radix(&gamma, 2).unwrap() * u32::from_str_radix(&epsilon, 2).unwrap()
}

fn part2(report: &Vec<&str>) -> u32 {
    oxygen_rating(report) * carbon_rating(report)
}

fn oxygen_rating(report: &Vec<&str>) -> u32 {
    let mut potential_numbers = report.clone();
    let mut index_considered = 0;

    while potential_numbers.len() > 1 {
        let ones = potential_numbers
            .iter()
            .filter(|&&s| s.chars().nth(index_considered) == Some('1'))
            .count() as f64;

        potential_numbers = if ones >= potential_numbers.len() as f64 / 2.0 {
            potential_numbers
                .into_iter()
                .filter(|&s| s.chars().nth(index_considered) == Some('1'))
                .collect()
        } else {
            potential_numbers
                .into_iter()
                .filter(|&s| s.chars().nth(index_considered) == Some('0'))
                .collect()
        };

        index_considered += 1;
    }

    u32::from_str_radix(potential_numbers[0], 2).unwrap()
}

fn carbon_rating(report: &Vec<&str>) -> u32 {
    let mut potential_numbers = report.clone();
    let mut index_considered = 0;

    while potential_numbers.len() > 1 {
        let ones = potential_numbers
            .iter()
            .filter(|&&s| s.chars().nth(index_considered) == Some('1'))
            .count() as f64;

        potential_numbers = if ones < potential_numbers.len() as f64 / 2.0 {
            potential_numbers
                .into_iter()
                .filter(|&s| s.chars().nth(index_considered) == Some('1'))
                .collect()
        } else {
            potential_numbers
                .into_iter()
                .filter(|&s| s.chars().nth(index_considered) == Some('0'))
                .collect()
        };

        index_considered += 1;
    }

    u32::from_str_radix(potential_numbers[0], 2).unwrap()
}
