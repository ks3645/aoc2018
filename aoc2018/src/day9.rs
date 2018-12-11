use utils;
use utils::Part;

pub fn solve(part: Part) -> i32 {
    let mut input = String::new();
    utils::read_input_to_string(&mut input, 9).unwrap();

    do_the_thing(input, part)
}

fn do_the_thing(input:String, part:Part) -> i32 {
    let tokens:Vec<&str> = input.split_whitespace().collect();

    let num_players:i32 = tokens[0].parse().unwrap();
    let num_marbles:i32 = match part {
        Part::One => tokens[6].parse().unwrap(),
        Part::Two => tokens[6].parse::<i32>().unwrap() * 100,
    };

    let mut player_cycle = (0..num_players).cycle();
    let mut circle = vec![0];

    let mut next_marble_score = 1;

    let mut scores = vec![0; num_players as usize];

    let mut current_marble = 0;

    'play: loop {
        if next_marble_score > num_marbles { break 'play; }
        let player = match player_cycle.next() {
            Some(id) => id,
            None => break 'play,
        };

        //print!("[{}]    ", player);
        let score = play_the_game(next_marble_score, &mut circle, &mut current_marble);

        //print!("{:?}", circle);
        //println!();
        scores[player as usize] += score;
        next_marble_score += 1;
    }

    *scores.iter().max().unwrap()
}

fn play_the_game(marble_num:i32, circle:&mut Vec<i32>, current_marble:&mut i32) -> i32 {
    let mut score_earned = 0;

    if marble_num % 23 == 0 {
        score_earned += marble_num;
        *current_marble -= 7;
        while current_marble < &mut 0 { *current_marble += circle.len() as i32;}
        let removed = circle.remove(insert_pos(marble_num) as usize);
        score_earned += removed;

        //println!("score earned: {} + {}", marble_num, removed);
    }
    else {
        let pos = insert_pos(marble_num);

        if pos == (circle.len()+1) as i32{
            circle.push(marble_num)
        }
        else {
            circle.insert(pos as usize, marble_num);
        }
    }

    score_earned
}

fn elf_score(id:i32, num_player:i32, max_marble_score:i32, total_players:i32) -> i32 {
    let players_marbles = (0..total_players)
        .cycle()
        .zip(1..max_marble_score+1);

    let scoring_moves = players_marbles
        .filter(|(_, m)| m % 23 == 0);

    0
}

fn insert_pos_from_marble_num (marble_num:i32) -> i32 {
    if marble_num == 0 { return 0; }
    if marble_num == 1 { return 1; }
    if marble_num % 2 == 0 {
        (2 * insert_pos_from_marble_num(marble_num / 2) - 1)
    }
    else {
        (2 * insert_pos_from_marble_num(marble_num / 2) + 1)
    }
}

fn insert_pos(marble_num:i32) -> i32 {
    if marble_num == 1 { return 1; }
    let mut pos = match marble_num % 23 == 0 {
        true => insert_pos(marble_num - 1) - 7,
        false => insert_pos(marble_num - 1) + 2,
    };
    let len = len_from_marble_num(marble_num - 1);
    let result = match marble_num % 23 == 0 {
        true => {
            pos += len;
            if pos == len {
                pos
            } else {
                pos % len
            }
        }
        false => {
            if pos == len {
                pos
            } else {
                pos % len
            }
        }
    };
    result
}

fn insert_pos_with_offset(marble_num:i32) -> i32 {
    let mut offset = insert_pos_from_marble_num(marble_num) - (9 * (marble_num / 23));

    println!("len before: {}", offset);
    let len = 9;//len_from_marble_num(marble_num - 1);
    while offset < 0 {
        offset += len;
    }
    println!("len after: {}", offset);

    offset
}

fn len_from_marble_num(marble_num:i32) -> i32 {
    marble_num + 1 - ((marble_num/23) * 2)
}