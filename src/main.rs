mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
// mod day16;
mod day17;

macro_rules! problems {
    ()=>{};
    ($day:tt $($parts:ident)+, $($rest:tt)*) => {
        problems!($day $($parts)+);
        problems!($($rest)*);
    };
    ($day:tt $part:tt $($rest:tt)+) => {
        problems!($day $part);
        problems!($day $($rest)+);
    };
    ($day:tt $part:tt)=>{
        println!(
            "{} {}\t{:?}",
            stringify!($day),
            stringify!($part),
            $day::$part(include_str!(concat!("../assets/",stringify!($day),".txt")))
        );
    };
}

fn main() {
    problems!(
        day1 part1 part2,
        day2 part1 part2,
        day3 part1 part2,
        day4 part1 part2,
        day5 part1 part2,
        day6 part1 part2,
        day7 part1 part2,
        day8 part1 part2,
        day9 part1 part2,
        day10 part1 part2,
        day11 part1 part2,
        day12 part1 part2,
        day13 part1 part2,
        day14 part1 part2,
        day15 part1 part2,
        // day16 part1
        day17 part1
    );
}
