use utils;
use utils::Part;

pub fn solve(part: Part) -> i32 {
    let mut input = String::new();
    utils::read_input_to_string(&mut input, 10).unwrap();

    do_the_thing(input, part)
}

fn do_the_thing(input:String, _part:Part) -> i32 {

    let mut particles:Vec<Particle> = input.lines().map(|line| Particle::from(line)).collect();

    for i in 0..100_000 {
        display_particles(&particles, i);
        for p in particles.iter_mut() {
            p.time_step();
        }
    }
    0
}

fn display_particles(particles:&Vec<Particle>, time:i32) {
    let min_x = particles.iter().min_by_key(|p| p.pos_x).unwrap().pos_x;
    let min_y = particles.iter().min_by_key(|p| p.pos_y).unwrap().pos_y;
    let max_x = particles.iter().max_by_key(|p| p.pos_x).unwrap().pos_x;
    let max_y = particles.iter().max_by_key(|p| p.pos_y).unwrap().pos_y;

    let width:usize = (max_x + 1 - min_x) as usize;
    let height:usize = (max_y + 1 - min_y) as usize;

    if height > 10 { return; }

    println!("time: {}", time);
    let mut grid = Vec::with_capacity(height);

    for _ in min_y..max_y+1 {
        let mut row = Vec::with_capacity(width);
        for _ in min_x..max_x+1 {
            row.push(".");
        }
        grid.push(row);
    }

    for p in particles {
        let adjusted_x = p.pos_x - min_x;
        let adjusted_y = p.pos_y - min_y;

        grid[adjusted_y as usize][adjusted_x as usize] = "#";
    }

    for y in 0..(max_y+1-min_y) as usize {
        println!("{:?}", grid[y]);
    }
}

#[derive (Debug, Copy, Clone)]
struct Particle {
    pos_x:i32,
    pos_y:i32,
    vel_x:i32,
    vel_y:i32,
}

impl Particle {
    fn time_step(&mut self) {
        self.pos_x += self.vel_x;
        self.pos_y += self.vel_y;
    }

    fn from(input:&str) -> Particle {
        let cleaned = input
            .replace(">", "")
            .replace(",", "")
            .replace("<", " ");

        let tokens:Vec<&str> = cleaned
            .split_whitespace()
            .collect();

        Particle { pos_x:tokens[1].parse().unwrap(), pos_y:tokens[2].parse().unwrap(),
                    vel_x:tokens[4].parse().unwrap(), vel_y:tokens[5].parse().unwrap()}
    }
}

