use std::collections::HashSet;
use utils;
use utils::Part;

pub fn solve(part: Part) -> i32 {
    let mut input = String::new();
    utils::read_input_to_string(&mut input, 1).unwrap();

    find_frequency(input, part)
}

pub fn find_frequency(input: String, part: Part) -> i32 {
    let changes = input
        .trim()
        .split('\n')
        .map(|s| String::from(s))
        .map(|s| s.trim().parse::<i32>().unwrap());

    let result: i32 = match part {
        Part::One => changes.sum(),
        Part::Two => {
            let mut cycle = changes.cycle();
            let mut freq = 0;
            let mut seen_freqs = HashSet::new();
            let mut first_repeat = i32::min_value();
            'change_cycle: loop {
                match cycle.next() {
                    Some(change) => {
                        freq += change;
                        if seen_freqs.contains(&freq) {
                            first_repeat = freq;
                            break 'change_cycle;
                        } else {
                            seen_freqs.insert(freq);
                        }
                    }
                    None => break 'change_cycle,
                }
            }
            first_repeat
        }
    };

    result
}
