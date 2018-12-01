macro_rules! days {
    ( $( $x:ident ),* ) => {
        $(
            mod $x;
        )*
        fn print_all_solutions() {
            $(
                  println!("{}: Part One Solution: {}", stringify!($x), $x::solve(Part::PartOne));
                  println!("{}: Part Two Solution: {}", stringify!($x), $x::solve(Part::PartTwo));
            )*
        }
    };
}

mod utils;
use utils::Part;
days!(day1);


fn main() {
    print_all_solutions();
}
