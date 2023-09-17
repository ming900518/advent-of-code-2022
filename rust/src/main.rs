#![warn(clippy::all, clippy::nursery, clippy::pedantic, clippy::perf)]
#![allow(dead_code)]

use days::day1;
use days::day2;
use days::day7;

mod days;

fn main() {
    day1::part1();
    day1::part2();
    day2::part1();
    day2::part2();
    day7::part1();
}
