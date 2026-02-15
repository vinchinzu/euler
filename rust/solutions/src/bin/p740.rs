// Project Euler 740 - Secret Santa
//
// DP with memoization. State: (ns0, ns1, ns2).

const NS0_MAX: usize = 101;
const NS1_MAX: usize = 201;
const NS2_MAX: usize = 101;

static mut MEMO: [[[f64; NS2_MAX]; NS1_MAX]; NS0_MAX] = [[[0.0; NS2_MAX]; NS1_MAX]; NS0_MAX];
static mut VISITED: [[[bool; NS2_MAX]; NS1_MAX]; NS0_MAX] =
    [[[false; NS2_MAX]; NS1_MAX]; NS0_MAX];

fn npr(n: i32, r: i32) -> f64 {
    let mut result = 1.0;
    for i in 0..r {
        result *= (n - i) as f64;
    }
    result
}

fn f(ns0: i32, ns1: i32, ns2: i32) -> f64 {
    if ns0 < 0 || ns1 < 0 || ns2 < 0 {
        return 0.0;
    }
    let sum_ns = ns0 + ns1 + ns2;
    if sum_ns == 1 {
        return if ns0 == 0 { 0.0 } else { 1.0 };
    }
    if sum_ns <= 0 {
        return 0.0;
    }

    let (u0, u1, u2) = (ns0 as usize, ns1 as usize, ns2 as usize);
    unsafe {
        if VISITED[u0][u1][u2] {
            return MEMO[u0][u1][u2];
        }
    }

    let slips = [2 * ns0 + ns1, ns1, 2 * ns2, 1];
    let mut result = 0.0;

    for p in 0..3 {
        let ns_p = match p {
            0 => ns0,
            1 => ns1,
            _ => ns2,
        };
        if ns_p <= 0 {
            continue;
        }

        let s2_max = |s1: usize| if s1 == 2 { 4 } else { 3 };

        for s1 in 0..3 {
            for s2 in 0..s2_max(s1) {
                let mut num_s1 = slips[s1];
                let mut num_s2 = slips[s2];

                if s1 == p as usize {
                    num_s1 -= p as i32;
                }
                if s2 == p as usize {
                    num_s2 -= p as i32;
                }

                if s1 == s2 {
                    num_s2 -= if s1 == 2 { 2 } else { 1 };
                }

                if num_s1 <= 0 || num_s2 <= 0 {
                    continue;
                }

                let mut new_ns0 = ns0;
                let mut new_ns1 = ns1;
                let mut new_ns2 = ns2;

                match p {
                    0 => new_ns0 -= 1,
                    1 => new_ns1 -= 1,
                    _ => new_ns2 -= 1,
                }

                if s1 != 0 {
                    if s1 == 1 {
                        new_ns1 -= 1;
                    } else if s1 == 2 {
                        new_ns2 -= 1;
                    }

                    if s2 == 3 {
                        new_ns0 += 1;
                    }

                    if s1 == 1 {
                        new_ns0 += 1;
                    } else if s1 == 2 {
                        if s2 != 3 {
                            new_ns1 += 1;
                        }
                    }
                }

                if s2 != 0 && s2 != 3 {
                    if s2 == 1 {
                        new_ns1 -= 1;
                        new_ns0 += 1;
                    } else if s2 == 2 {
                        new_ns2 -= 1;
                        new_ns1 += 1;
                    }
                }

                if new_ns0 < 0 || new_ns1 < 0 || new_ns2 < 0 {
                    continue;
                }
                if new_ns0 as usize >= NS0_MAX
                    || new_ns1 as usize >= NS1_MAX
                    || new_ns2 as usize >= NS2_MAX
                {
                    continue;
                }

                let sub = f(new_ns0, new_ns1, new_ns2);
                result +=
                    sub * num_s1 as f64 * num_s2 as f64 / npr(2 * sum_ns - p as i32, 2)
                        * ns_p as f64;
            }
        }
    }

    result /= sum_ns as f64;
    unsafe {
        MEMO[u0][u1][u2] = result;
        VISITED[u0][u1][u2] = true;
    }
    result
}

fn main() {
    let result = f(0, 0, 100);
    println!("{:.10}", result);
}
