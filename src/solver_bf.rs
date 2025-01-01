#![allow(unused)]
use std::collections::{HashMap, HashSet};
use itertools::Itertools;

#[derive(Debug)]
struct InputBatch {
    unique_chars: Vec<char>,
    lines: Vec<String>,
    result: String,
}

impl InputBatch {
    fn from_str(input: &str) -> Self {
        let unique_set: HashSet<char> = input.chars().filter(|ch| ch.is_alphabetic()).collect();
        let mut split_eq = input.trim().split("==");

        let mut lines: Vec<String> = split_eq.next().unwrap().split("+").map(|s| s.trim().chars().collect()).collect();
        lines.sort_by(|a, b| b.len().cmp(&a.len()));

        let result = split_eq.next().unwrap().trim().chars().collect();

        Self {
            unique_chars: unique_set.into_iter().collect(),
            lines,
            result
        }
    }

    fn applly_mapping_to_line(input: &str, mapping: &HashMap<char, u8>) -> Option<u64> {
        if mapping.get(&input.chars().nth(0).unwrap()).unwrap() == &0 {
            None
        } else {
            Some(input.chars().rev().enumerate().map(|(idx, ch)| 10u64.pow(idx as u32) * *mapping.get(&ch).unwrap() as u64).sum())
        }
    }

    fn map_lines_sum(&self, mapping: &HashMap<char, u8>) -> Option<u64> {
        self.lines.iter().map(|s| Self::applly_mapping_to_line(s, mapping)).sum()
    }

    fn map_line_result(&self, mapping: &HashMap<char, u8>) -> Option<u64> {
        Self::applly_mapping_to_line(&self.result, mapping)
    }

}


pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let batch = InputBatch::from_str(input);

    let k = batch.unique_chars.len();

    for permutation in (0..10u8).permutations(k) {
        let mapping: HashMap<char, u8> = batch.unique_chars.iter().copied().zip(permutation.into_iter()).collect();

        if let (Some(lines_sum_value), Some(result_value)) = (batch.map_lines_sum(&mapping), batch.map_line_result(&mapping)) {
            if lines_sum_value == result_value {
                return Some(mapping)
            }
        }
    }
    None
}


#[test]
fn puzzle_with_three_letters() {
    let answer = solve("I + BB == ILL");
    let expected = [('I', 1), ('B', 9), ('L', 0)].into_iter().collect();
    assert_eq!(answer, Some(expected));
}

#[test]
fn solution_must_have_unique_value_for_each_letter() {
    let answer = solve("A == B");
    assert_eq!(answer, None);
}

#[test]
fn leading_zero_solution_is_invalid() {
    let answer = solve("ACA + DD == BD");
    assert_eq!(answer, None);
}

#[test]
fn puzzle_with_two_digits_final_carry() {
    let answer = solve("A + A + A + A + A + A + A + A + A + A + A + B == BCC");
    let expected = [('A', 9), ('B', 1), ('C', 0)].into_iter().collect();
    assert_eq!(answer, Some(expected));
}

#[test]
fn puzzle_with_four_letters() {
    let answer = solve("AS + A == MOM");
    let expected = [('A', 9), ('S', 2), ('M', 1), ('O', 0)]
        .into_iter()
        .collect();
    assert_eq!(answer, Some(expected));
}

#[test]
fn puzzle_with_six_letters() {
    let answer = solve("NO + NO + TOO == LATE");
    let expected = [('N', 7), ('O', 4), ('T', 9), ('L', 1), ('A', 0), ('E', 2)]
        .into_iter()
        .collect();
    assert_eq!(answer, Some(expected));
}

#[test]
fn puzzle_with_seven_letters() {
    let answer = solve("HE + SEES + THE == LIGHT");
    let expected = [
        ('E', 4),
        ('G', 2),
        ('H', 5),
        ('I', 0),
        ('L', 1),
        ('S', 9),
        ('T', 7),
    ]
    .into_iter()
    .collect();
    assert_eq!(answer, Some(expected));
}

#[test]
fn puzzle_with_eight_letters() {
    let answer = solve("SEND + MORE == MONEY");
    let expected = [
        ('S', 9),
        ('E', 5),
        ('N', 6),
        ('D', 7),
        ('M', 1),
        ('O', 0),
        ('R', 8),
        ('Y', 2),
    ]
    .into_iter()
    .collect();
    assert_eq!(answer, Some(expected));
}

#[test]
fn puzzle_with_ten_letters() {
    let answer = solve("AND + A + STRONG + OFFENSE + AS + A + GOOD == DEFENSE");
    let expected = [
        ('A', 5),
        ('D', 3),
        ('E', 4),
        ('F', 7),
        ('G', 8),
        ('N', 0),
        ('O', 2),
        ('R', 1),
        ('S', 6),
        ('T', 9),
    ]
    .into_iter()
    .collect();
    assert_eq!(answer, Some(expected));
}