use utils;
use utils::Part;
use std::collections::HashSet;

pub fn solve(part: Part) -> i32 {
    let mut input = String::new();
    utils::read_input_to_string(&mut input, 5).unwrap();

    do_the_thing(input, part)
}

fn do_the_thing(input:String, part:Part) -> i32 {
    let polymer_stack = match part {
        Part::One => get_reacted_polymer(&input),
        Part::Two => find_min_polymer(&input),
    } ;

    polymer_stack.len() as i32
}

fn get_reacted_polymer(input:&str) -> Vec<char> {
    let mut polymer_stack:Vec<char> = vec![];

    for c in input.chars() {
        match polymer_stack.clone().last() {
            None => polymer_stack.push(c),
            Some(unit) => {
                if (unit.to_uppercase().to_string() == c.to_uppercase().to_string())
                    && (unit.is_uppercase() ^ c.is_uppercase()) {
                    polymer_stack.pop();
                }
                    else {
                        polymer_stack.push(c);
                    }
            }
        }
    }

    polymer_stack
}

fn find_min_polymer(input:&str) -> Vec<char> {
    let unique_chars:HashSet<char> = input.chars().map(|c| c.to_lowercase().next().unwrap()).collect();

    unique_chars
        .iter()
        .map(|c| input.replace(*c, "").replace(c.to_uppercase().next().unwrap(), ""))
        .map(|s| get_reacted_polymer(&s))
        .min_by_key(|polymer| polymer.len())
        .unwrap()
}