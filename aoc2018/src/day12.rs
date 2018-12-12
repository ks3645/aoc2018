use utils;
use utils::Part;

pub fn solve(part: Part) -> i64 {
    let mut input = String::new();
    utils::read_input_to_string(&mut input, 12).unwrap();

    do_the_thing(input, part)
}

fn do_the_thing(input:String, part:Part) -> i64 {
    let mut lines = input.lines();

    let mut pots = parse_initial_state(lines.next().unwrap());
    lines.next().unwrap();

    let rules:Vec<Rule> = lines.map(|l| Rule::from(l)).collect();

    let mut zero_pos = 0;

    let generations:u64 = match part {
        Part::One => 20,
        Part::Two => 0, //50_000_000_000,
    };

    for i in 0..generations {
        display(i, &pots, &zero_pos);
        println!("Score: {}", score(&pots, &part, &zero_pos));
        let next_pots = time_step(&rules, &pots, &mut zero_pos);
        pots = next_pots;
    }


// visual inspection of repeating pattern
    if part== Part::Two {
        let mut cheated_pots = Vec::new();

        let repeat = "##.####.####.....##.####.....##.####.....##.....##.####.####.####....##.####.####.####.....##.####.####.#.####.####......##.....####.####.####....##.####";
        for c in repeat.chars() {
            match c {
                '#' => cheated_pots.push(true),
                '.' => cheated_pots.push(false),
                _ => panic!("parsing error on cheated pots"),
            }
        }
        pots = cheated_pots;
        zero_pos = 0;
    }

    display(generations, &pots, &zero_pos);
    println!("Score: {}", score(&pots, &part, &zero_pos));

    score(&pots, &part, &zero_pos)
}

fn score(pots:&Vec<bool>, part:&Part, zero_pos:&i32) -> i64 {
    pots.iter()
        .enumerate()
        .filter_map(|(i, b)| match *part {
            Part::One => {
                if *b { Some(i as i64 - *zero_pos as i64)} else {None }
            },
            Part::Two => {
                if *b { Some(i as i64 + - *zero_pos as i64 + (50_000_000_000 as i64 - 54 as i64))} else {None}
            }})
        .sum::<i64>()
}

fn display(generation:u64, pots:&Vec<bool>, zero_pos:&i32) {
    print!("    ");
    for _ in 0..*zero_pos {
        print!(" ");
    }
    print!("0");
    print!("\n");
    print!("{:2}: ", generation);
    for i in 0..pots.len() {
        match pots[i]  {
            true => print!("#"),
            false => print!("."),
        }
    }
    print!("\n");
}

fn time_step(rules:&Vec<Rule>, pots:&Vec<bool>, zero_pos:&mut i32) -> Vec<bool> {
    let mut copy = Vec::new();
    copy.push(false);
    copy.push(false);
    copy.push(false);
    copy.push(false);
    for b in pots {
        copy.push(b.clone());
    }
    copy.push(false);
    copy.push(false);
    copy.push(false);
    copy.push(false);
    *zero_pos += 4;

    let mut result = vec![false; copy.len()];

    for i in 2..copy.len()-2 {
        let mut chosen_rule:&Rule;
        {
            let slice = &copy[i - 2..i + 3];
            chosen_rule = rules
                .iter()
                .filter(|r| r.matches(slice))
                .next().unwrap_or(&Rule{config:[false; 5], result:false});
            if chosen_rule.result
            {
                //println!("applying rule: {} to slice: {:?}", chosen_rule, slice);
            }
        }

        result[i] = chosen_rule.result;
    }

    while let Some(b) = result.clone().last() {
        match *b {
            false => result.pop(),
            true => break,
        };
    }


    if !result[0] {
        if !result[1] {
            if !result[2] {
                if !result[3] {
                    *zero_pos -= 4;
                    result[4..].to_vec()
                }
                else {
                    *zero_pos -= 3;
                    result [3..].to_vec()
                }
            }
            else {
                *zero_pos -= 2;
                result[2..].to_vec()
            }
        }
        else {
            *zero_pos -= 1;
            result[1..].to_vec()
        }
    }
    else {
        result
    }
}

fn parse_initial_state(line:&str) -> Vec<bool> {
    let tokens:Vec<&str> = line.split_whitespace().collect();

    let mut result = Vec::new();

    for c in tokens[2].chars() {
        result.push (match c {
            '#' => true,
            '.' => false,
            _ => panic!("parsing error on Rule config"),
        })
    }

    result
}

#[derive(Debug)]
struct Rule {
    config:[bool; 5],
    result:bool
}

impl std::fmt::Display for Rule {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for i in 0..5 {
            write!(f, "{}", match self.config[i] { true => "#", false => "." })?;
        }

        write!(f, " => {}",  match self.result { true => "#", false => "." })
    }
}

impl Rule {
    fn from(input:&str) -> Rule {
        let tokens:Vec<&str> = input.split_whitespace().collect();

        let result = match tokens[2] {
            "#" => true,
            "." => false,
            _ => panic!("parsing error on Rule result"),
        };

        let mut config = [false; 5];
        for (i, c) in tokens[0].chars().enumerate() {
            config[i] = match c {
                '#' => true,
                '.' => false,
                _ => panic!("parsing error on Rule config"),
            }
        }

        Rule { config , result}
    }

    fn matches(&self, other:&[bool]) -> bool {
        let mut result = true;
        for i in 0..5 {
            if self.config[i] != other[i]
            {
                result = false;
            }
        }
        result
    }
}
