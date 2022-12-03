use std::cmp::{max, Reverse};
use std::collections::BinaryHeap;

pub fn day01() {
    day01a();
    day01b();
}

fn day01a() {
    let mut curr_elf: u32 = 0;
    let mut max_elf: u32 = 0;

    include_str!("inputs/day01")
        .lines()
        .for_each(|x| if x.is_empty() {
            max_elf = max(max_elf, curr_elf);
            curr_elf = 0;
        } else {
            curr_elf += x.parse::<u32>().unwrap();
        });

    println!("{}", max_elf)
}

fn day01b() {
    let mut curr_elf: u32 = 0;
    let mut max_elves: BinaryHeap<Reverse<u32>> = BinaryHeap::with_capacity(3);

    include_str!("inputs/day01")
        .lines()
        .for_each(|x| if x.is_empty() {
            if max_elves.len() == 0 || curr_elf > max_elves.peek().unwrap().0 {
                if max_elves.len() == 3 {
                    max_elves.pop();
                }
                max_elves.push(Reverse(curr_elf));
            }
            curr_elf = 0;
        } else {
            curr_elf += x.parse::<u32>().unwrap();
        });

    let mut max_sum: u32 = 0;
    max_elves.iter().for_each(|x| max_sum += x.0);
    println!("{}", max_sum);
}