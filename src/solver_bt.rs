#![allow(unused)]
use std::{char, collections::{HashMap, HashSet}};
use itertools::Itertools;

type Column = (HashMap<char, u8>, char);
type Mapping = HashMap<char, Option<u8>>;

#[derive(Debug)]
struct InputBatch {
    unique_chars: Vec<char>,
    lines: Vec<String>,
    result: String,
    columns_count: usize,
    columns: Vec<Column>,
}

impl InputBatch {
    fn from_str(input: &str) -> Self {
        let unique_set: HashSet<char> = input.chars().filter(|ch| ch.is_alphabetic()).collect();
        let mut split_eq = input.trim().split("==");

        let mut lines: Vec<String> = split_eq.next().unwrap().split("+").map(|s| s.trim().chars().rev().collect()).collect();
        lines.sort_by(|a, b| b.len().cmp(&a.len()));

        let result: String = split_eq.next().unwrap().trim().chars().rev().collect();
        let columns_count = result.len();

        fn get_column_chars(col_idx: usize, lines: &Vec<String>) -> Vec<char> {
            let mut lines_chars = vec![];
            let mut lines_iter = lines.iter();
            while let Some(line) = lines_iter.next() {
                if let Some(line_char) = line.chars().nth(col_idx) {
                    lines_chars.push(line_char);
                } else { break; }
            }
            lines_chars
        }

        let columns: Vec<_> = (0..columns_count)
            .map(|idx| get_column_chars(idx, &lines).iter()
                .fold(HashMap::new(), |mut acc, &el| {
                    *acc.entry(el).or_insert(0u8) += 1;
                    acc
                }))
            .zip(result.chars())
            .collect();

        Self {
            unique_chars: unique_set.into_iter().collect(),
            lines,
            result,
            columns_count,
            columns,
        }
    }
}

fn apply_mapping_to_column(last_remainder: u8, col: &Column, mapping: &Mapping) -> Option<u8> {
    let lines_sum_value: u32 = (last_remainder as u32) + col.0.iter().map(|(k, &v)| v as u32 * mapping.get(k).unwrap().unwrap() as u32).sum::<u32>();
    let result_value = mapping.get(&col.1).unwrap().unwrap() as u32;
    if lines_sum_value % 10 == result_value { Some((lines_sum_value / 10) as u8) } else { None }
}

fn check_no_leftside_zeros(batch: &InputBatch, mapping: &Mapping) -> bool {
    for line in batch.lines.iter() {
        if let Some(left_side_char) = line.chars().last() {
            if let Some(left_side_value_option) = mapping.get(&left_side_char) {
                if let Some(left_side_value) = left_side_value_option {
                    if left_side_value == &0 {
                        return false;
                    }
                }
            }
        }
    }
    true
}

fn build_result(column_idx: usize, batch: &InputBatch, last_remainder: u8, mapping: &Mapping, column: &Column) -> Option<Mapping> {
    let column_solution = apply_mapping_to_column(last_remainder, column, mapping);

    if let Some(last_remainder) = column_solution {
        // Check if has any zero on left side in sum lines
        let all_mapping_done = mapping.values().all(|v| v.is_some());
        if all_mapping_done {
            let all_left_not_zero = batch.lines.iter()
                .filter_map(|s| s.chars().last())
                .map(|c| mapping.get(&c).unwrap().unwrap())
                .all(|v| v != 0);
            if !all_left_not_zero {
                return None;
            }

            // Check if has zero on left side of result
            if mapping.get(&batch.result.chars().last().unwrap()).unwrap().unwrap() == 0 {
                return None;
            }
        }

        // Seems valid
        return if column_idx == batch.columns_count - 1 {
            Some(mapping.clone())
        } else {
            find_column_mapping(
                column_idx + 1, 
                batch, 
                last_remainder, 
                mapping.clone())
        }
    }
    None
}

