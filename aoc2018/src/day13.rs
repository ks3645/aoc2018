use utils;
use utils::Part;
use std::collections::HashSet;

pub fn solve(part: Part) -> Point {
    let mut input = String::new();
    utils::read_input_to_string(&mut input, 13).unwrap();

    do_the_thing(input, part)
}

fn do_the_thing(input:String, part:Part) -> Point {
    let width = input.lines().map(|l| l.len()).max().unwrap();
    let height = input.lines().count();

    let mut cart_grid = vec![vec![TrackType::None; width]; height];
    let mut carts:Vec<Cart> = Vec::new();

    let mut horizontal_line_open = false;
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let track_type = match c {
                '+' => TrackType::Intersection,
                '|' => TrackType::Vertical,
                '-' => TrackType::Horizontal,
                '<' => {
                    carts.push( Cart::new(Point {x, y}, Direction::Left));
                    TrackType::Horizontal
                },
                '>' => {
                    carts.push( Cart::new(Point {x, y}, Direction::Right));
                    TrackType::Horizontal
                },
                '^' => {
                    carts.push( Cart::new(Point {x, y}, Direction::Up));
                    TrackType::Vertical
                },
                'v' => {
                    carts.push( Cart::new(Point {x, y}, Direction::Down));
                    TrackType::Vertical
                },
                '/' => {
                    if horizontal_line_open {
                        horizontal_line_open = false;
                        TrackType::BottomRightCorner
                    }
                    else {
                        horizontal_line_open = true;
                        TrackType::TopLeftCorner
                    }
                },
                '\\' => {
                    if horizontal_line_open {
                        horizontal_line_open = false;
                        TrackType::TopRightCorner
                    }
                        else {
                            horizontal_line_open = true;
                            TrackType::BottomLeftCorner
                        }
                }
                _ => TrackType::None,
            };
            cart_grid[y][x] = track_type;
        }
    }

    let collision_pos =
    'tick: loop {
        if carts.len() == 1 {
            break 'tick carts[0].pos;
        }

        let mut pos_map:HashSet<Point> = HashSet::new();
        let mut to_remove:HashSet<usize> = HashSet::new();

        carts.sort();

        for c in carts.clone() {
            pos_map.insert(c.pos);
        }

        for i in 0..carts.len() {
            if to_remove.contains(&i) {
                continue;
            }
            pos_map.remove(&carts[i].pos);
            match carts[i].heading {
                Direction::Left => {
                    carts[i].pos.x -= 1;
                    match cart_grid[carts[i].pos.y][carts[i].pos.x] {
                        TrackType::TopLeftCorner => {
                            carts[i].heading = Direction::Down;
                        },
                        TrackType::BottomLeftCorner => {
                            carts[i].heading = Direction::Up;
                        },
                        TrackType::Intersection => {
                            carts[i].last_turn = next_turn(&carts[i].last_turn);
                            carts[i].heading = combine_local_global_heading(&carts[i].heading, &carts[i].last_turn);
                        },
                        TrackType::Horizontal => { },
                        _ => panic!("cart encountered unexpected TrackType"),
                    }
                },
                Direction::Up => {
                    carts[i].pos.y -= 1;
                    match cart_grid[carts[i].pos.y][carts[i].pos.x] {
                        TrackType::TopLeftCorner => {
                            carts[i].heading = Direction::Right;
                        },
                        TrackType::TopRightCorner => {
                            carts[i].heading = Direction::Left;
                        },
                        TrackType::Intersection => {
                            carts[i].last_turn = next_turn(&carts[i].last_turn);
                            carts[i].heading = combine_local_global_heading(&carts[i].heading, &carts[i].last_turn);
                        },
                        TrackType::Vertical => { },
                        _ => panic!("cart encountered unexpected TrackType"),
                    }
                },
                Direction::Right => {
                    carts[i].pos.x += 1;
                    match cart_grid[carts[i].pos.y][carts[i].pos.x ] {
                        TrackType::TopRightCorner => {
                            carts[i].heading = Direction::Down;
                        },
                        TrackType::BottomRightCorner => {
                            carts[i].heading = Direction::Up;
                        },
                        TrackType::Intersection => {
                            carts[i].last_turn = next_turn(&carts[i].last_turn);
                            carts[i].heading = combine_local_global_heading(&carts[i].heading, &carts[i].last_turn);
                        },
                        TrackType::Horizontal => { },
                        _ => panic!("cart encountered unexpected TrackType"),
                    }
                },
                Direction::Down => {
                    carts[i].pos.y += 1;
                    match cart_grid[carts[i].pos.y][carts[i].pos.x ] {
                        TrackType::BottomRightCorner => {
                            carts[i].heading = Direction::Left;
                        },
                        TrackType::BottomLeftCorner => {
                            carts[i].heading = Direction::Right;
                        },
                        TrackType::Intersection => {
                            carts[i].last_turn = next_turn(&carts[i].last_turn);
                            carts[i].heading = combine_local_global_heading(&carts[i].heading, &carts[i].last_turn);
                        },
                        TrackType::Vertical => { },
                        _ => panic!("cart encountered unexpected TrackType"),
                    }
                }
                _ => panic!("Don't use None as heading"),
            }

            if pos_map.contains(&carts[i].pos) {
               match part {
                   Part::One => break 'tick carts[i].pos,
                   Part::Two => {
                       pos_map.remove(&carts[i].pos);

                       println!("Collision at pos: {}", &carts[i].pos);
                       let carts_to_remove:Vec<usize> = carts
                           .iter().clone().enumerate()
                           .filter_map(|(index, cart)| if cart.pos.x == carts[i].pos.x && cart.pos.y == carts[i].pos.y {Some(index)} else {None})
                           .collect();

                       if carts_to_remove.len() != 2 {
                           panic!("I think there should only be two at a time here");
                       }
                        println!("collision! removing carts at {:?}", carts_to_remove);

                       to_remove.insert(carts_to_remove[0]);
                       to_remove.insert(carts_to_remove[1]);
                   },
               }
            }
            else {
                pos_map.insert(carts[i].pos);
            }
        }

        println!("{:?}", carts);
        if to_remove.len() != 0 {
            println!("carts before remove: {:?}", carts);
            carts = carts.iter().enumerate().filter_map(|(i, c)| if to_remove.contains(&i) { None } else {Some(c.clone())}).collect();
           println!("carts after remove: {:?}", carts);
        }
    };


    collision_pos
}

