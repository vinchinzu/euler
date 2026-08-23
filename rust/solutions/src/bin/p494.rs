// Project Euler 494: Collatz prefix families
//
// Base count is F_90; extra families come from non-redundant "special"
// prefixes found by scanning starts < L. Inverse-Collatz counting unrolls
// the doubling spine and parallelizes inverse-odd branches.

use rayon::prelude::*;

const NSTEPS: usize = 90;
const L: u64 = 100_000;
const M3_39: u64 = 3u64.pow(39);
const PAR_STEPS: usize = 36;

fn fibonacci(n: usize) -> u64 {
    if n == 0 {
        return 0;
    }
    let (mut a, mut b) = (1u64, 1u64);
    for _ in 2..n {
        let c = a + b;
        a = b;
        b = c;
    }
    if n == 1 { a } else { b }
}

#[inline(always)]
fn is_power_of_2(n: u64) -> bool {
    n > 0 && (n & (n - 1)) == 0
}

/// n < M3_39, so 2n is either 2n or 2n - M3_39.
#[inline(always)]
fn double_mod(n: u64) -> u64 {
    let n2 = n << 1;
    if n2 >= M3_39 { n2 - M3_39 } else { n2 }
}

/// True iff some Collatz iterate exceeds `start` while the 3/2-ratio is < 1.
fn is_special(start: u64) -> bool {
    let mut n = start;
    let mut r = 1.0f64;
    for _ in 0..NSTEPS {
        if n & 1 == 0 {
            n >>= 1;
            r *= 0.5;
        } else {
            n = 3 * n + 1;
            r *= 3.0;
        }
        if is_power_of_2(n) {
            break;
        }
        if n > start && r < 1.0 {
            return true;
        }
    }
    false
}

/// Sequential inverse-Collatz path count (explicit stack, unrolled doubles).
/// Stack entries pack (n, steps << 1 | even).
fn helper_seq(n0: u64, num_steps: usize, n_even: bool) -> u64 {
    let mut stack = [(0u64, 0usize); 128];
    let mut sp = 1usize;
    stack[0] = (n0, (num_steps << 1) | usize::from(n_even));
    let mut result = 0u64;
    while sp > 0 {
        sp -= 1;
        let (mut n, packed) = stack[sp];
        let mut steps = packed >> 1;
        let mut even = packed & 1 == 1;
        loop {
            if steps == 0 {
                result += 1;
                break;
            }
            if even && n % 3 == 1 {
                stack[sp] = ((n - 1) / 3, (steps - 1) << 1);
                sp += 1;
            }
            n = double_mod(n);
            even = true;
            steps -= 1;
        }
    }
    result
}

/// Parallel over inverse-odd branches along the doubling spine when deep enough.
fn helper_func(n0: u64, num_steps: usize, n_even: bool) -> u64 {
    if num_steps < PAR_STEPS {
        return helper_seq(n0, num_steps, n_even);
    }
    let mut extras = Vec::with_capacity(num_steps);
    let mut n = n0;
    let mut even = n_even;
    let mut steps = num_steps;
    while steps > 0 {
        if even && n % 3 == 1 {
            extras.push(((n - 1) / 3, steps - 1));
        }
        n = double_mod(n);
        even = true;
        steps -= 1;
    }
    1 + extras
        .into_par_iter()
        .with_min_len(1)
        .map(|(nu, su)| helper_func(nu, su, false))
        .sum::<u64>()
}

fn main() {
    let mut ans = fibonacci(NSTEPS);

    let mut special_set = vec![false; L as usize];
    special_set
        .par_iter_mut()
        .enumerate()
        .skip(1)
        .for_each(|(i, slot)| *slot = is_special(i as u64));

    let jobs: Vec<(u64, usize)> = (1..L)
        .filter_map(|start| {
            if !special_set[start as usize] {
                return None;
            }
            let mut n = start;
            let mut len = 0usize;
            for _ in 0..NSTEPS {
                if len > 0 {
                    let v = n as usize;
                    if v < special_set.len() && special_set[v] {
                        return None;
                    }
                }
                len += 1;
                n = if n & 1 == 0 { n >> 1 } else { 3 * n + 1 };
                if is_power_of_2(n) {
                    break;
                }
            }
            Some((start, NSTEPS - len))
        })
        .collect();

    let extra: u64 = jobs
        .into_par_iter()
        .with_min_len(1)
        .map(|(start, num_steps)| helper_func(start, num_steps, false))
        .sum();
    ans += extra;

    println!("{}", ans);
}