fn find_column_mapping(column_idx: usize, batch: &InputBatch, last_remainder: u8, mut mapping: Mapping) -> Option<Mapping> {
    let column = &batch.columns[column_idx];

    let mut required_chars: HashSet<char> = column.0.keys().copied().collect();
    required_chars.insert(column.1);

    let not_mapped_chars: HashSet<char> = required_chars.iter().copied().filter(|key| mapping.get(key).unwrap().is_none()).collect();
    let not_mappedchars_count = not_mapped_chars.len();

    // Nothing to map, no more permutations needed - just check
    if not_mappedchars_count == 0 {
        if let Some(value) = build_result(column_idx, batch, last_remainder, &mapping, column) {
            return Some(value);
        }
    }

    // Drop already mapped
    let possible_values: Vec<_> = (0..10u8).filter(|&v| !mapping.values().contains(&Some(v))).collect();

    // Start finding mapping for yet not mapped chars
    for permutation in possible_values.iter().permutations(not_mappedchars_count) {

        // Update mapping
        let update_iter = not_mapped_chars.iter()
            .copied()
            .zip(permutation.into_iter());
        for (key, &value) in update_iter {
            *mapping.get_mut(&key).unwrap() = Some(value);
        }

        // Fast validate new mapping
        if !check_no_leftside_zeros(batch, &mapping) {
            continue;
        } 

        // Find solution with recently updated mapping
        if let Some(value) = build_result(column_idx, batch, last_remainder, &mapping, column) {
            return Some(value);
        }
    }

    None
}

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let batch = InputBatch::from_str(input);
    let result = find_column_mapping(
        0,
        &batch,
        0,
        HashMap::from_iter(batch.unique_chars.iter().copied().zip((0..batch.unique_chars.len()).map(|_| None)))
    );
    result.map(|r| r.into_iter()
        .map(|(k, v)| (k, v.unwrap())).collect())
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

#[test]
fn puzzle_with_ten_letters_and_199_addends() {
    let answer = solve("THIS + A + FIRE + THEREFORE + FOR + ALL + HISTORIES + I + TELL + A + TALE + THAT + FALSIFIES + ITS + TITLE + TIS + A + LIE + THE + TALE + OF + THE + LAST + FIRE + HORSES + LATE + AFTER + THE + FIRST + FATHERS + FORESEE + THE + HORRORS + THE + LAST + FREE + TROLL + TERRIFIES + THE + HORSES + OF + FIRE + THE + TROLL + RESTS + AT + THE + HOLE + OF + LOSSES + IT + IS + THERE + THAT + SHE + STORES + ROLES + OF + LEATHERS + AFTER + SHE + SATISFIES + HER + HATE + OFF + THOSE + FEARS + A + TASTE + RISES + AS + SHE + HEARS + THE + LEAST + FAR + HORSE + THOSE + FAST + HORSES + THAT + FIRST + HEAR + THE + TROLL + FLEE + OFF + TO + THE + FOREST + THE + HORSES + THAT + ALERTS + RAISE + THE + STARES + OF + THE + OTHERS + AS + THE + TROLL + ASSAILS + AT + THE + TOTAL + SHIFT + HER + TEETH + TEAR + HOOF + OFF + TORSO + AS + THE + LAST + HORSE + FORFEITS + ITS + LIFE + THE + FIRST + FATHERS + HEAR + OF + THE + HORRORS + THEIR + FEARS + THAT + THE + FIRES + FOR + THEIR + FEASTS + ARREST + AS + THE + FIRST + FATHERS + RESETTLE + THE + LAST + OF + THE + FIRE + HORSES + THE + LAST + TROLL + HARASSES + THE + FOREST + HEART + FREE + AT + LAST + OF + THE + LAST + TROLL + ALL + OFFER + THEIR + FIRE + HEAT + TO + THE + ASSISTERS + FAR + OFF + THE + TROLL + FASTS + ITS + LIFE + SHORTER + AS + STARS + RISE + THE + HORSES + REST + SAFE + AFTER + ALL + SHARE + HOT + FISH + AS + THEIR + AFFILIATES + TAILOR + A + ROOFS + FOR + THEIR + SAFE == FORTRESSES");
    let expected = [
        ('A', 1),
        ('E', 0),
        ('F', 5),
        ('H', 8),
        ('I', 7),
        ('L', 2),
        ('O', 6),
        ('R', 3),
        ('S', 4),
        ('T', 9),
    ]
    .into_iter()
    .collect();
    assert_eq!(answer, Some(expected));
}