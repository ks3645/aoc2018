use utils;
use utils::Part;
use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::FromIterator;

pub fn solve(part: Part) -> String {
    let mut input = String::new();
    utils::read_input_to_string(&mut input, 2).unwrap();

    process(input, part)
}

fn process(input:String, part:Part) -> String {
    let data:Vec<(&str, HashMap<char, i32>)> = input.split('\n')
        .map(|s| s.trim())
        .map(|s| (s, get_char_counts(s)))
        .collect();


    match part {
       Part::One => {
           let checksum_factors = data.iter()
               .map(|data_tuple| determine_linetype(&data_tuple.1))
               .fold((0, 0), |acc, linetype| (acc.0 + linetype.value().0, acc.1 + linetype.value().1));

           (checksum_factors.0 * checksum_factors.1).to_string()
       },
        Part::Two => {
            let mut result = String::new();

            for i in 0..data.len() {
                for j in i+1..data.len() {
                    let diff = diff(data[i].0, data[j].0);
                    if diff == 1 {
                        result = get_intersected_string(data[i].0, data[j].0);
                    }
                }
            }

            result
        }
    }
}

fn get_intersected_string(word_one:&str, word_two:&str) -> String {
    let mut result = String::new();

    for i in 0..word_one.len() {
        if word_one[i..i+1] == word_two[i..i+1] {
            result += &word_one[i..i+1];
        }
    }

    result
}

fn diff(one:&str, two:&str) -> i32 {
    let mut diff = 0;
    for i in 0..one.len() {
        if one[i..i+1] != two[i..i+1] {
            diff += 1;
        }
    }
    diff
}

fn get_char_counts(input:&str) -> HashMap<char, i32> {
    let mut counts = HashMap::new();

    for char in input.chars() {
        *counts.entry(char).or_insert(0) += 1;
    }

    counts
}



#[derive(PartialEq)]
enum LineType {
    None,
    Two,
    Three,
    TwoAndThree,
}

impl LineType {
    fn value(&self) -> (u32, u32) {
        match *self {
            LineType::None => (0, 0),
            LineType::Two => (1, 0),
            LineType::Three => (0, 1),
            LineType::TwoAndThree => (1, 1)
        }
    }
}

fn determine_linetype (charcounts:&HashMap<char, i32>) -> LineType {
    let mut result = LineType::None;
    for (_,v) in charcounts {
        if *v == 2 {
            if result == LineType::Three {
                result = LineType::TwoAndThree;
                break;
            }
            else {
                result = LineType::Two;
            }
        }
        if *v == 3 {
            if result == LineType::Two {
                result = LineType::TwoAndThree;
                break;
            }
            else {
                result = LineType::Three;
            }
        }
    }

    result
}