fn next_turn(last:&Direction) -> Direction{
    match *last {
        Direction::Right => Direction::Left,
        Direction::Left => Direction::None,
        Direction::None => Direction::Right,
        _ => panic!("This last direction shouldn't be possible"),
    }
}

fn combine_local_global_heading(global:&Direction, local:&Direction) -> Direction {
    match *global {
        Direction::Left => match *local {
            Direction::None => Direction::Left,
            Direction::Right => Direction::Up,
            Direction::Left => Direction::Down,
            _ => panic!("This local heading shouldn't be possible"),
        },
        Direction::Up => match *local {
            Direction::None => Direction::Up,
            Direction::Right => Direction::Right,
            Direction::Left => Direction::Left,
            _ => panic!("This local heading shouldn't be possible"),
        },
        Direction::Right => match *local {
            Direction::None => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Left => Direction::Up,
            _ => panic!("This local heading shouldn't be possible"),
        },
        Direction::Down => match *local {
            Direction::None => Direction::Down,
            Direction::Right => Direction::Left,
            Direction::Left => Direction::Right,
            _ => panic!("This local heading shouldn't be possible"),
        },
        _ => panic!("No rule for global heading None"),
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Point {
    x:usize,
    y:usize,
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Ord for Point {
    fn cmp(&self, other:&Point) -> std::cmp::Ordering {
        if self.y < other.y {
            std::cmp::Ordering::Less
        }
        else if self.y > other.y {
            std::cmp::Ordering::Greater
        }
        else {
            if self.x < other.x {
                std::cmp::Ordering::Less
            }
            else if self.x > other.x {
                std::cmp::Ordering::Greater
            }
            else {
                std::cmp::Ordering::Equal
            }
        }
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other:&Point) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}

#[derive(Debug, Copy, Clone)]
enum TrackType {
    TopLeftCorner,
    TopRightCorner,
    BottomLeftCorner,
    BottomRightCorner,
    Vertical,
    Horizontal,
    Intersection,
    None,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    None,
}

#[derive( Eq, PartialEq, Clone, Copy)]
struct Cart {
    pos:Point,
    heading:Direction,
    last_turn:Direction,
}

impl std::fmt::Debug for Cart {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}, {}): {}", self.pos.x, self.pos.y, match self.heading {
            Direction::Left => "<",
            Direction::Up => "^",
            Direction::Right => ">",
            Direction::Down => "v",
            _ => "",
        })
    }
}

impl Ord for Cart {
    fn cmp(&self, other:&Cart) -> std::cmp::Ordering {
        self.pos.cmp(&other.pos)
    }
}

impl PartialOrd for Cart {
    fn partial_cmp(&self, other:&Cart) -> Option<std::cmp::Ordering> {
        self.pos.partial_cmp(&other.pos)
    }
}

impl Cart {
    fn new(pos:Point, heading:Direction) -> Cart {
        Cart{pos, heading, last_turn:Direction::Right}
    }
}