// Project Euler 415 - Titanic sets
// Uses Lucy DP for sum of multiplicative functions.
// Euler phi sieve, prefix sums, then Lucy DP + main formula.

use euler_utils::mod_pow;

const N_VAL: i64 = 100_000_000_000; // 10^11
const MOD: i64 = 100_000_000;       // 10^8

fn modd(x: i64) -> i64 {
    ((x % MOD) + MOD) % MOD
}

fn s_k(k: i32, n: i64) -> i64 {
    if n <= 0 { return 0; }
    let nm = modd(n);
    match k {
        0 => nm,
        1 => {
            if n % 2 == 0 {
                modd((n / 2) % MOD * modd(n + 1)) % MOD
            } else {
                modd(nm * modd((n + 1) / 2)) % MOD
            }
        }
        2 => {
            let (mut a, mut b, mut c) = (n, n + 1, 2 * n + 1);
            if a % 2 == 0 { a /= 2; } else { b /= 2; }
            if a % 3 == 0 { a /= 3; } else if b % 3 == 0 { b /= 3; } else { c /= 3; }
            modd(modd(a) * modd(b) % MOD * modd(c)) % MOD
        }
        3 => {
            let t = s_k(1, n);
            t * t % MOD
        }
        _ => 0,
    }
}

fn sq(n: i64) -> i64 {
    let v = modd(n);
    v * v % MOD
}

fn sum_ag(l: i64, e: i32) -> i64 {
    if l <= 0 { return 0; }
    let p2 = mod_pow(2, (l + 1) as u64, MOD as u64) as i64;
    match e {
        0 => modd(p2 - 2),
        1 => modd(modd(l - 1) * p2 % MOD + 2),
        2 => {
            let lm = modd(l);
            let t = modd(lm * lm % MOD - 2 * lm % MOD + 3);
            modd(t * p2 % MOD - 6)
        }
        _ => 0,
    }
}

fn isqrt_ll(n: i64) -> i64 {
    let mut r = (n as f64).sqrt() as i64;
    while r * r > n { r -= 1; }
    while (r + 1) * (r + 1) <= n { r += 1; }
    r
}

