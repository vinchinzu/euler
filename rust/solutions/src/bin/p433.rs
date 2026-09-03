use rayon::prelude::*;

const N: usize = 5_000_000;

#[inline(always)]
fn ncr2(n: i64) -> i64 {
    if n < 2 {
        0
    } else {
        n * (n - 1) / 2
    }
}

#[derive(Clone, Copy)]
struct Step {
    q: i32,
    rem: i32,
    b: i32,
    b_over_g: i32,
}

#[inline(always)]
fn extgcd_and_steps(b: i64, a: i64, steps: &mut [Step; 16]) -> (i64, i64, usize) {
    let (mut y, mut y1) = (0i64, 1i64);
    let mut cur_a = b;
    let mut cur_b = a;
    let mut count = 0;
    while cur_b != 0 {
        let q = cur_a / cur_b;
        let rem = cur_a % cur_b;
        steps[count] = Step { q: q as i32, rem: rem as i32, b: cur_b as i32, b_over_g: cur_b as i32 };
        count += 1;
        let next_y1 = y - q * y1;
        y = y1;
        y1 = next_y1;
        cur_a = cur_b;
        cur_b = rem;
    }
    let gcd = cur_a;
    if gcd > 1 {
        for s in &mut steps[..count] {
            s.b_over_g /= gcd as i32;
        }
    }
    (gcd, y, count)
}

#[inline(always)]
fn floor_sum_from_steps(mut n: i64, steps: &[Step]) -> i64 {
    let mut ans = 0i64;
    let mut sign = 1i64;
    for s in steps {
        if n <= 0 {
            break;
        }
        ans += sign * (s.q as i64) * (n * (n + 1) / 2);
        if s.rem == 0 {
            break;
        }
        let m = (s.rem as i64 * n) / s.b as i64;
        ans += sign * (m * n + n / s.b_over_g as i64);
        sign = -sign;
        n = m;
    }
    ans
}

#[inline(always)]
fn contribution_for_b(b: i64, c: i64, sqrt_c: i64, x2: i64, steps: &mut [Step; 16]) -> i64 {
    let mut res = 0i64;
    for a in 1..b {
        let (gcd, y_coef, num_steps) = extgcd_and_steps(b, a, steps);
        let scale = c / gcd;
        let temp = y_coef * scale;
        let y_mod = temp % b;
        let y = y_mod - b;
        let x = (c - a * y) / b;

        let x1 = c / (a + b);
        let active_steps = &steps[..num_steps];

        if sqrt_c > x1 {
            let pts1 = floor_sum_from_steps(x - x1 - 1, active_steps);
            let pts2 = floor_sum_from_steps(x - sqrt_c - 1, active_steps);
            let pts3 = floor_sum_from_steps(x - x2 - 1, active_steps);
            res += ncr2(x1) + pts1 + pts2 - 2 * pts3 + (2 * x2 - x1 - sqrt_c) * y;
        } else {
            let pts1 = floor_sum_from_steps(x - x1 - 1, active_steps);
            let pts3 = floor_sum_from_steps(x - x2 - 1, active_steps);
            res += 2 * (ncr2(x1) + pts1 - pts3 + (x2 - x1) * y) - ncr2(sqrt_c);
        }
    }
    res
}

struct Task {
    c: i64,
    sqrt_c: i64,
    m: i64,
    b_start: i64,
    b_end: i64,
}

fn main() {
    let mut phi = vec![0i32; N + 1];
    let mut mobius = vec![0i8; N + 1];
    let mut primes = Vec::with_capacity(350_000);

    phi[1] = 1;
    mobius[1] = 1;

    let mut ans: i64 = 0;

    for i in 2..=N {
        if phi[i] == 0 {
            phi[i] = (i - 1) as i32;
            mobius[i] = -1;
            primes.push(i as u32);
        }
        if i >= 3 {
            ans += (N / i) as i64 * (phi[i] as i64 / 2);
        }
        for &p in &primes {
            let p = p as usize;
            let ip = i * p;
            if ip > N {
                break;
            }
            if i % p == 0 {
                phi[ip] = phi[i] * p as i32;
                mobius[ip] = 0;
                break;
            } else {
                phi[ip] = phi[i] * (p as i32 - 1);
                mobius[ip] = -mobius[i];
            }
        }
    }

    drop(phi);
    drop(primes);

    let mut pref_mobius = vec![0i32; N + 1];
    for i in 1..=N {
        pref_mobius[i] = pref_mobius[i - 1] + mobius[i] as i32;
    }
    drop(mobius);

    let mut tasks = Vec::with_capacity(9000);
    let mut l = 1;
    while l <= N {
        let c = (N / l) as i64;
        let r = N / (c as usize);
        let m = (pref_mobius[r] - pref_mobius[l - 1]) as i64;
        if m != 0 && c >= 4 {
            let sqrt_c = (c as u64).isqrt() as i64;
            if sqrt_c > 64 {
                let step = 32;
                let mut b_start = 1;
                while b_start <= sqrt_c {
                    let b_end = (b_start + step - 1).min(sqrt_c);
                    tasks.push(Task { c, sqrt_c, m, b_start, b_end });
                    b_start += step;
                }
            } else {
                tasks.push(Task { c, sqrt_c, m, b_start: 1, b_end: sqrt_c });
            }
        }
        l = r + 1;
    }
    drop(pref_mobius);

    let extra: i64 = tasks
        .into_par_iter()
        .map(|task| {
            let mut steps = [Step { q: 0, rem: 0, b: 0, b_over_g: 0 }; 16];
            let mut sum = 0i64;
            for b in task.b_start..=task.b_end {
                sum += contribution_for_b(b, task.c, task.sqrt_c, task.c / b, &mut steps);
            }
            task.m * sum
        })
        .sum();
    ans += extra;

    ans *= 4;
    ans += N as i64 * N as i64 + N as i64 / 2;

    println!("{}", ans);
}
