use utils;
use utils::Part;
use std::collections::HashMap;

pub fn solve(part: Part) -> String {
    let mut input = String::new();
    utils::read_input_to_string(&mut input, 2).unwrap();

    match part {
        Part::One => do_the_thing(input).to_string(),
        Part::Two => do_part_two(input),
    }
}

fn do_part_two(input:String) -> String {
    let lines:Vec<&str> = input.split('\n').collect();

    let mut selected_line:usize = 0;

    let mut word_one = String::new();
    let mut word_two = String::new();

    'outer: loop {
        let first_word = lines[selected_line];

        println!("First Word: {}", first_word);

        for i in (selected_line+1)..lines.len() {
            let second_word = lines[i];

            println!("Second Word: {}", second_word);

            let mut diff_count = 0;

            for i in 0..first_word.len()-1 {
                if first_word[i..i+1] != second_word[i..i+1] {
                    diff_count += 1;
                }
            }

            println!("diff_count: {0}", diff_count);
            if diff_count == 1 {
                word_one = String::from(first_word);
                word_two = String::from(second_word);
                break 'outer;
            }
        }

        selected_line += 1;
        if selected_line == lines.len() {
            break 'outer;
        }
    }

    let mut result = String::new();

    for i in 0..word_one.len() - 1 {
        if word_one[i..i+1] == word_two[i..i+1] {
            result += &word_one[i..i+1];
        }
    }

    result
}

fn do_the_thing(input:String) -> i32 {
    let lines = input.split('\n');


    let mut twos_count = 0;
    let mut threes_count = 0;
    for line in lines {
        let linetype = determine_linetype(line);

        match linetype {
            LineType::None => {},
            LineType::Two => {
                twos_count += 1;
            }
            LineType::Three => {
                threes_count += 1;
            }
            LineType::TwoAndThree => {
                twos_count += 1;
                threes_count += 1;
            }
        }
    }

    twos_count * threes_count
}

#[derive(PartialEq)]
enum LineType {
    None,
    Two,
    Three,
    TwoAndThree,
}

fn determine_linetype (line:&str) -> LineType {
    let mut charcounts = HashMap::new();

    for c in line.chars() {
        if charcounts.contains_key(&c){
            let val = charcounts.get_mut(&c).unwrap();
            *val += 1;
        }
        else {
            charcounts.insert(c, 1);
        }
    }

    let mut result = LineType::None;
    for (_,v) in charcounts {
        if v == 2 {
            if result == LineType::Three || result == LineType::TwoAndThree {
                result = LineType::TwoAndThree;
            }
            else {
                result = LineType::Two;
            }
        }
        if v == 3 {
            if result == LineType::Two || result == LineType::TwoAndThree {
                result = LineType::TwoAndThree;
            }
                else {
                    result = LineType::Three;
                }
        }
    }

    result
}