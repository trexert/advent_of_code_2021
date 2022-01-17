use crate::transform::T;
use itertools::Itertools;
use std::collections::HashSet;
use std::fmt::{Display, Formatter};

fn main() {}

fn parse_input(input: &str) -> Vec<Scanner> {
    input
        .split("\n\n")
        .map(|single_scanner| {
            let beacons: HashSet<_> = single_scanner
                .lines()
                .filter(|line| !line.starts_with("---"))
                .map(|pos_str| {
                    let mut xyz_str = pos_str.split(",").map(|s| s.parse().unwrap());
                    Point {
                        x: xyz_str.next().unwrap(),
                        y: xyz_str.next().unwrap(),
                        z: xyz_str.next().unwrap(),
                    }
                })
                .collect();
            Scanner::new(beacons)
        })
        .collect()
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

impl Point {
    fn transformed(&self, transformation: T) -> Point {
        let mut p = self.clone();
        let (c2, c3, c4) = transformation.destruc();
        for _ in 0..c2 {
            p = Point {
                x: -p.y,
                y: -p.x,
                z: -p.z,
            };
        }
        for _ in 0..c3 {
            p = Point {
                x: p.y,
                y: p.z,
                z: p.x,
            };
        }
        for _ in 0..c4 {
            p = Point {
                x: p.x,
                y: p.z,
                z: -p.y,
            };
        }
        p
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("({},{},{})", self.x, self.y, self.z))
    }
}

struct Scanner {
    beacons: Vec<HashSet<Point>>,
    other_scanners: HashSet<Point>,
}

impl Scanner {
    pub fn new(raw_beacons: HashSet<Point>) -> Self {
        let beacons = T::all()
            .map(|t| raw_beacons.iter().map(|b| b.transformed(t)).collect())
            .collect();
        let other_scanners = HashSet::new();
        Scanner {
            beacons,
            other_scanners,
        }
    }
}

mod transform {
    #[derive(Clone, Copy, Debug)]
    pub struct T(u8, u8, u8);

    impl T {
        pub fn new(c2: u8, c3: u8, c4: u8) -> Self {
            if c2 >= 2 || c3 >= 3 || c4 >= 4 {
                panic!("Order of rotation too large: {}, {}, {}", c2, c3, c4)
            }
            T(c2, c3, c4)
        }
        pub fn destruc(&self) -> (u8, u8, u8) {
            (self.0, self.1, self.2)
        }
        pub fn all() -> impl Iterator<Item = T> {
            TIter {
                c2: 0,
                c3: 0,
                c4: 0,
                first: true,
            }
        }
    }

    struct TIter {
        c2: u8,
        c3: u8,
        c4: u8,
        first: bool,
    }

    impl Iterator for TIter {
        type Item = T;

        fn next(&mut self) -> Option<Self::Item> {
            match (self.c2 < 1, self.c3 < 2, self.c4 < 3, self.first) {
                (_, _, _, true) => self.first = false,
                (_, _, true, false) => self.c4 += 1,
                (_, true, false, false) => {
                    self.c3 += 1;
                    self.c4 = 0
                }
                (true, false, false, false) => {
                    self.c2 += 1;
                    self.c3 = 0;
                    self.c4 = 0
                }
                (false, false, false, false) => return None,
            }
            Some(T(self.c2, self.c3, self.c4))
        }
    }
}

#[test]
fn all_transforms() {
    let test_points = [
        Point { x: 1, y: 0, z: 0 },
        Point { x: 0, y: 1, z: 0 },
        Point { x: 0, y: 0, z: 1 },
    ];
    let result = T::all()
        .map(|t| {
            let new_points = test_points
                .iter()
                .map(|p| p.transformed(t))
                .collect_tuple::<(Point, Point, Point)>()
                .unwrap();
            (new_points.0, new_points.1, new_points.2, t.destruc())
        })
        .collect_vec();
    assert_eq!(
        24,
        result
            .iter()
            .map(|(p1, p2, p3, _)| (p1, p2, p3))
            .unique()
            .count(),
        "\nexpected 24 distinct tuples, got: {:?}\n",
        result
            .iter()
            .sorted()
            .map(|ps| format!("({}, {}, {}, {:?})", ps.0, ps.1, ps.2, ps.3))
            .collect_vec(),
    );
}
