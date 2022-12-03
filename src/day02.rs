pub fn day02() {
    day02a();
    day02b();
}

const WIN: u32 = 6;
const DRAW: u32 = 3;

fn day02a() {
    let mut score: u32 = 0;

    include_str!("inputs/day02")
        .lines()
        .for_each(|line| {
            let chars = line.as_bytes();
            let opp = chars[0] - 65; // A = 0, B = 1, C = 2
            let me = chars[2] - 65 - 23; // X = 0, Y = 1, C = 2

            score += 1 + me as u32;

            if opp == me {
                score += DRAW
            } else if index_to_win_against(opp) == me {
                score += WIN
            }
        });

    println!("{}", score);
}

fn day02b() {
    let mut score: u32 = 0;

    include_str!("inputs/day02")
        .lines()
        .for_each(|line| {
            let chars = line.as_bytes();
            let opp = chars[0] - 65; // A = 0, B = 1, C = 2
            let outcome_char = chars[2];

            match outcome_char {
                b'X' => { // LOSS
                    score += index_to_lose_against(opp) as u32 + 1;
                },
                b'Y' => { // DRAW
                    score += DRAW;
                    score += opp as u32 + 1;
                },
                b'Z' => { // WIN
                    score += WIN;
                    score += index_to_win_against(opp) as u32 + 1;
                },
                _ => panic!()
            };
        });

    println!("{}", score);
}

fn index_to_win_against(opp_index: u8) -> u8 {
    (opp_index + 1) % 3
}

fn index_to_lose_against(opp_index: u8) -> u8 {
    (opp_index + 2) % 3
}