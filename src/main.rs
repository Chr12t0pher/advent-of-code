extern crate core;

use crate::day01::day01;
use crate::day02::day02;
use crate::day03::day03;
use crate::day04::day04;
use crate::day05::day05;
use crate::day06::day06;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;

fn main() {
    day06()
}

#[allow(dead_code)]
fn previous() {
    day01();
    day02();
    day03();
    day04();
    day05();
}
