fn main() {
    let map: Vec<Vec<u8>> = include_str!("input.txt")
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_string().parse().unwrap())
                .collect()
        })
        .collect();

    println!("part1 result: {}", part1(&map));
}

fn part1(map: &Vec<Vec<u8>>) -> usize {
    let size = map.len();
    let mut low_point_scores = 0;
    for y in 0..size {
        for x in 0..size {
            let height = map[y][x];
            if (y == 0 || height < map[y - 1][x])
                && (x == 0 || height < map[y][x - 1])
                && (y >= size - 1 || height < map[y + 1][x])
                && (x >= size - 1 || height < map[y][x + 1])
            {
                low_point_scores += 1 + height as usize
            }
        }
    }
    low_point_scores
}
