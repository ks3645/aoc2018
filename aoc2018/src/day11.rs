use utils;
use utils::Part;

pub fn solve(part: Part) -> String {
    let mut input = String::new();
    utils::read_input_to_string(&mut input, 11).unwrap();

    let input = input.parse::<i32>().unwrap();

    format!("{:?}", do_the_thing(input, part))
}

fn do_the_thing(input:i32, part:Part) -> Point {
    let mut grid:[[i32; 301];301] = [[0; 301]; 301];

    for i in 1..301 {
        for j in 1..301 {
            let p = Point {x:i, y:j};
            let mut power_level = power_level(&p) as i32;
            power_level += input;
            power_level *= rack_id(&p) as i32;
            power_level = power_level - (power_level % 100);
            power_level %= 1000;
            power_level /= 100;
            power_level -= 5;
            grid[j][i] = power_level;
        }
    }

    match part {
        Part::One => {
            let mut result = Point{ x:0, y:0};
            let mut max = i32::min_value();
            for i in 1..301-2 {
                for j in 1..301-2 {
                    let p = Point{x:i, y:j};
                    let sum = get_square_power(&p, &grid, 3);
                    if sum > max {
                        max = sum;
                        result = p
                    }
                }
            }
            result
        },
        Part::Two => {
            let mut result = Point{ x:0, y:0 };
            let mut max = i32::min_value();
            let mut max_size = 0;

            for size in 1..301 {
                for i in 1..301-(size-1) {
                    for j in 1..301-(size-1) {
                        let p = Point{x:i, y:j};
                        let sum = get_square_power(&p, &grid, size);
                        if sum > max {
                            max = sum;
                            result = p;
                            max_size = size;
                        }
                    }
                }
            }
            println!("max_size: {}", max_size);

            result
        }
    }
}

fn get_square_power(p:&Point, grid:&[[i32;301];301], size:usize) -> i32 {
    let mut sum = 0;
    for i in p.x..p.x+size {
        for j in p.y..p.y+size {
            sum += grid[j][i];
        }
    }
    sum
}

#[derive(Debug)]
struct Point {
    x:usize,
    y:usize,
}

fn rack_id(point:&Point) -> usize {
    point.x + 10
}

fn power_level(point:&Point) -> usize {
    rack_id(point) * point.y
}
