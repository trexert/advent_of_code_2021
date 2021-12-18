use rusttype::Point;
use std::collections::{BinaryHeap, HashSet};

fn main() {
    let map: Vec<Vec<usize>> = include_str!("input.txt")
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_string().parse().unwrap())
                .collect()
        })
        .collect();

    println!("part1 result: {}", part1(&map));
    println!("part2 result: {}", part2(&map));
}

fn part1(map: &Vec<Vec<usize>>) -> usize {
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

fn part2(map: &Vec<Vec<usize>>) -> usize {
    let mut basins: BinaryHeap<usize> = BinaryHeap::new();
    let mut visited = HashSet::new();
    for y in 0..map.len() as usize {
        for x in 0..map[0].len() as usize {
            let point = Point { x, y };
            if !visited.contains(&point) && map[y][x] < 9 {
                basins.push(find_basin_size(point, map, &mut visited));
            }
        }
    }

    basins
        .pop()
        .and_then(|a| basins.pop().and_then(|b| basins.pop().map(|c| a * b * c)))
        .unwrap()
}

fn find_basin_size(
    start_point: Point<usize>,
    map: &Vec<Vec<usize>>,
    visited: &mut HashSet<Point<usize>>,
) -> usize {
    let mut basin_size = 0;
    let mut points_to_visit = vec![start_point];
    visited.insert(start_point);

    while let Some(point) = points_to_visit.pop() {
        let x = point.x;
        let y = point.y;
        basin_size += 1;

        let opt_top = if y > 0 {
            Some(Point { x, y: y - 1 })
        } else {
            None
        };
        let opt_right = if x < map[0].len() - 1 {
            Some(Point { x: x + 1, y })
        } else {
            None
        };
        let opt_bottom = if y < map.len() - 1 {
            Some(Point { x, y: y + 1 })
        } else {
            None
        };
        let opt_left = if x > 0 {
            Some(Point { x: x - 1, y })
        } else {
            None
        };

        if let Some(top) = opt_top {
            if !visited.contains(&top) && map[top.y][top.x] < 9 {
                points_to_visit.push(top);
                visited.insert(top);
            }
        }
        if let Some(right) = opt_right {
            if !visited.contains(&right) && map[right.y][right.x] < 9 {
                points_to_visit.push(right);
                visited.insert(right);
            }
        }
        if let Some(bottom) = opt_bottom {
            if !visited.contains(&bottom) && map[bottom.y][bottom.x] < 9 {
                points_to_visit.push(bottom);
                visited.insert(bottom);
            }
        }
        if let Some(left) = opt_left {
            if !visited.contains(&left) && map[left.y][left.x] < 9 {
                points_to_visit.push(left);
                visited.insert(left);
            }
        }
    }

    basin_size
}
