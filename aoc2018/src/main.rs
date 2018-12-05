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
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

fn main() {
    day!(day1, day2, day3, day4, day5);
}
