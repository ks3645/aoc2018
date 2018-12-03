use utils;
use utils::Part;
use std::collections::HashSet;

pub fn solve(part: Part) -> i32 {
    let mut input = String::new();
    utils::read_input_to_string(&mut input, 3).unwrap();

    do_the_thing(input, part)
}

fn do_the_thing(input:String, part:Part) -> i32 {


    let mut fabric = vec![vec![0;1000];1000];

    let mut claims = HashSet::new();

    for line in input.lines() {
        let claim = Claim::from(line);
        claims.insert(claim);
        apply_claim(claim, &mut fabric);
    }


    match part {
        Part::One => {
            let mut result = 0;
            for i in 0..1000 {
                for j in 0..1000 {
                    if fabric[j][i] >= 2 {
                        result += 1;
                    }
                }
            }
            result
        },
        Part::Two => {
            let mut result = 0;
            for claim in claims {
                if check_intact_claim(claim, &fabric) {
                    result = claim.num;
                    break;
                }
            }
            result
        }
    }
}

fn check_intact_claim(claim:Claim, fabric:&Vec<Vec<i32>>) -> bool {
    let mut intact = true;

    'outer: for i in claim.start_top..claim.start_top+claim.height {
        for j in claim.start_left..claim.start_left+claim.width{
            if fabric[j][i] > 1 {
                intact = false;
                break 'outer;
            }
        }
    }

    intact
}

fn apply_claim(claim:Claim, fabric:&mut Vec<Vec<i32>>) {
    for i in claim.start_top..claim.start_top+claim.height {
        for j in claim.start_left..claim.start_left+claim.width{
            fabric[j][i] += 1;
        }
    }
}

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
struct Claim {
    num:i32,
    start_left:usize,
    start_top:usize,
    width:usize,
    height:usize,
}

impl Claim {
    fn new() -> Claim {
        Claim {num:0, start_left:0, start_top:0, width:0, height:0}
    }

    fn from(input:&str) -> Claim {
        let mut cleaned = input
            .replace("#", "")
            .replace("@", "")
            .replace(",", " ")
            .replace(":", "")
            .replace("x", " ");


        let mut tokens:Vec<&str> = cleaned.split_whitespace().collect();
        let mut claim = Claim::new();


        claim.num = tokens[0].parse::<i32>().unwrap();
        claim.start_left = tokens[1].parse::<usize>().unwrap();
        claim.start_top = tokens[2].parse::<usize>().unwrap();
        claim.width = tokens[3].parse::<usize>().unwrap();
        claim.height = tokens[4].parse::<usize>().unwrap();

        claim
    }
}