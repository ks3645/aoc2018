use utils;
use utils::Part;
use std::collections::HashSet;

pub fn solve(part: Part) -> i32 {
    let mut input = String::new();
    utils::read_input_to_string(&mut input, 1).unwrap();

    do_the_thing(input, part)
}

pub fn do_the_thing(input:String, part: Part) -> i32 {
    let changes:Vec<String> =  input.trim().split('\n').map(|s| String::from(s)).collect();

    let mut result_set = HashSet::new();

    let mut result = 0;

    let mut freq_changes = changes.clone().into_iter();
    let mut freq_cycle = changes.into_iter().cycle();

    'main: loop {
        let change;

        match part {
            Part::One => {
                match freq_changes.next() {
                    Some(thing) => change = thing,
                    None => break 'main,
                }
            },
            Part::Two => {
                match freq_cycle.next() {
                    Some(thing) => change = thing,
                    None => break 'main,
                }
            }
        }

        result += change.trim().parse::<i32>().unwrap();

        match part {
            Part::One => {},
            Part::Two => {
                if result_set.contains(&result) {
                    break 'main;
                } else {
                    result_set.insert(result);
                }
            }
        }

    }

    result
}

