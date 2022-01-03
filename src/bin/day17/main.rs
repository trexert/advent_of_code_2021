use itertools::Itertools;
use std::cmp::max;
use std::collections::{BTreeMap, BTreeSet};

fn main() {
    println!("part1 result: {}", part1());
    println!("part2 result: {}", part2());
}

fn part1() -> isize {
    let initial_velocity = TYMIN.abs() - 1;
    initial_velocity * (initial_velocity + 1) / 2
}

fn part2() -> usize {
    let mut x_initial_vs: BTreeMap<usize, Vec<isize>> = BTreeMap::new();
    let mut y_initial_vs: BTreeMap<usize, Vec<isize>> = BTreeMap::new();

    for initial_vx in 0..=TXMAX {
        for step_number in steps_with_initial_vx(initial_vx) {
            x_initial_vs
                .entry(step_number)
                .or_default()
                .push(initial_vx);
        }
    }

    for initial_vy in -TYMIN.abs()..TYMIN.abs() {
        for step_number in steps_with_initial_vy(initial_vy) {
            y_initial_vs
                .entry(step_number)
                .or_default()
                .push(initial_vy);
        }
    }

    let all_initial_vs = x_initial_vs
        .into_iter()
        .filter_map(|(step_count, vxs)| {
            y_initial_vs.get(&step_count).map(move |vys| {
                vxs.iter()
                    .flat_map(|&vx| vys.iter().map(move |&vy| (vx, vy)))
                    .collect_vec()
            })
        })
        .flatten()
        .collect::<BTreeSet<(isize, isize)>>();

    all_initial_vs.len()
}

fn steps_with_initial_vx(initial_v: isize) -> Vec<usize> {
    let mut v = initial_v;
    let mut x = 0;
    let mut steps = 0;

    let mut result = vec![];
    while x <= TXMAX && (v > 0 || (x >= TXMIN && steps <= MAX_STEPS)) {
        if x >= TXMIN {
            result.push(steps);
        }
        steps += 1;
        x += v;
        v = max(v - 1, 0);
    }

    result
}

fn steps_with_initial_vy(initial_v: isize) -> Vec<usize> {
    let mut v = initial_v;
    let mut y = 0;
    let mut steps = 0;

    let mut result = vec![];
    while y >= TYMIN {
        if y <= TYMAX {
            result.push(steps);
        }
        steps += 1;
        y += v;
        v = v - 1;
    }

    result
}

const TXMIN: isize = 207;
const TXMAX: isize = 263;
const TYMIN: isize = -115;
const TYMAX: isize = -63;
const MAX_STEPS: usize = (TYMIN.abs() * 2) as usize;
