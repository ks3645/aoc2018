macro_rules! day {
    ( $( $x:ident ),* ) => {
        $(
            println!("{}: Part One Solution: {}", stringify!($x), $x::solve(Part::One));
            println!("{}: Part Two Solution: {}", stringify!($x), $x::solve(Part::Two));
        )*
    };
}

mod utils;
use utils::Part;
#[allow(dead_code)]
mod day1;
#[allow(dead_code)]
mod day2;
#[allow(dead_code)]
mod day3;
#[allow(dead_code)]
mod day4;
#[allow(dead_code)]
mod day5;
#[allow(dead_code)]
mod day6;
#[allow(dead_code)]
mod day7;
#[allow(dead_code)]
mod day8;
#[allow(dead_code)]
mod day9;
#[allow(dead_code)]
mod day10;

fn main() {
    day!(/*day1, day2, day3, day4, day5, day6, day7, day8,*/ day9/*, day10*/);
}
