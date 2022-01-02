use ndarray::Array2;
use rusttype::Point;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};

fn main() {
    let risks: Vec<u8> = include_str!("input.txt")
        .chars()
        .filter_map(|c| c.to_digit(10).map(|d| d as u8))
        .collect();
    let size = (risks.len() as f64).sqrt() as usize;
    let map = Array2::from_shape_vec([size, size], risks).unwrap();
    let big_map = make_big_map(&map);

    println!("part1 result: {}", dijkstra(&map));
    println!("part2 result: {}", dijkstra(&big_map));
}

fn dijkstra(map: &Array2<u8>) -> usize {
    let max_index = map.shape()[0] - 1;
    let start_pos = Point { x: 0, y: 0 };
    let end_pos = Point {
        x: max_index,
        y: max_index,
    };

    let mut visited = HashSet::new();
    let mut current_state = State {
        pos: start_pos,
        cost: 0,
    };
    let mut queue = BinaryHeap::new();
    while current_state.pos != end_pos {
        if !visited.contains(&current_state.pos) {
            visited.insert(current_state.pos);

            for neighbour in get_neighbours(current_state.pos, max_index) {
                queue.push(State {
                    pos: neighbour,
                    cost: current_state.cost + map[[neighbour.y, neighbour.x]] as usize,
                })
            }
        }

        current_state = queue.pop().unwrap();
    }

    current_state.cost
}

fn make_big_map(base_tile: &Array2<u8>) -> Array2<u8> {
    let tile_size = base_tile.shape()[0];
    let total_size = tile_size * 5;

    Array2::from_shape_fn([total_size, total_size], |(x, y)| {
        let source_x = x % tile_size;
        let inc_x = (x / tile_size) as u8;
        let source_y = y % tile_size;
        let inc_y = (y / tile_size) as u8;
        ((base_tile[[source_x, source_y]] - 1 + inc_x + inc_y) % 9) + 1
    })
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct State {
    pos: Point<usize>,
    cost: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.pos.cmp(&other.pos))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn get_neighbours(pos: Point<usize>, max_index: usize) -> Vec<Point<usize>> {
    let mut result = vec![];

    if pos.x > 0 {
        result.push(Point {
            x: pos.x - 1,
            y: pos.y,
        });
    }
    if pos.y > 0 {
        result.push(Point {
            x: pos.x,
            y: pos.y - 1,
        });
    }
    if pos.x < max_index {
        result.push(Point {
            x: pos.x + 1,
            y: pos.y,
        });
    }
    if pos.y < max_index {
        result.push(Point {
            x: pos.x,
            y: pos.y + 1,
        });
    }

    result
}
