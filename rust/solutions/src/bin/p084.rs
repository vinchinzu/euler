// Project Euler 84: Monopoly odds
// Monte Carlo simulation with 4-sided dice.

const BOARD_SIZE: usize = 40;
const DICE_SIDES: u64 = 4;
const SIMULATIONS: u64 = 10_000_000;

const GO: usize = 0;
const JAIL: usize = 10;
const G2J: usize = 30;
const CC1: usize = 2;
const CC2: usize = 17;
const CC3: usize = 33;
const CH1: usize = 7;
const CH2: usize = 22;
const CH3: usize = 36;
const C1: usize = 11;
const E3: usize = 24;
const H2: usize = 39;
const R1: usize = 5;
const R2: usize = 15;
const R3: usize = 25;
const R4: usize = 35;
const U1: usize = 12;
const U2: usize = 28;

struct Rng(u64);

impl Rng {
    fn next(&mut self, lo: u64, hi: u64) -> u64 {
        self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        lo + ((self.0 >> 33) % (hi - lo + 1))
    }
}

fn next_railway(pos: usize) -> usize {
    if pos < R1 { R1 }
    else if pos < R2 { R2 }
    else if pos < R3 { R3 }
    else if pos < R4 { R4 }
    else { R1 }
}

fn next_utility(pos: usize) -> usize {
    if pos < U1 || pos >= U2 { U1 } else { U2 }
}

fn process_cc(pos: usize, rng: &mut Rng) -> usize {
    let card = rng.next(0, 15);
    match card {
        0 => GO,
        1 => JAIL,
        _ => pos,
    }
}

fn process_ch(pos: usize, rng: &mut Rng) -> usize {
    let card = rng.next(0, 15);
    match card {
        0 => GO,
        1 => JAIL,
        2 => C1,
        3 => E3,
        4 => H2,
        5 => R1,
        6 | 7 => next_railway(pos),
        8 => next_utility(pos),
        9 => (pos + BOARD_SIZE - 3) % BOARD_SIZE,
        _ => pos,
    }
}

fn main() {
    let mut visits = [0u64; BOARD_SIZE];
    let mut rng = Rng(12345);
    let mut pos = 0usize;
    let mut doubles_count = 0u32;

    for _ in 0..SIMULATIONS {
        let d1 = rng.next(1, DICE_SIDES);
        let d2 = rng.next(1, DICE_SIDES);

        if d1 == d2 {
            doubles_count += 1;
            if doubles_count == 3 {
                pos = JAIL;
                doubles_count = 0;
                visits[pos] += 1;
                continue;
            }
        } else {
            doubles_count = 0;
        }

        pos = (pos + d1 as usize + d2 as usize) % BOARD_SIZE;

        if pos == G2J {
            pos = JAIL;
        } else if pos == CC1 || pos == CC2 || pos == CC3 {
            pos = process_cc(pos, &mut rng);
        } else if pos == CH1 || pos == CH2 || pos == CH3 {
            pos = process_ch(pos, &mut rng);
            if pos == CC1 || pos == CC2 || pos == CC3 {
                pos = process_cc(pos, &mut rng);
            }
        }

        visits[pos] += 1;
    }

    // Find top 3 squares
    let mut indices: Vec<usize> = (0..BOARD_SIZE).collect();
    indices.sort_by(|&a, &b| visits[b].cmp(&visits[a]));

    println!("{:02}{:02}{:02}", indices[0], indices[1], indices[2]);
}
