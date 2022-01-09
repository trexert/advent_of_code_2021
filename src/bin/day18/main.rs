#![feature(if_let_guard)]
#![feature(assert_matches)]

use crate::ChildSide::{Left, Right};
use itertools::Itertools;
use std::iter::Sum;
use std::ops::{Add, DerefMut};
use std::str::Chars;

fn main() {
    println!("part1 result: {}", part1());
    println!("part2 result: {}", part2());
}

fn part1() -> usize {
    include_str!("input.txt")
        .lines()
        .map(|s| SnailNumber::from_str(s))
        .sum::<SnailNumber>()
        .magnitude()
}

fn part2() -> usize {
    let numbers_as_str = include_str!("input.txt").lines().collect_vec();

    numbers_as_str
        .iter()
        .flat_map(|x| {
            numbers_as_str.iter().map(move |y| {
                if x == y {
                    0
                } else {
                    (SnailNumber::from_str(x) + SnailNumber::from_str(y)).magnitude()
                }
            })
        })
        .max()
        .unwrap()
}

#[derive(Debug, PartialEq)]
struct SnailNumber {
    value: Box<SnailNode>,
}

impl SnailNumber {
    pub fn from_str(s: &str) -> Self {
        SnailNumber {
            value: SnailNode::from_chars(&mut s.chars(), None),
        }
    }

    pub fn magnitude(&self) -> usize {
        self.value.magnitude()
    }

    fn reduce(&mut self) {
        let mut made_change = true;
        while made_change {
            made_change = self.check_for_explosions();
            if !made_change {
                made_change = self.check_for_splits();
            }
        }
    }

    pub fn iter(&self) -> SnailIter {
        self.value.iter()
    }

    pub fn iter_mut(&mut self) -> SnailIterMut {
        self.value.iter_mut()
    }

    fn check_for_explosions(&mut self) -> bool {
        let mut made_change = false;
        let mut current_node = self.value.as_mut();
        while let Some(node) = current_node.next() {
            current_node = node;
            if current_node.value().is_none() && current_node.contents.depth >= 4 {
                current_node.explode();
                made_change = true;
                break;
            }
        }
        made_change
    }

    fn check_for_splits(&mut self) -> bool {
        let mut made_change = false;
        let mut current_node = self.value.as_mut();
        while let Some(node) = current_node.next() {
            current_node = node;
            match current_node.value() {
                Some(x) if x >= 10 => {
                    current_node.split();
                    made_change = true;
                    break;
                }
                _ => {}
            }
        }
        made_change
    }
}

impl Add for SnailNumber {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut result_node = SnailNode::new(None);
        let as_ptr: *mut SnailNode = &mut *result_node;
        let mut left = self.value;
        let mut right = rhs.value;
        left.links.parent = Some(Parent {
            node: as_ptr,
            side: Left,
        });
        right.links.parent = Some(Parent {
            node: as_ptr,
            side: Right,
        });
        left.iter_mut().for_each(|contents| contents.depth += 1);
        right.iter_mut().for_each(|contents| contents.depth += 1);

        result_node.links.children = Some(SnailPair { left, right });

        let mut result = SnailNumber { value: result_node };
        result.reduce();
        result
    }
}

impl ToString for SnailNumber {
    fn to_string(&self) -> String {
        self.value.to_string()
    }
}

impl Sum for SnailNumber {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.reduce(|acc, num| acc + num).unwrap()
    }
}

#[derive(Debug)]
struct SnailNode {
    links: SnailLinks,
    contents: SnailContents,
}

impl SnailNode {
    fn new(parent: Option<Parent>) -> Box<Self> {
        let depth = match &parent {
            Some(p) => unsafe { p.node.as_ref().unwrap().depth() + 1 },
            _ => 0,
        };
        Box::new(Self {
            links: SnailLinks {
                parent,
                children: None,
            },
            contents: SnailContents { value: None, depth },
        })
    }

    fn from_chars(chars: &mut Chars, parent: Option<Parent>) -> Box<Self> {
        let mut this = Self::new(parent);
        let as_ptr: *mut Self = &mut *this;
        match chars.next() {
            Some(c) if let Some(x) = c.to_digit(10) => this.contents.value = Some(x as u8),
            Some('[') => {
                let left = Self::from_chars(chars, Some(Parent { node: as_ptr, side: Left }));
                assert_eq!(chars.next(), Some(','));
                let right = Self::from_chars(chars, Some(Parent { node: as_ptr, side: Right }));
                assert_eq!(chars.next(), Some(']'));
                this.links.children = Some(SnailPair { left, right });
            },
            x => panic!("unexpected char.next: {:?}", x),
        };
        this
    }

    fn value(&self) -> Option<u8> {
        self.contents.value
    }

