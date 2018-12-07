use utils;
use utils::Part;
use std::collections::HashMap;

pub fn solve(part: Part) -> i32 {
    let mut input = String::new();
    utils::read_input_to_string(&mut input, 6).unwrap();

    match part {
        Part::One => part_one(input),
        Part::Two => part_two(input),
    }

}

fn part_two(input:String) -> i32 {
    let points:Vec<(usize, Point)> = (1..).zip(input
        .lines()
        .map(|line| Point::from(line)))
        .collect();

    let start_x = points.iter().min_by_key(|(_, p)| p.x).unwrap().1.x;
    let start_y = points.iter().min_by_key(|(_, p)| p.y).unwrap().1.y;
    let end_x = points.iter().max_by_key(|(_, p)| p.x).unwrap().1.x;
    let end_y = points.iter().max_by_key(|(_, p)| p.x).unwrap().1.y;

    let width = end_x - start_x;
    let height = end_y - start_y;

    let mut points_map:HashMap<Point, i32> = HashMap::new();

    for x in start_x-(width*2)..end_x+(width*2) {
        for y in start_y-(height*2)..end_y+(height*2) {
            let pt = Point {x, y};

            let mut dists:Vec<(usize, i32)> =  points
                .iter()
                .map(|(id, p)| (*id, p.dist_to(&pt)))
                .collect();


            points_map.insert(pt, dists.iter().map(|&(_, d)| d).sum());
        }
        //println!();
    }

    points_map
        .iter()
        .filter_map(|(&p, &total)| if total < 10000 {Some((p, total))} else {None})
        .count() as i32
}

fn part_one(input:String) -> i32 {
    let points:Vec<(usize, Point)> = (1..).zip(input
        .lines()
        .map(|line| Point::from(line)))
        .collect();

    let start_x = points.iter().min_by_key(|(_, p)| p.x).unwrap().1.x;
    let start_y = points.iter().min_by_key(|(_, p)| p.y).unwrap().1.y;
    let end_x = points.iter().max_by_key(|(_, p)| p.x).unwrap().1.x;
    let end_y = points.iter().max_by_key(|(_, p)| p.x).unwrap().1.y;

    let width = end_x - start_x;
    let height = end_y - start_y;

    let mut points_map = HashMap::new();

    let mut second_points_map = HashMap::new();

    for x in start_x-(width*2)..end_x+(width*2) {
        for y in start_y-(height*2)..end_y+(height*2) {
            let pt = Point {x, y};

            let mut dists:Vec<(usize, i32)> =  points
                .iter()
                .map(|(id, p)| (*id, p.dist_to(&pt)))
                .collect();

            dists.sort_by_key(|(_, d)| d.abs());

            //println!("dist: {:?}", dists);
            //let unique_dists:HashSet<i32> = HashSet::from_iter(dists.iter().map(|(_, d)|*d));
            //println!("{:?}", unique_dists);

            if dists[0].1 == dists[1].1 {
                //print!(".  ");
               // points_map.insert(pt, -1);
            }
            else {
                //print!("{:2} ", dists[0].0 as i32);
                points_map.insert(pt, dists[0].0 as i32);
            }
        }
        //println!();
    }

    for x in start_x-(width*3)..end_x+(width*3) {
        for y in start_y-(height*3)..end_y+(height*3) {
            let pt = Point {x, y};

            let mut dists:Vec<(usize, i32)> =  points
                .iter()
                .map(|(id, p)| (*id, p.dist_to(&pt)))
                .collect();

            dists.sort_by_key(|(_, d)| d.abs());

            //println!("dist: {:?}", dists);
            //let unique_dists:HashSet<i32> = HashSet::from_iter(dists.iter().map(|(_, d)|*d));
            //println!("{:?}", unique_dists);

            if dists[0].1 == dists[1].1 {
                //print!(".  ");
                // points_map.insert(pt, -1);
            }
                else {
                    //print!("{:2} ", dists[0].0 as i32);
                    second_points_map.insert(pt, dists[0].0 as i32);
                }
        }
        //println!();
    }

    //println!("{:?}", points_map);

    let one:Vec<(&usize, usize)> = points
        .iter()
        .filter(|(_, p)| p.x != start_x && p.x != end_x && p.y != start_y && p.y != end_y )
        .map(|(id, _)| (id, points_map.iter().filter(|(_, &v)| (v != -1) && v == (*id as i32)).count()))
        .collect();

    let two:Vec<(&usize, usize)>  = points
        .iter()
        .filter(|(_, p)| p.x != start_x && p.x != end_x && p.y != start_y && p.y != end_y )
        .map(|(id, _)| (id, second_points_map.iter().filter(|(_, &v)| (v != -1) && v == (*id as i32)).count()))
        .collect();

    one.iter().zip(two.iter())
        .filter(|(a,b)| a.1 == b.1)
        .map(|(a, _)| *a)
        .max_by_key(|(_, count)| *count).unwrap().1 as i32
}


#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Point {
    x:i32,
    y:i32,
}

impl Point {
    fn from(input:&str) -> Point {
        let tokens:Vec<&str> = input.trim().split(",").map(|s| s.trim()).collect();

        Point{ x:tokens[0].parse().unwrap(), y:tokens[1].parse().unwrap()}
    }

    fn dist_to(&self, other:&Point) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}