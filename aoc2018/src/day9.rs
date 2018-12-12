use utils;
use utils::Part;

pub fn solve(part: Part) -> i32 {
    let mut input = String::new();
    utils::read_input_to_string(&mut input, 9).unwrap();

    //for n in 1..50 {
    //    println!("[{}]: {}", n, insert_pos(n));
    //}

    if part == Part::One {
        do_the_thing(input, part)
    }
    else {
        0
    }
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

    'play: loop {
        if next_marble_score > num_marbles { break 'play; }
        let player = match player_cycle.next() {
            Some(id) => id,
            None => break 'play,
        };

        //print!("[{}]    ", player);
        let score = play_the_game(next_marble_score, &mut circle);

        //print!("{:?}", circle);
        //println!();
        scores[player as usize] += score;
        next_marble_score += 1;
    }

    *scores.iter().max().unwrap()
}


fn play_the_game(marble_num:i32, circle:&mut Vec<i32>) -> i32 {
    let mut score_earned = 0;

    if marble_num % 23 == 0 {
        score_earned += marble_num;
        let removed = circle.remove(insert_pos(marble_num));
        score_earned += removed;
    }
    else {
        let pos = insert_pos(marble_num);

        if pos == circle.len()+1{
            circle.push(marble_num)
        }
        else {
            circle.insert(pos, marble_num);
        }
    }

    display_state(marble_num, insert_pos(marble_num), circle);

    score_earned
}

fn display_state(marble_num:i32, pos:usize, circle:&Vec<i32>) {
    print!("[{}]: ", marble_num);
    for i in 0..circle.len() {
        if i == pos {
            print!(" ({}) ", circle[i]);
        }
        else {
            print!(" {} ", circle[i]);
        }
    }
    print!("\n");
}


fn elf_score(id:i32, num_player:i32, max_marble_score:i32, total_players:i32) -> i32 {
    let players_marbles = (0..total_players)
        .cycle()
        .zip(1..max_marble_score+1);

    let scoring_moves = players_marbles
        .filter(|(_, m)| m % 23 == 0);

    0
}

fn insert_pos(marble_num:i32) -> usize {
    let game_offset = marble_num as usize/23 * 9;

    let mut pos = a(marble_num as usize);

    if game_offset > pos {
        pos + len_from_marble_num(marble_num as usize) - game_offset
    }
    else {
        pos - game_offset
    }
}

fn a(n:usize) -> usize {
    if n == 0 { return 0; }
    if n == 1 { return 1; }
    let a_n;
    let r = n%2;
    let n = n/2;
    if r == 1 {
        a_n = 2 * a(n) + 1
    }
    else {
        a_n = 2 * a(n) - 1
    }
    a_n
}

fn len_from_marble_num(marble_num:usize) -> usize {
    marble_num + 1 - ((marble_num/23) * 2)
}