    fn depth(&self) -> u8 {
        self.contents.depth
    }

    fn parent(&self) -> Option<*mut Self> {
        self.links.parent.as_ref().map(|p| p.node)
    }

    fn sibling(&self) -> Option<&Self> {
        let parent = self.links.parent.as_ref();
        unsafe {
            parent.and_then(|p| match p.side {
                Left => (*p.node).child(Right),
                Right => (*p.node).child(Left),
            })
        }
    }

    fn sibling_mut(&self) -> Option<&mut Self> {
        let parent = self.links.parent.as_ref();
        unsafe {
            parent.and_then(|p| match p.side {
                Left => (*p.node).child_mut(Right),
                Right => (*p.node).child_mut(Left),
            })
        }
    }

    fn side(&self) -> Option<ChildSide> {
        self.links.parent.as_ref().map(|p| p.side)
    }

    fn child_mut(&mut self, side: ChildSide) -> Option<&mut Self> {
        self.links.children.as_mut().map(|kids| match side {
            Left => kids.left.deref_mut(),
            Right => kids.right.deref_mut(),
        })
    }

    fn child(&self, side: ChildSide) -> Option<&Self> {
        self.links.children.as_ref().map(|kids| match side {
            Left => kids.left.as_ref(),
            Right => kids.right.as_ref(),
        })
    }

    fn explode(&mut self) {
        self.neighbour(Left).map(|n| {
            *n.contents.value.as_mut().unwrap() += self.child(Left).unwrap().value().unwrap()
        });
        self.neighbour(Right).map(|n| {
            *n.contents.value.as_mut().unwrap() += self.child(Right).unwrap().value().unwrap()
        });
        self.links.children = None;
        self.contents.value = Some(0u8);
    }

    fn split(&mut self) {
        let value = self.value().unwrap();
        let left_value = value / 2;
        let mut left = Self::new(Some(Parent {
            node: self as *mut Self,
            side: Left,
        }));
        left.contents.value = Some(left_value);
        let right_value = value / 2 + value % 2;
        let mut right = Self::new(Some(Parent {
            node: self as *mut Self,
            side: Right,
        }));
        right.contents.value = Some(right_value);
        self.contents.value = None;
        self.links.children = Some(SnailPair { left, right });
    }

    fn bottom_child(&mut self, direction: ChildSide) -> &mut Self {
        let mut node = self;
        while node.child(direction).is_some() {
            node = node.child_mut(direction).unwrap();
        }
        node
    }

    fn neighbour(&self, side: ChildSide) -> Option<&mut Self> {
        let mut current_node = self;
        // Find first left turn.
        while current_node.side() == Some(side) {
            current_node = unsafe { &mut *current_node.parent().unwrap() };
        }

        // Take the left turn, and follow it to the right.
        current_node
            .sibling_mut()
            .map(|n| n.bottom_child(side.reverse()))
    }

    fn iter_mut(&mut self) -> SnailIterMut {
        SnailIterMut {
            next_node: Some(self),
        }
    }

    fn iter(&self) -> SnailIter {
        SnailIter {
            next_node: Some(self),
        }
    }

    fn next(&mut self) -> Option<&mut Self> {
        match (
            &mut self.links.children,
            &self.links.parent.as_ref().map(|p| p.side),
        ) {
            (Some(SnailPair { left, right: _ }), _) => Some(left),
            (None, Some(Left)) => unsafe {
                self.links
                    .parent
                    .as_mut()
                    .and_then(|p| (*p.node).child_mut(Right))
            },
            (None, Some(Right)) => {
                let mut potential_next = unsafe { &mut *self.links.parent.as_mut().unwrap().node };
                while potential_next.side() == Some(Right) {
                    potential_next = unsafe { &mut *potential_next.parent().unwrap() }
                }
                potential_next.sibling_mut()
            }
            _ => panic!("unexpected nothing"),
        }
    }

    fn magnitude(&self) -> usize {
        if let Some(children) = &self.links.children {
            3 * children.left.magnitude() + 2 * children.right.magnitude()
        } else {
            self.value().unwrap() as usize
        }
    }
}

impl ToString for SnailNode {
    fn to_string(&self) -> String {
        if let Some(children) = &self.links.children {
            format!(
                "[{},{}]",
                children.left.to_string(),
                children.right.to_string()
            )
        } else {
            self.contents.value.unwrap().to_string()
        }
    }
}

impl PartialEq for SnailNode {
    fn eq(&self, other: &Self) -> bool {
        self.value() == other.value()
            && self.depth() == other.depth()
            && self.side() == other.side()
            && self.child(Left) == other.child(Left)
            && self.child(Right) == other.child(Right)
    }
}

