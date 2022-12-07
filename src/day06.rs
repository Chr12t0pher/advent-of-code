
pub fn day06() {
    day06a();
    day06b();
}

fn day06a() {
    println!("{}", find_n_unique_chars(include_bytes!("inputs/day06"), 4));
}

fn day06b() {
    println!("{}", find_n_unique_chars(include_bytes!("inputs/day06"), 14));
}

fn find_n_unique_chars(bytes: &[u8], n: u8) -> u32 {
    let mut counts: [u8; 26] = [0; 26];
    let mut index: usize;
    let mut collisions: u8 = 0;

    for i in 0..(n as usize) {
        index = index_for(bytes[i]);
        counts[index] += 1;
        if counts[index] == 2 {
            collisions += 1;
        }
    }

    for i in (n as usize)..bytes.len() {
        index = index_for(bytes[i - n as usize]);
        counts[index] -= 1;
        if counts[index] == 1 {
            collisions -= 1;
        }

        index = index_for(bytes[i]);
        counts[index] += 1;
        if counts[index] == 2 {
            collisions += 1
        }

        if collisions == 0 {
            return i as u32 + 1;
        }
    }

    panic!()
}

fn index_for(byte: u8) -> usize {
    usize::from(byte - b'a')
}