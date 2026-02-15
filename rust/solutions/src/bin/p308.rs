// Project Euler 308: An amazing prime-generating automaton
//
// Conway's PRIMEGAME FRACTRAN program. Count steps to reach 10001st prime power of 2.
//
// Key optimization: after the first failed trial in state 2's inner loop,
// five becomes constant S and two resets to 0. Each subsequent failed trial
// with divisor d costs exactly 2*floor(S/d) + 6*S + 2 steps.
// We batch-compute all these trials using the O(√S) floor-sum trick,
// jumping directly to the next exact-division point.

/// Sum of floor(s/d) for d = lo, lo+1, ..., hi.  O(√s) via grouping.
fn floor_sum(s: i64, lo: i64, hi: i64) -> i64 {
    if lo > hi || s <= 0 {
        return 0;
    }
    let mut total = 0i64;
    let mut d = lo;
    while d <= hi {
        let q = s / d;
        if q == 0 {
            break;
        }
        let d_end = (s / q).min(hi);
        total += q * (d_end - d + 1);
        d = d_end + 1;
    }
    total
}

/// Largest divisor of s that is <= d_max.  O(√s).
fn largest_divisor_le(s: i32, d_max: i32) -> i32 {
    let mut best = 1i32;
    let mut i = 1i32;
    while (i as i64) * (i as i64) <= s as i64 {
        if s % i == 0 {
            if i <= d_max && i > best {
                best = i;
            }
            let j = s / i;
            if j <= d_max && j > best {
                best = j;
            }
        }
        i += 1;
    }
    best
}

fn main() {
    let mut two: i32 = 1;
    let mut three: i32 = 0;
    let mut five: i32 = 0;
    let mut seven: i32 = 0;
    let mut state: i32 = 0;
    let mut steps: i64 = 0;
    let mut prime_count: i32 = 0;

    loop {
        if state == 0 && three == 0 && five == 0 && seven == 0 && two > 1 {
            prime_count += 1;
            if prime_count >= 10001 {
                break;
            }
        }

        match state {
            0 => {
                if two > 0 {
                    steps += two as i64;
                    three += two;
                    five += two;
                    two = 0;
                } else if seven > 0 {
                    steps += seven as i64;
                    seven = 0;
                } else {
                    five += 1;
                    state = 1;
                    steps += 1;
                }
            }
            1 => {
                if three > 0 {
                    steps += 2 * three as i64 + 1;
                    seven += three;
                    three = 0;
                    state = 2;
                } else {
                    state = 2;
                    steps += 1;
                }
            }
            2 => {
                if seven <= 0 {
                    state = 1;
                    steps += 1;
                } else {
                    // First trial
                    let d = seven;
                    let q = five / d;
                    let r = five - q * d;

                    steps += q as i64 * (4 * d as i64 + 2);
                    two += q * d;
                    five = r;

                    if r == 0 {
                        // Exact division on first trial
                        seven -= 1;
                        state = 3;
                        steps += 1;
                    } else {
                        // First failed trial: merge states 2_exit→3→4→1→2
                        steps += 2 * r as i64;
                        two += r;
                        // two now = original_two + original_five =: S
                        let s = two;
                        steps += 2; // state 2 exit + state 3
                        steps += 2 * s as i64 + 1; // state 4
                        five = s;
                        seven = d - 1; // net: d - r + (r-1) if r>1, or d - r if r=1; both = d-1
                        if r > 1 {
                            steps += 2 * (r - 1) as i64 + 1;
                        } else {
                            steps += 1;
                        }
                        three = 0;

                        // Now: five = s (constant), two = 0, seven = d - 1 >= 1
                        // Batch all remaining failed trials + final exact division.
                        // Each failed trial at divisor d' costs 2*floor(s/d') + 6s + 2 steps.
                        // We batch from d' = seven down to d' = d_div+1, where d_div is
                        // the largest divisor of s that is <= seven.

                        let d_div = largest_divisor_le(s, seven);
                        if d_div < seven {
                            let hi = seven as i64;
                            let lo = (d_div + 1) as i64;
                            let fs = floor_sum(s as i64, lo, hi);
                            steps += 2 * fs + (6 * s as i64 + 2) * (hi - lo + 1);
                            seven = d_div;
                        }

                        // Exact division at seven = d_div (which divides s)
                        let dd = seven;
                        let qq = five / dd; // s / d_div, exact
                        steps += qq as i64 * (4 * dd as i64 + 2);
                        two = qq * dd; // = s
                        five = 0;
                        seven = dd - 1;
                        state = 3;
                        steps += 1;
                    }
                }
            }
            3 => {
                if five > 0 {
                    five -= 1;
                    two += 1;
                    three += 1;
                    state = 2;
                    steps += 1;
                } else if three > 0 {
                    three -= 1;
                    state = 4;
                    steps += 1;
                } else {
                    state = 0;
                    steps += 1;
                }
            }
            4 => {
                if two > 0 {
                    steps += 2 * two as i64;
                    five += two;
                    two = 0;
                } else {
                    seven += 1;
                    state = 1;
                    steps += 1;
                }
            }
            _ => unreachable!(),
        }
    }

    println!("{}", steps);
}
