// Project Euler 84: Monopoly odds
// Stationary distribution with 4-sided dice (40 squares × 3 consecutive-doubles).

const BOARD: usize = 40;
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
const NSTATE: usize = 120; // pos + 40 * consecutive_doubles (0..=2)

fn next_railway(pos: usize) -> usize {
    if pos < R1 {
        R1
    } else if pos < R2 {
        R2
    } else if pos < R3 {
        R3
    } else if pos < R4 {
        R4
    } else {
        R1
    }
}

fn next_utility(pos: usize) -> usize {
    if pos < U1 || pos >= U2 {
        U1
    } else {
        U2
    }
}

fn is_cc(pos: usize) -> bool {
    pos == CC1 || pos == CC2 || pos == CC3
}

fn is_ch(pos: usize) -> bool {
    pos == CH1 || pos == CH2 || pos == CH3
}

fn add(dest: &mut [f64; NSTATE], pos: usize, doubles: usize, p: f64) {
    dest[pos + BOARD * doubles] += p;
}

fn apply_land(pos: usize, doubles: usize, p: f64, dest: &mut [f64; NSTATE]) {
    if pos == G2J {
        add(dest, JAIL, doubles, p);
        return;
    }
    if is_cc(pos) {
        add(dest, GO, doubles, p / 16.0);
        add(dest, JAIL, doubles, p / 16.0);
        add(dest, pos, doubles, p * 14.0 / 16.0);
        return;
    }
    if is_ch(pos) {
        add(dest, pos, doubles, p * 6.0 / 16.0);
        add(dest, GO, doubles, p / 16.0);
        add(dest, JAIL, doubles, p / 16.0);
        add(dest, C1, doubles, p / 16.0);
        add(dest, E3, doubles, p / 16.0);
        add(dest, H2, doubles, p / 16.0);
        add(dest, R1, doubles, p / 16.0);
        add(dest, next_railway(pos), doubles, p * 2.0 / 16.0);
        add(dest, next_utility(pos), doubles, p / 16.0);
        apply_land((pos + BOARD - 3) % BOARD, doubles, p / 16.0, dest);
        return;
    }
    add(dest, pos, doubles, p);
}

fn main() {
    let mut cur = [0.0f64; NSTATE];
    cur[GO] = 1.0;

    for _ in 0..200 {
        let mut nxt = [0.0f64; NSTATE];
        for s in 0..NSTATE {
            let p0 = cur[s];
            if p0 == 0.0 {
                continue;
            }
            let pos = s % BOARD;
            let dbl = s / BOARD;
            for d1 in 1..=4usize {
                for d2 in 1..=4usize {
                    let p = p0 / 16.0;
                    if d1 == d2 {
                        if dbl == 2 {
                            add(&mut nxt, JAIL, 0, p);
                        } else {
                            apply_land((pos + d1 + d2) % BOARD, dbl + 1, p, &mut nxt);
                        }
                    } else {
                        apply_land((pos + d1 + d2) % BOARD, 0, p, &mut nxt);
                    }
                }
            }
        }
        cur = nxt;
    }

    let mut visits = [0.0f64; BOARD];
    for s in 0..NSTATE {
        visits[s % BOARD] += cur[s];
    }

    let mut idx = [0usize; BOARD];
    for i in 0..BOARD {
        idx[i] = i;
    }
    idx.sort_by(|&a, &b| visits[b].partial_cmp(&visits[a]).unwrap());
    println!("{:02}{:02}{:02}", idx[0], idx[1], idx[2]);
}
