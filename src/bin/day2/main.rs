use rusttype::Point;

fn main() {
    let instructions: Vec<Instruction> = include_str!("input.txt")
        .lines()
        .map(|s| Instruction::from_str(s))
        .collect();

    println!("part1 result = {}", part1(&instructions));
    println!("part2 result = {}", part2(&instructions));
}

fn part1(instructions: &Vec<Instruction>) -> u32 {
    let mut pos = Point { x: 0, y: 0 };
    instructions.iter().for_each(|ins| match ins {
        Instruction::Fwd(dist) => pos.x += dist,
        Instruction::Up(dist) => pos.y -= dist,
        Instruction::Down(dist) => pos.y += dist,
    });
    pos.x * pos.y
}

fn part2(instructions: &Vec<Instruction>) -> u32 {
    let mut pos = Point { x: 0, y: 0 };
    let mut angle = 0;
    instructions.iter().for_each(|ins| match ins {
        Instruction::Fwd(dist) => {
            pos.x += dist;
            pos.y += dist * angle
        }
        Instruction::Up(dist) => angle -= dist,
        Instruction::Down(dist) => angle += dist,
    });
    pos.x * pos.y
}

#[derive(Debug)]
enum Instruction {
    Fwd(u32),
    Up(u32),
    Down(u32),
}

impl Instruction {
    fn from_str(s: &str) -> Instruction {
        let mut split_s = s.split(" ");
        let direction = split_s.next().unwrap();
        let distance = split_s.next().unwrap().parse().unwrap();
        match direction {
            "forward" => Instruction::Fwd(distance),
            "up" => Instruction::Up(distance),
            "down" => Instruction::Down(distance),
            _ => panic!("Unexpected input!"),
        }
    }
}
