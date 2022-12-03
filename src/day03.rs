pub fn day03() {
    day03a();
    day03b();
}


fn day03a() {
    let mut items: u64 = 0;
    let mut total: u32 = 0;

    include_str!("inputs/day03")
        .lines()
        .for_each(|line| {
            let bytes = line.as_bytes();
            let len = bytes.len();
            let half = len / 2;

            for i in 0..half {
                items |= 1 << item_value(bytes[i]);
            }

            for i in half..bytes.len() {
                let value = item_value(bytes[i]);
                let has_item = items & 1 << value != 0;
                if has_item {
                    total += value as u32;
                    break;
                }
            }

            items = 0; // reset 'map'
        });

    println!("{}", total);
}

fn day03b() {
    let mut items_found: u64 = u64::MAX;
    let mut total: u32 = 0;

    let mut lines_iter = include_str!("inputs/day03").lines();
    let mut line = lines_iter.next();
    while line.is_some() {
        parse_items(&mut items_found, line.unwrap());
        line = lines_iter.next();
        parse_items(&mut items_found, line.unwrap());
        line = lines_iter.next();
        parse_items(&mut items_found, line.unwrap());

        let mut value = 0;
        while items_found != 1 {
            items_found = items_found >> 1;
            value += 1;
        }

        total += value;

        items_found = u64::MAX;
        line = lines_iter.next();
    }

    println!("{}", total);
}

fn parse_items(items: &mut u64, line: &str) {
    let mut items_curr: u64 = 0;
    for byte in line.as_bytes() {
        items_curr |= 1 << item_value(*byte);
    }
    *items &= items_curr;
}

fn item_value(item: u8) -> u8 {
    if item > b'a' { // lowercase: 1-26
        item - b'a' + 1
    } else { // capital: 27-52
        item - 38
    }
}