#[derive(Clone, Debug, PartialEq)]
struct SnailContents {
    value: Option<u8>,
    depth: u8,
}

#[derive(Debug, PartialEq)]
struct SnailLinks {
    parent: Option<Parent>,
    children: Option<SnailPair>,
}

#[derive(Debug, PartialEq)]
struct SnailPair {
    left: Box<SnailNode>,
    right: Box<SnailNode>,
}

#[derive(Debug, PartialEq)]
struct Parent {
    node: *mut SnailNode,
    side: ChildSide,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum ChildSide {
    Left,
    Right,
}

impl ChildSide {
    fn reverse(&self) -> Self {
        match self {
            Left => Right,
            Right => Left,
        }
    }
}

struct SnailIterMut<'a> {
    next_node: Option<&'a mut SnailNode>,
}

impl<'a> Iterator for SnailIterMut<'a> {
    type Item = &'a mut SnailContents;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(node) = self.next_node.take() {
            let to_emit = &mut node.contents;
            self.next_node = match (
                &mut node.links.children,
                &node.links.parent.as_ref().map(|p| p.side),
            ) {
                (Some(SnailPair { left, right: _ }), _) => Some(left),
                (None, Some(Left)) => unsafe {
                    node.links
                        .parent
                        .as_mut()
                        .and_then(|p| (*p.node).child_mut(Right))
                },
                (None, Some(Right)) => {
                    let mut potential_next =
                        unsafe { &mut *node.links.parent.as_mut().unwrap().node };
                    while potential_next.side() == Some(Right) {
                        potential_next = unsafe { &mut *potential_next.parent().unwrap() }
                    }
                    potential_next.sibling_mut()
                }
                _ => panic!("unexpected nothing"),
            };
            Some(to_emit)
        } else {
            None
        }
    }
}

struct SnailIter<'a> {
    next_node: Option<&'a SnailNode>,
}

impl<'a> Iterator for SnailIter<'a> {
    type Item = &'a SnailContents;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(node) = self.next_node {
            let to_emit = &node.contents;
            self.next_node = match (
                &node.links.children,
                &node.links.parent.as_ref().map(|p| p.side),
            ) {
                (Some(SnailPair { left, right: _ }), _) => Some(left),
                (None, Some(Left)) => unsafe {
                    node.links
                        .parent
                        .as_ref()
                        .and_then(|p| (*p.node).child(Right))
                },
                (None, Some(Right)) => {
                    let mut potential_next =
                        unsafe { &mut *node.links.parent.as_ref().unwrap().node };
                    while potential_next.side() == Some(Right) {
                        potential_next = unsafe { &mut *potential_next.parent().unwrap() }
                    }
                    potential_next.sibling()
                }
                _ => panic!("unexpected nothing"),
            };
            Some(to_emit)
        } else {
            None
        }
    }
}

#[test]
fn iter_mut() {
    let mut number = SnailNumber::from_str("[[1,2],3]");
    number.iter_mut().for_each(|node| node.depth += 1);
    assert_eq!(
        vec![1, 2, 3, 3, 2],
        number.iter_mut().map(|node| node.depth).collect_vec()
    );
}

#[test]
fn explode() {
    let mut number = SnailNumber::from_str("[[1,2],3]");
    number.value.child_mut(Left).unwrap().explode();
    assert_eq!(
        vec![0u8, 5u8],
        number
            .iter_mut()
            .filter_map(|node| node.value)
            .collect_vec()
    )
}

#[test]
fn split() {
    let mut number = SnailNumber::from_str("[[1,2],3]");
    number.value.child_mut(Right).unwrap().split();
    assert_eq!(
        vec![1u8, 2u8, 1u8, 2u8],
        number
            .iter_mut()
            .filter_map(|node| node.value)
            .collect_vec()
    )
}

#[test]
fn add() {
    let num1 = SnailNumber::from_str("[[1,2],3]");
    let num2 = SnailNumber::from_str("[4,[[5,6],7]]");
    assert_eq!(
        (num1 + num2).iter().map(|n| n.depth).collect_vec(),
        SnailNumber::from_str("[[[1,2],3],[4,[[5,6],7]]]")
            .iter()
            .map(|n| n.depth)
            .collect_vec(),
    )
}

#[test]
fn reduce() {
    let mut num = SnailNumber::from_str("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]");
    num.reduce();
    assert_eq!(
        SnailNumber::from_str("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"),
        num
    );
}

#[test]
fn sum() {
    let nums = "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]"
        .lines()
        .map(|s| SnailNumber::from_str(s));
    assert_eq!(
        SnailNumber::from_str("[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]"),
        nums.sum()
    );
}

#[test]
fn magnitude() {
    assert_eq!(143, SnailNumber::from_str("[[1,2],[[3,4],5]]").magnitude());
}
