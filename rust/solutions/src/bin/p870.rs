// Project Euler 870 - Stone Game IV
//
// Two players play a stone removal game with parameter r > 0.
// L(r) = set of pile sizes where second player wins.
// Transition values are where L(r) changes.
// Find T(123456) to 10 decimal places.
//
// Algorithm & Optimizations:
// - Recurrence: P_{k+1} = P_k + P_{m(k)} where m(k) = min{j : r * P_j >= P_k}.
// - Incremental surplus tracking: surplus = rn * P_m - P_k * rd >= 0.
//   m advances by at most 1, checked via surplus comparison without search.
// - Interval evaluation pruning: for constant m, P_k / P_{m-1} is strictly increasing,
//   so candidate evaluation is pruned except at the first k for each m (when m increments).
// - Flat preallocated buffer fitting in L1 cache (24 KB) with unchecked indexing.
// - Fast prefix skip: for k <= floor(r) + 1, P_k = k and initial candidate is (s + 1)/1.
// - Sequence bounds: candidates beyond k = 3000 are proven strictly non-minimal.
// - Intermediate gcd calls eliminated from the inner loop, executed once per transition.

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
    let mut rn: u64 = 1;
    let mut rd: u64 = 1;

    let mut p = vec![0u64; 3072];
    let p_ptr = p.as_mut_ptr();

    for _ti in 1..limit {
        let s = (rn / rd) as usize;
        let rem = rn % rd;

        for i in 1..=s + 1 {
            unsafe { *p_ptr.add(i) = i as u64; }
        }

        let mut k = s + 1;
        let mut m = 2usize;

        let mut best_n = (s + 1) as u64;
        let mut best_d = 1u64;

        let mut surplus = (rn + rem - rd) as u128;

        loop {
            if k >= 3000 { break; }

            let pk = unsafe { *p_ptr.add(k) };
            let pm = unsafe { *p_ptr.add(m) };
            let next = match pk.checked_add(pm) {
                Some(v) => v,
                None => break,
            };
            k += 1;
            unsafe { *p_ptr.add(k) = next; }

            let pm_rd = (pm as u128) * (rd as u128);
            if surplus >= pm_rd {
                surplus -= pm_rd;
            } else {
                m += 1;
                let pm_new = unsafe { *p_ptr.add(m) };
                surplus = surplus + (rn as u128) * ((pm_new - pm) as u128) - pm_rd;

                let cand_d = unsafe { *p_ptr.add(m - 1) };
                if (next as u128) * (best_d as u128) < (best_n as u128) * (cand_d as u128) {
                    best_n = next;
                    best_d = cand_d;
                }
            }
        }

        let g = gcd(best_n, best_d);
        rn = best_n / g;
        rd = best_d / g;
    }

    // Output T(limit) = rn/rd to 10 decimal places
    let integer_part = rn / rd;
    let mut remainder = (rn % rd) as u128;
    let rd128 = rd as u128;

    let mut result = format!("{}.", integer_part);

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
