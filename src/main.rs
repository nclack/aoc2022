mod day1;
mod day2;
mod day3;

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
        day3 part1 part2
    );
}
