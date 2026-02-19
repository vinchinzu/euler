// Project Euler 870 - Stone Game IV
//
// Two players play a stone removal game with parameter r > 0.
// L(r) = set of pile sizes where second player wins.
// Transition values are where L(r) changes.
// Find T(123456) to 10 decimal places.
//
// Algorithm:
// The losing positions form a sequence P_1=1, P_2, P_3, ... satisfying
// the recurrence P_{k+1} = P_k + P_{m(k)} where
// m(k) = min{j : floor(r * P_j) >= P_k}.
//
// Transition values occur at ratios P_a / P_b from the current losing sequence.
// The next transition after r is: min{P_k / P_{m(k)-1} : m(k) >= 2, P_k/P_{m(k)-1} > r}
// where the minimum is over all steps k in the recurrence.
//
// We use exact rational arithmetic (u128 for intermediate products).
// The sequence runs until u64 overflow (no early termination).

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

fn solve() {
    let limit: usize = 123456;
    let mut rn: u64 = 1; // r = rn / rd
    let mut rd: u64 = 1;

    let mut p: Vec<u64> = Vec::with_capacity(16384);

    for _ti in 1..limit {
        // Compute losing sequence for r = rn/rd
        // p[0] = dummy, p[1] = 1, ...
        p.clear();
        p.push(0);
        p.push(1);
        let mut j_lo = 1usize;

        let mut best_n: u64 = 0;
        let mut best_d: u64 = 0;
        let mut best_set = false;

        loop {
            let k = p.len() - 1;
            let pk = p[k];

            // Find m = min{j >= j_lo : rn * p[j] >= pk * rd}
            let target = pk as u128 * rd as u128;

            let mut m = j_lo;
            while m <= k {
                if rn as u128 * p[m] as u128 >= target {
                    break;
                }
                m += 1;
            }
            if m > k { break; }

            // Check transition candidate: pk / p[m-1]
            if m >= 2 {
                let cand_d = p[m - 1];
                // Check pk / cand_d > rn / rd
                if (pk as u128) * (rd as u128) > (rn as u128) * (cand_d as u128) {
                    if !best_set || (pk as u128) * (best_d as u128) < (best_n as u128) * (cand_d as u128) {
                        let g = gcd(pk, cand_d);
                        best_n = pk / g;
                        best_d = cand_d / g;
                        best_set = true;
                    }
                }
            }

            // Compute next element
            let pm = p[m];
            match pk.checked_add(pm) {
                Some(v) => p.push(v),
                None => break, // overflow
            }
            j_lo = m;
        }

        if !best_set {
            eprintln!("ERROR: Could not find transition at step {}", _ti + 1);
            return;
        }

        rn = best_n;
        rd = best_d;
    }

    // Output T(limit) = rn/rd to 10 decimal places
    let integer_part = rn / rd;
    let mut remainder = (rn % rd) as u128;
    let rd128 = rd as u128;

    let mut result = format!("{}", integer_part);
    result.push('.');

    for _ in 0..10 {
        remainder *= 10;
        let digit = remainder / rd128;
        remainder %= rd128;
        result.push((b'0' + digit as u8) as char);
    }

    // Check rounding: need to look at the 11th digit
    remainder *= 10;
    if remainder / rd128 >= 5 {
        let mut chars: Vec<u8> = result.bytes().collect();
        let mut i = chars.len() - 1;
        loop {
            if chars[i] == b'.' {
                if i == 0 { break; }
                i -= 1;
                continue;
            }
            if chars[i] < b'9' {
                chars[i] += 1;
                break;
            } else {
                chars[i] = b'0';
                if i == 0 {
                    chars.insert(0, b'1');
                    break;
                }
                i -= 1;
            }
        }
        result = String::from_utf8(chars).unwrap();
    }

    println!("{}", result);
}

fn main() {
    solve();
}
