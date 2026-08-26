// Project Euler 433 - Steps in Euclid's Algorithm
// S(N) = sum of E(x,y) for 1 <= x,y <= N, N = 5*10^6.

use rayon::prelude::*;

const N: usize = 5_000_000;

fn gcd_fn(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

fn isqrt(n: i64) -> i64 {
    let mut x = (n as f64).sqrt() as i64;
    while x > 0 && x * x > n {
        x -= 1;
    }
    while (x + 1) * (x + 1) <= n {
        x += 1;
    }
    x
}

fn ncr2(n: i64) -> i64 {
    if n < 2 {
        0
    } else {
        n * (n - 1) / 2
    }
}

fn floor_sum(n: i64, a: i64, b: i64) -> i64 {
    if n <= 0 || a == 0 {
        return 0;
    }
    let mut ans = 0i64;
    let mut a = a;
    if a >= b {
        ans += (a / b) * n * (n + 1) / 2;
        a %= b;
    }
    if a == 0 {
        return ans;
    }
    let m = a * n / b;
    let g = gcd_fn(a, b);
    ans + m * n - floor_sum(m, b, a) + (m * g) / a
}

fn extgcd(mut a: i64, mut b: i64) -> (i64, i64, i64) {
    let (mut x, mut y, mut x1, mut y1) = (1i64, 0i64, 0i64, 1i64);
    while b != 0 {
        let q = a / b;
        let t = b;
        b = a % b;
        a = t;
        let t = x1;
        x1 = x - q * x1;
        x = t;
        let t = y1;
        y1 = y - q * y1;
        y = t;
    }
    (a, x, y)
}

#[inline]
fn contribution_for_b(b: i64, c: i64, sqrt_c: i64) -> i64 {
    let mut res = 0i64;
    for a in 1..b {
        let (gcd, _x_coef, y_coef) = extgcd(b, a);
        let scale = c / gcd;
        let temp = y_coef * scale;
        let y_mod = temp % b;
        let y = y_mod - b;
        let x = (c - a * y) / b;

        let x1 = c / (a + b);
        let x2 = c / b;

        if sqrt_c > x1 {
            res += ncr2(x1);
            let pts1 = floor_sum(x - x1 - 1, b, a);
            let pts2 = floor_sum(x - sqrt_c - 1, b, a);
            let pts3 = floor_sum(x - x2 - 1, b, a);
            res += pts1 + pts2 - 2 * pts3;
            res += (2 * x2 - x1 - sqrt_c) * y;
        } else {
            let pts1 = floor_sum(x - x1 - 1, b, a);
            let pts3 = floor_sum(x - x2 - 1, b, a);
            res += 2 * (ncr2(x1) + pts1 - pts3 + (x2 - x1) * y);
            res -= ncr2(sqrt_c);
        }
    }
    res
}

fn main() {
    // Sieve for phi and mobius
    let mut phi = vec![0i32; N + 1];
    let mut mobius = vec![0i8; N + 1];
    let mut spf = vec![0i32; N + 1];

    for i in 0..=N {
        phi[i] = i as i32;
        spf[i] = i as i32;
        mobius[i] = 1;
    }

    for i in 2..=N {
        if spf[i] == i as i32 {
            phi[i] = (i - 1) as i32;
            mobius[i] = -1;
            let mut j = (i as i64) * (i as i64);
            while j <= N as i64 {
                if spf[j as usize] == j as i32 {
                    spf[j as usize] = i as i32;
                }
                j += i as i64;
            }
        } else {
            let p = spf[i] as usize;
            let q = i / p;
            if q % p == 0 {
                phi[i] = phi[q] * p as i32;
                mobius[i] = 0;
            } else {
                phi[i] = phi[q] * (p as i32 - 1);
                mobius[i] = -mobius[q];
            }
        }
    }

    let mut ans: i64 = 0;

    for sum in 3..=N {
        ans += (N / sum) as i64 * (phi[sum] as i64 / 2);
    }

    let extra: i64 = (1..=N)
        .into_par_iter()
        .map(|g| {
            if mobius[g] == 0 {
                return 0;
            }

            let c = (N / g) as i64;
            let sqrt_c = isqrt(c);
            // A handful of small g values contain most of the work. Split their
            // b-ranges too, so the g=1 task cannot pin one Rayon worker.
            let res: i64 = if g <= 64 {
                (1..sqrt_c + 1)
                    .into_par_iter()
                    .map(|b| contribution_for_b(b, c, sqrt_c))
                    .sum()
            } else {
                (1..sqrt_c + 1)
                    .map(|b| contribution_for_b(b, c, sqrt_c))
                    .sum()
            };
            mobius[g] as i64 * res
        })
        .sum();
    ans += extra;

    ans *= 4;
    ans += N as i64 * N as i64 + N as i64 / 2;

    println!("{}", ans);
}
