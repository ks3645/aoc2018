use std::collections::HashMap;
use std::iter;
use utils;
use utils::Part;

pub fn solve(part: Part) -> String {
    let mut input = String::new();
    utils::read_input_to_string(&mut input, 2).unwrap();

    match part {
        Part::One => get_checksum(input).to_string(),
        Part::Two => get_correct_boxes(input),
    }
}

fn get_checksum(input: String) -> i32 {
    let checksum_factors = input
        .lines()
        .map(|line| determine_line_type(line))
        .fold((0, 0), |acc, add| (acc.0 + add.0, acc.1 + add.1));

    (checksum_factors.0 * checksum_factors.1)
}

fn get_correct_boxes(input: String) -> String {
    input
        .lines()
        .enumerate()
        .flat_map(|(i, line)| iter::repeat(line).zip(input.lines().skip(i + 1)))
        .filter_map(|(one, two)| {
            if diff(one, two) == 1 {
                Some(get_intersected_string(one, two))
            } else {
                None
            }
        }).next()
        .unwrap()
}

fn get_intersected_string(one: &str, two: &str) -> String {
    one.chars()
        .zip(two.chars())
        .filter_map(|(c1, c2)| if c1 == c2 { Some(c1) } else { None })
        .collect()
}

fn diff(one: &str, two: &str) -> i32 {
    one.chars()
        .zip(two.chars())
        .filter_map(|(c1, c2)| if c1 != c2 { Some(1) } else { None })
        .sum()
}

fn get_char_counts(input: &str) -> HashMap<char, i32> {
    let mut counts = HashMap::new();

    for char in input.chars() {
        *counts.entry(char).or_insert(0) += 1;
    }

    counts
}

fn determine_line_type(line: &str) -> (i32, i32) {
    let char_counts = get_char_counts(line);

    (
        char_counts.values().any(|v| *v == 2) as i32,
        char_counts.values().any(|v| *v == 3) as i32,
    )
}
