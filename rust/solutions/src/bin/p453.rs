// Project Euler 453 - Simple quadrilaterals
// Mobius function and modular arithmetic on lattice.

use euler_utils::mod_pow;

const W: i64 = 12345;
const H: i64 = 6789;
const M: u64 = 135707531;

fn mod_inv_val(a: u64, m: u64) -> u64 {
    euler_utils::mod_inv(a, m).unwrap()
}

fn sum_powers(n: i64, k: u64) -> u64 {
    let mut result = 0u64;
    for i in 1..=n as u64 {
        result = (result + mod_pow(i, k, M)) % M;
    }
    result
}

fn f(a: u64, b: u64) -> u64 {
    mod_pow((W + 1) as u64, a, M) * mod_pow((H + 1) as u64, b, M) % M
}

fn g(a: u64, b: u64, c: u64, mu: &[i32]) -> u64 {
    let l = W.min(H);
    let mut res = 0u64;
    for gv in 1..=l {
        for gp in 1..=(l / gv) {
            if mu[gp as usize] == 0 { continue; }
            let mut term = if mu[gp as usize] < 0 {
                M - ((-mu[gp as usize]) as u64 * mod_pow(gv as u64, a + b + c, M) % M) % M
            } else {
                mu[gp as usize] as u64 * mod_pow(gv as u64, a + b + c, M) % M
            };
            term = term * mod_pow(gp as u64, a + b, M) % M;
            term = term * sum_powers(W / gv / gp, a) % M;
            term = term * sum_powers(H / gv / gp, b) % M;
            res = (res + term) % M;
        }
    }
    res
}

fn main() {
    let l = W.min(H) as usize;

    // Compute Mobius function
    let mut is_prime = vec![true; l + 1];
    is_prime[0] = false;
    if l >= 1 { is_prime[1] = false; }
    let mut mu = vec![1i32; l + 1];

    for i in 2..=l {
        if is_prime[i] {
            for j in (i..=l).step_by(i) {
                is_prime[j] = false;
                if (j as i64) % ((i as i64) * (i as i64)) == 0 {
                    mu[j] = 0;
                } else {
                    mu[j] = -mu[j];
                }
            }
        }
    }
    // Fix: i is prime for itself
    for i in 2..=l { if is_prime[i] { /* already handled in loop */ } }

    let inv3 = mod_inv_val(3, M);
    let inv12 = mod_inv_val(12, M);
    let inv18 = mod_inv_val(18, M);
    let inv432 = mod_inv_val(432, M);

    let g112 = g(1, 1, 2, &mu);
    let g102 = g(1, 0, 2, &mu);
    let g012 = g(0, 1, 2, &mu);
    let g002 = g(0, 0, 2, &mu);
    let g101 = g(1, 0, 1, &mu);
    let g011 = g(0, 1, 1, &mu);
    let g001 = g(0, 0, 1, &mu);
    let g111 = g(1, 1, 1, &mu);

    let f00 = f(0, 0);
    let f01 = f(0, 1);
    let f10 = f(1, 0);
    let f11 = f(1, 1);
    let f12 = f(1, 2);
    let f21 = f(2, 1);
    let f22 = f(2, 2);
    let f13 = f(1, 3);
    let f31 = f(3, 1);
    let f14 = f(1, 4);
    let f41 = f(4, 1);
    let f24 = f(2, 4);
    let f42 = f(4, 2);
    let f33 = f(3, 3);
    let f44 = f(4, 4);

    let mm = |a: u64, b: u64| -> u64 { (a as u128 * b as u128 % M as u128) as u64 };

    let mut ans: u64 = 0;

    // Term 1: 20/3 * (...)
    let t1 = (mm(f00, g112) + 4 * M - mm(f01, g102) - mm(f10, g012) + mm(f11, g002)) % M;
    ans = (ans + mm(20 * inv3 % M, t1)) % M;

    // Term 2: 7 * (...)
    let t2 = (mm(f01, g101) + mm(f10, g011) + 4 * M - mm(f11, g001) - mm(f00, g111)) % M;
    ans = (ans + 7 * t2 % M) % M;

    // Term 3: 4 * (...)
    let t3 = (mm(f12, g101) + mm(f21, g011) + 4 * M - mm(f22, g001) - mm(f11, g111)) % M;
    ans = (ans + 4 * t3 % M) % M;

    // Remaining terms
    ans = (ans + mm(7 * inv12 % M, f11)) % M;
    ans = (ans + M - mm(5 * inv18 % M, (f12 + f21) % M)) % M;
    ans = (ans + M - mm(7 * inv12 % M, (f13 + f31) % M)) % M;
    ans = (ans + mm(269 * inv432 % M, f22)) % M;
    ans = (ans + mm(5 * inv18 % M, (f14 + f41) % M)) % M;
    ans = (ans + M - mm(149 * inv432 % M, (f24 + f42) % M)) % M;
    ans = (ans + mm(7 * inv12 % M, f33)) % M;
    ans = (ans + mm(29 * inv432 % M, f44)) % M;

    ans %= M;

    println!("{}", ans);
}
