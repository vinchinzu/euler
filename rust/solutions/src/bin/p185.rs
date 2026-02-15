// Project Euler 185: Number Mind
// Hill-climbing / random restart to find 16-digit number matching guess constraints.

const DIGITS: usize = 16;
const NUM_GUESSES: usize = 22;

static GUESS_STRS: [&str; NUM_GUESSES] = [
    "5616185650518293", "3847439647293047", "5855462940810587",
    "9742855507068353", "4296849643607543", "3174248439465858",
    "4513559094146117", "7890971548908067", "8157356344118483",
    "2615250744386899", "8690095851526254", "6375711915077050",
    "6913859173121360", "6442889055042768", "2321386104303845",
    "2326509471271448", "5251583379644322", "1748270476758276",
    "4895722652190306", "3041631117224635", "1841236454324589",
    "2659862637316867",
];

static TARGETS: [i32; NUM_GUESSES] = [
    2, 1, 3, 3, 3, 1, 2, 3, 1, 2, 3, 1, 1, 2, 0, 2, 2, 3, 1, 3, 3, 2,
];

fn main() {
    let mut guesses = [[0u8; DIGITS]; NUM_GUESSES];
    for (gi, s) in GUESS_STRS.iter().enumerate() {
        for (p, c) in s.bytes().enumerate() {
            guesses[gi][p] = c - b'0';
        }
    }

    let mut match_table = [[[false; NUM_GUESSES]; 10]; DIGITS];
    for pos in 0..DIGITS {
        for gi in 0..NUM_GUESSES {
            match_table[pos][guesses[gi][pos] as usize][gi] = true;
        }
    }

    let mut rng_state: u64 = 2024;
    let mut rng_next = || -> u32 {
        rng_state = rng_state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        (rng_state >> 32) as u32
    };

    let mut best_sequence = [0u8; DIGITS];
    let mut best_cost = 999999i32;

    for _attempt in 0..200 {
        let mut sequence = [0u8; DIGITS];
        for i in 0..DIGITS { sequence[i] = (rng_next() % 10) as u8; }

        let mut matches = [0i32; NUM_GUESSES];
        for pos in 0..DIGITS {
            let d = sequence[pos] as usize;
            for gi in 0..NUM_GUESSES {
                if match_table[pos][d][gi] { matches[gi] += 1; }
            }
        }

        let mut contrib = [0i32; NUM_GUESSES];
        let mut current_cost = 0i32;
        for gi in 0..NUM_GUESSES {
            contrib[gi] = (matches[gi] - TARGETS[gi]).abs();
            current_cost += contrib[gi];
        }

        if current_cost < best_cost {
            best_cost = current_cost;
            best_sequence = sequence;
        }

        for _iter in 0..6000 {
            if current_cost == 0 { break; }
            let mut improved = false;

            let mut positions: [usize; DIGITS] = core::array::from_fn(|i| i);
            for i in (1..DIGITS).rev() {
                let j = rng_next() as usize % (i + 1);
                positions.swap(i, j);
            }

            for &pos in &positions {
                if current_cost == 0 { break; }
                let orig_digit = sequence[pos] as usize;
                let mut best_digit = orig_digit;
                let mut best_delta = 0i32;

                for td in 0..10usize {
                    if td == orig_digit { continue; }
                    let mut delta_cost = 0i32;
                    for gi in 0..NUM_GUESSES {
                        let dm = match_table[pos][td][gi] as i32 - match_table[pos][orig_digit][gi] as i32;
                        if dm == 0 { continue; }
                        let new_match = matches[gi] + dm;
                        let new_contrib = (new_match - TARGETS[gi]).abs();
                        delta_cost += new_contrib - contrib[gi];
                    }
                    if delta_cost < best_delta {
                        best_delta = delta_cost;
                        best_digit = td;
                    }
                }

                if best_digit == orig_digit { continue; }

                for gi in 0..NUM_GUESSES {
                    matches[gi] += match_table[pos][best_digit][gi] as i32 - match_table[pos][orig_digit][gi] as i32;
                    contrib[gi] = (matches[gi] - TARGETS[gi]).abs();
                }
                sequence[pos] = best_digit as u8;
                current_cost += best_delta;
                improved = true;

                if current_cost < best_cost {
                    best_cost = current_cost;
                    best_sequence = sequence;
                }
            }

            if !improved {
                for _ in 0..2 {
                    let pos = rng_next() as usize % DIGITS;
                    let nd = rng_next() as usize % 10;
                    if nd == sequence[pos] as usize { continue; }
                    let od = sequence[pos] as usize;
                    for gi in 0..NUM_GUESSES {
                        matches[gi] += match_table[pos][nd][gi] as i32 - match_table[pos][od][gi] as i32;
                        contrib[gi] = (matches[gi] - TARGETS[gi]).abs();
                    }
                    sequence[pos] = nd as u8;
                }
                current_cost = contrib.iter().sum();
                if current_cost < best_cost {
                    best_cost = current_cost;
                    best_sequence = sequence;
                }
            }
        }

        if current_cost == 0 {
            best_sequence = sequence;
            break;
        }
    }

    let s: String = best_sequence.iter().map(|&d| (b'0' + d) as char).collect();
    println!("{}", s);
}