fn main() {
    let n = N_VAL;
    let l = isqrt_ll(n);

    let mut u = (n as f64).powf(2.0 / 3.0).round() as i64;
    if u < l + 1 { u = l + 1; }

    // Euler phi sieve
    let mut phi_arr: Vec<i32> = (0..=u as usize).map(|i| i as i32).collect();
    for i in 2..=u as usize {
        if phi_arr[i] == i as i32 {
            let mut j = i;
            while j <= u as usize {
                phi_arr[j] -= phi_arr[j] / i as i32;
                j += i;
            }
        }
    }

    // Prefix sums
    let mut pref1 = vec![0i32; u as usize + 1];
    let mut pref2 = vec![0i32; u as usize + 1];
    {
        let mut running = 0i64;
        for x in 1..=u as usize {
            running = (running + (x as i64 % MOD) * phi_arr[x] as i64) % MOD;
            pref1[x] = running as i32;
        }
    }
    {
        let mut running = 0i64;
        for x in 1..=u as usize {
            let xm = x as i64 % MOD;
            running = (running + xm * xm % MOD * phi_arr[x] as i64) % MOD;
            pref2[x] = running as i32;
        }
    }

    // Convert phi_arr to pref0 in-place
    {
        let mut running = 0i64;
        phi_arr[0] = 0;
        for x in 1..=u as usize {
            running = (running + phi_arr[x] as i64) % MOD;
            phi_arr[x] = running as i32;
        }
    }

    // Lucy DP arrays
    let mut sm: [Vec<i32>; 3] = [
        vec![0i32; l as usize + 2],
        vec![0i32; l as usize + 2],
        vec![0i32; l as usize + 2],
    ];
    let mut lg: [Vec<i32>; 3] = [
        vec![0i32; l as usize + 2],
        vec![0i32; l as usize + 2],
        vec![0i32; l as usize + 2],
    ];

    let prefs: [&[i32]; 3] = [&phi_arr, &pref1, &pref2];

    for e in 0..3 {
        let pref = prefs[e];
        for q in 1..=l as usize {
            sm[e][q] = pref[q];
        }

        for g in (1..=l as usize).rev() {
            let nv = n / g as i64;
            if nv <= u {
                lg[e][g] = pref[nv as usize];
                continue;
            }

            let mut result = s_k(e as i32 + 1, nv);
            let mut d = 2i64;
            while d <= nv {
                let q = nv / d;
                let d_max = nv / q;
                let coeff = modd(s_k(e as i32, d_max) - s_k(e as i32, d - 1));

                let te_q = if q <= u {
                    pref[q as usize] as i64
                } else if q <= l {
                    sm[e][q as usize] as i64
                } else {
                    lg[e][(n / q) as usize] as i64
                };

                result = modd(result - coeff * (te_q % MOD) % MOD);
                d = d_max + 1;
            }

            lg[e][g] = (result % MOD) as i32;
        }
    }

    // Helper closures
    let sp_div = |ee: usize, gg: i64| -> i64 {
        let v = n / gg;
        if v <= l { sm[ee][v as usize] as i64 } else { lg[ee][gg as usize] as i64 }
    };

    let sp_get = |ee: usize, qq: i64| -> i64 {
        if qq <= l { sm[ee][qq as usize] as i64 } else { lg[ee][(n / qq) as usize] as i64 }
    };

    // Main formula
    let p2n1 = mod_pow(2, (n + 1) as u64, MOD as u64) as i64;
    let mut ans = mod_pow(p2n1 as u64, (n + 1) as u64, MOD as u64) as i64;
    ans = modd(ans - 1);
    ans = modd(ans - sq(n + 1));

    let n1 = modd(n + 1);
    let n1sq = sq(n + 1);

    let term = modd(p2n1 - 1 - n1 - s_k(1, n));
    ans = modd(ans - 2 * n1 % MOD * term % MOD);

    // First loop: g = 1..L
    for g in 1..=l {
        let gm = modd(g);
        let t = modd(
            gm * gm % MOD * modd(sp_div(2, g)) % MOD
            - 3 * n1 % MOD * gm % MOD * modd(sp_div(1, g)) % MOD
            + 2 * n1sq % MOD * modd(sp_div(0, g)) % MOD
            - modd(n + 1 - g) * n1 % MOD
        );
        let p2g = mod_pow(2, g as u64, MOD as u64) as i64;
        ans = modd(ans - modd(p2g - 2) * t % MOD);
    }

    // Second loop: q = 1..N/L - 1
    let q_lim = n / l;
    for q in 1..q_lim {
        let t2 = modd(sp_get(2, q));
        let t1 = modd(sp_get(1, q));
        let t0 = modd(sp_get(0, q));

        let nq = n / q;
        let nq1 = n / (q + 1);

        let sag2 = modd(sum_ag(nq, 2) - sum_ag(nq1, 2));
        let sag1 = modd(sum_ag(nq, 1) - sum_ag(nq1, 1));
        let sag0 = modd(sum_ag(nq, 0) - sum_ag(nq1, 0));
        let sp2 = modd(s_k(2, nq) - s_k(2, nq1));
        let sp1 = modd(s_k(1, nq) - s_k(1, nq1));
        let sp0 = modd(s_k(0, nq) - s_k(0, nq1));

        let v3t1m1 = modd(3 * t1 - 1);
        let v2t0m1 = modd(2 * t0 - 1);

        ans = modd(ans
            - t2 * sag2 % MOD
            + n1 * v3t1m1 % MOD * sag1 % MOD
            - n1sq * v2t0m1 % MOD * sag0 % MOD
            + 2 * t2 % MOD * sp2 % MOD
            - 2 * n1 % MOD * v3t1m1 % MOD * sp1 % MOD
            + 2 * n1sq % MOD * v2t0m1 % MOD * sp0 % MOD
        );
    }

    println!("{}", modd(ans));
}
