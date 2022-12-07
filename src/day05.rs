use std::collections::{VecDeque};

pub fn day05() {
    day05a();
    day05b();
}

fn day05a() {
    let mut input = include_bytes!("inputs/day05").iter();
    let mut char: &u8;

    let mut stacks: Vec<VecDeque<&u8>> = Vec::new();
    let mut stack_index: usize;

    'setup: loop { // load initial stacks
        stack_index = 0;
        'row: loop { // parse row
            char = input.nth(1).unwrap(); // skip first char, it's either empty or a bracket
            match char {
                b'A'..=b'Z' => {
                    if stacks.len() <= stack_index {
                        stacks.push(VecDeque::new());
                    }
                    stacks[stack_index].push_front(char);
                }
                b' ' => {
                    if stacks.len() <= stack_index {
                        stacks.push(VecDeque::new());
                    }
                }, // empty, do nothing
                b'0'..=b'9' => { // finished loading
                    input.nth(stacks.len() * 4 - 4); // skip the number row
                    break 'setup;
                }
                _ => panic!()
            }

            char = input.nth(1).unwrap(); // either the cl, or a line break
            if *char == 10 {
                break 'row;
            }

            stack_index += 1
        }
    }

    input.next();

    let mut qty: u8;
    let mut from: usize;
    let mut to: usize;
    loop {
        char = input.nth(5).unwrap();
        qty = char - b'0';

        char = input.next().unwrap();
        if *char != b' ' { // if grabbing a 2 digit number
            qty *= 10;
            qty += char - b'0';
            input.next(); // skip the space, so we're back in line with if it wasn't a 2 digit number
        }

        char = input.nth(5).unwrap();
        from = usize::from(char - b'0' - 1);

        char = input.nth(4).unwrap();
        to = usize::from(char - b'0' - 1);

        for _ in 0..qty {
            match stacks[from].pop_back() {
                Some(item) => stacks[to].push_back(item),
                None => break
            }
        }

        if input.next().is_none() {
            break;
        }
    }

    for i in 0..stacks.len() {
        let option = stacks[i].pop_back();
        match option {
            Some(x) => print!("{}", *x as char),
            None => print!(" ")
        }
    }

    println!();
}

fn day05b() {
    let mut input = include_bytes!("inputs/day05").iter();
    let mut char: &u8;

    let mut stacks: Vec<VecDeque<&u8>> = Vec::new();
    let mut stack_index: usize;

    'setup: loop { // load initial stacks
        stack_index = 0;
        'row: loop { // parse row
            char = input.nth(1).unwrap(); // skip first char, it's either empty or a bracket
            match char {
                b'A'..=b'Z' => {
                    if stacks.len() <= stack_index {
                        stacks.push(VecDeque::new());
                    }
                    stacks[stack_index].push_front(char);
                }
                b' ' => {
                    if stacks.len() <= stack_index {
                        stacks.push(VecDeque::new());
                    }
                }, // empty, do nothing
                b'0'..=b'9' => { // finished loading
                    input.nth(stacks.len() * 4 - 4); // skip the number row
                    break 'setup;
                }
                _ => panic!()
            }

            char = input.nth(1).unwrap(); // either the cl, or a line break
            if *char == 10 {
                break 'row;
            }

            stack_index += 1
        }
    }

    input.next();

    let mut grabber: VecDeque<&u8> = VecDeque::new();
    let mut qty: u8;
    let mut from: usize;
    let mut to: usize;
    loop {
        char = input.nth(5).unwrap();
        qty = char - b'0';

        char = input.next().unwrap();
        if *char != b' ' { // if grabbing a 2 digit number
            qty *= 10;
            qty += char - b'0';
            input.next(); // skip the space, so we're back in line with if it wasn't a 2 digit number
        }

        char = input.nth(5).unwrap();
        from = usize::from(char - b'0' - 1);

        char = input.nth(4).unwrap();
        to = usize::from(char - b'0' - 1);

        for _ in 0..qty {
            match stacks[from].pop_back() {
                Some(item) => grabber.push_front(item),
                None => break
            }
        }

        while !grabber.is_empty() {
            stacks[to].push_back(grabber.pop_front().unwrap());
        }

        if input.next().is_none() {
            break;
        }
    }

    for i in 0..stacks.len() {
        let option = stacks[i].pop_back();
        match option {
            Some(x) => print!("{}", *x as char),
            None => print!(" ")
        }
    }

    println!();
}

