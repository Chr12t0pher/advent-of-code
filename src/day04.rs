use std::cmp::{max, min, Reverse};
use std::collections::BinaryHeap;
use std::io::BufRead;

pub fn day04() {
    day04a();
    day04b();
}

fn day04a() {
    let mut total_overlaps: u16 = 0;

    let mut powers: [u32; 4] = [1, 1, 1, 1];
    let mut numbers: [u32; 4] = [0, 0, 0, 0];
    let mut curr_index: usize = 0;

    include_bytes!("inputs/day04")
        .iter()
        .for_each(|char| {
            match char {
                b'0'..=b'9' => {
                    let number = &mut numbers[curr_index];
                    let power = &mut powers[curr_index];
                    *number *= *power;
                    *number += (char - b'0') as u32;
                    *power *= 10;
                },
                b'-' | b',' => curr_index += 1,
                10 => { // carriage return, compute + reset
                    if does_contain(numbers) {
                        total_overlaps += 1
                    }

                    for i in 0..4 {
                        powers[i] = 1;
                        numbers[i] = 0;
                    }
                    curr_index = 0;
                },
                _ => panic!()
            }
        });

    if does_contain(numbers) { // last item
        total_overlaps += 1;
    }

    println!("{}", total_overlaps)
}

fn does_contain(numbers: [u32; 4]) -> bool {
    if numbers[0] == numbers[2] || numbers[1] == numbers[3] {
        return true
    } else if numbers[0] >= numbers[2] {
        if numbers[1] <= numbers[3] {
            return true
        }
    } else if numbers[0] <= numbers[2] {
        if numbers[1] >= numbers[3] {
            return true
        }
    }
    false
}

fn day04b() {
    let mut total_overlaps: u16 = 0;

    let mut powers: [u32; 4] = [1, 1, 1, 1];
    let mut numbers: [u32; 4] = [0, 0, 0, 0];
    let mut curr_index: usize = 0;

    include_bytes!("inputs/day04")
        .iter()
        .for_each(|char| {
            match char {
                b'0'..=b'9' => {
                    let number = &mut numbers[curr_index];
                    let power = &mut powers[curr_index];
                    *number *= *power;
                    *number += (char - b'0') as u32;
                    *power *= 10;
                },
                b'-' | b',' => curr_index += 1,
                10 => { // carriage return, compute + reset
                    if does_overlap(numbers) {
                        total_overlaps += 1
                    }

                    for i in 0..4 {
                        powers[i] = 1;
                        numbers[i] = 0;
                    }
                    curr_index = 0;
                },
                _ => panic!()
            }
        });

    if does_overlap(numbers) { // last item
        total_overlaps += 1;
    }

    println!("{}", total_overlaps)
}

fn does_overlap(numbers: [u32; 4]) -> bool {
    (numbers[0] <= numbers[2] && numbers[2] <= numbers[1]) ||
        (numbers[2] <= numbers[0] && (numbers[0] <= numbers[3]))
}

