use itertools::Itertools;
use ndarray::Array2;
use std::collections::BTreeSet;

fn main() {
    let mut input_lines = include_str!("input.txt").lines();
    let drawn_numbers = input_lines
        .next()
        .unwrap()
        .split(",")
        .map(|s| s.parse::<u8>().unwrap())
        .collect();
    input_lines.next();
    let card_chunks = input_lines.chunks(CARD_SIZE + 1);
    let cards: Vec<BingoCard> = card_chunks
        .into_iter()
        .map(|card_lines| BingoCard::from_lines(card_lines.take(5)))
        .collect();

    println!("part1 result: {}", part1(&drawn_numbers, &cards));
    println!("part2 result: {}", part2(&drawn_numbers, &cards));
}

fn part1(drawn_numbers: &Vec<u8>, cards: &Vec<BingoCard>) -> usize {
    let mut _cards = cards.clone();
    for &number in drawn_numbers {
        if let Some(result) = _cards
            .iter_mut()
            .filter_map(|card| {
                if card.check_number(number) {
                    Some(card.calculate_score(number))
                } else {
                    None
                }
            })
            .next()
        {
            return result;
        }
    }

    panic!("Should have a result");
}

fn part2(drawn_numbers: &Vec<u8>, cards: &Vec<BingoCard>) -> usize {
    let mut _cards = cards.clone();
    let mut most_recent_result: Option<usize> = None;
    for &number in drawn_numbers {
        _cards = _cards
            .into_iter()
            .filter_map(|mut card| {
                if !card.check_number(number) {
                    Some(card)
                } else {
                    most_recent_result = Some(card.calculate_score(number));
                    None
                }
            })
            .collect();
    }

    most_recent_result.expect("Should have a result")
}

#[derive(Clone, Debug)]
struct BingoCard {
    card: Array2<u8>,
    numbers: BTreeSet<u8>,
    checked: Array2<bool>,
}

impl BingoCard {
    fn from_lines<'a>(lines: impl Iterator<Item = &'a str>) -> BingoCard {
        let mut card = Array2::<u8>::zeros([CARD_SIZE, CARD_SIZE]);
        let mut numbers = BTreeSet::new();
        for (rowno, line) in lines.enumerate() {
            for (colno, number) in line.split_whitespace().enumerate() {
                let parsed_number = number.parse().unwrap();
                card[[rowno, colno]] = parsed_number;
                numbers.insert(parsed_number);
            }
        }

        BingoCard {
            card,
            numbers,
            checked: Array2::<bool>::from_elem([CARD_SIZE, CARD_SIZE], false),
        }
    }

    fn check_number(&mut self, called_number: u8) -> bool {
        if !&self.numbers.contains(&called_number) {
            return false;
        }

        let pos = self
            .card
            .iter()
            .position(|&value| value == called_number)
            .unwrap();
        let (rowno, colno) = (pos / CARD_SIZE, pos % CARD_SIZE);
        self.checked[[rowno, colno]] = true;

        self.check_row(rowno) || self.check_column(colno)
    }

    fn check_row(&self, rowno: usize) -> bool {
        self.checked.row(rowno).iter().all(|&is_checked| is_checked)
    }

    fn check_column(&self, colno: usize) -> bool {
        self.checked
            .column(colno)
            .iter()
            .all(|&is_checked| is_checked)
    }

    fn calculate_score(&self, just_called: u8) -> usize {
        self.checked
            .iter()
            .zip_eq(self.card.iter())
            .filter_map(|(&is_checked, &number)| {
                if !is_checked {
                    Some(number as usize)
                } else {
                    None
                }
            })
            .sum::<usize>()
            * just_called as usize
    }
}

const CARD_SIZE: usize = 5;
