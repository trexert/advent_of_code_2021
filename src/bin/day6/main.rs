use array_init::array_init;

fn main() {
    let fishes: Vec<u8> = include_str!("input.txt")
        .lines()
        .next()
        .unwrap()
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect();

    let school = School::from_fishes(fishes);

    println!("part1 result: {}", simulate(&school, 80));
    println!("part2 result: {}", simulate(&school, 256));
}

fn simulate(initial_school: &School, steps: usize) -> usize {
    let mut school = initial_school.clone();

    for _ in 0..steps {
        school.step()
    }

    school.count_fish()
}

#[derive(Clone)]
struct School {
    counts: [usize; MAX_AGE],
}

impl School {
    fn from_fishes(fishes: Vec<u8>) -> Self {
        let mut counts = [0; MAX_AGE];
        for fish_age in fishes {
            counts[fish_age as usize] += 1;
        }
        School { counts }
    }

    fn step(&mut self) {
        self.counts = array_init(|i| match i {
            6 => self.counts[0] + self.counts[7],
            8 => self.counts[0],
            _ => self.counts[i + 1],
        });
    }

    fn count_fish(&self) -> usize {
        self.counts.iter().sum()
    }
}

const MAX_AGE: usize = 9;
