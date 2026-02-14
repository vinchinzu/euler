// Project Euler 520 - Simbers
// Matrix exponentiation for counting Simbers with digit parity constraints.

const MOD: i64 = 1_000_000_123;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct State {
    zero_odd: u8,
    odd_odd: u8,
    even_odd: u8,
    odd_even: u8,
    even_even: u8,
    empty: bool,
}

fn main() {
    let n_exp = 39;

    // Generate states
    let mut states = Vec::new();
    let mut state_map = std::collections::HashMap::new();

    // Start state first
    let start = State { zero_odd: 5, odd_odd: 0, even_odd: 0, odd_even: 0, even_even: 5, empty: true };
    state_map.insert(start, 0usize);
    states.push(start);

    // All non-empty states
    for zo in 0..=5u8 {
        for oo in 0..=(5 - zo) {
            let eo = 5 - zo - oo;
            for oe in 0..=5u8 {
                let ee = 5 - oe;
                let s = State { zero_odd: zo, odd_odd: oo, even_odd: eo, odd_even: oe, even_even: ee, empty: false };
                if !state_map.contains_key(&s) {
                    let idx = states.len();
                    state_map.insert(s, idx);
                    states.push(s);
                }
            }
        }
    }

    let ns = states.len();
    let get_idx = |s: &State| -> usize { *state_map.get(s).unwrap() };

    // Build transition matrix
    let mut a = vec![0i64; ns * ns];
    let mat = |m: &[i64], i: usize, j: usize| -> i64 { m[i * ns + j] };
    let mat_set = |m: &mut [i64], i: usize, j: usize, v: i64| { m[i * ns + j] = v; };

    for si in 0..ns {
        let s = states[si];

        if s.zero_odd > 0 {
            let ns_new = State { zero_odd: s.zero_odd - 1, odd_odd: s.odd_odd + 1, even_odd: s.even_odd,
                                 odd_even: s.odd_even, even_even: s.even_even, empty: false };
            let ni = get_idx(&ns_new);
            let cur = mat(&a, ni, si);
            mat_set(&mut a, ni, si, (cur + s.zero_odd as i64) % MOD);
        }

        if s.odd_odd > 0 {
            let ns_new = State { zero_odd: s.zero_odd, odd_odd: s.odd_odd - 1, even_odd: s.even_odd + 1,
                                 odd_even: s.odd_even, even_even: s.even_even, empty: false };
            let ni = get_idx(&ns_new);
            let cur = mat(&a, ni, si);
            mat_set(&mut a, ni, si, (cur + s.odd_odd as i64) % MOD);
        }

        if s.even_odd > 0 {
            let ns_new = State { zero_odd: s.zero_odd, odd_odd: s.odd_odd + 1, even_odd: s.even_odd - 1,
                                 odd_even: s.odd_even, even_even: s.even_even, empty: false };
            let ni = get_idx(&ns_new);
            let cur = mat(&a, ni, si);
            mat_set(&mut a, ni, si, (cur + s.even_odd as i64) % MOD);
        }

        if s.odd_even > 0 {
            let ns_new = State { zero_odd: s.zero_odd, odd_odd: s.odd_odd, even_odd: s.even_odd,
                                 odd_even: s.odd_even - 1, even_even: s.even_even + 1, empty: false };
            let ni = get_idx(&ns_new);
            let cur = mat(&a, ni, si);
            mat_set(&mut a, ni, si, (cur + s.odd_even as i64) % MOD);
        }

        if s.even_even > 0 {
            let ns_new = State { zero_odd: s.zero_odd, odd_odd: s.odd_odd, even_odd: s.even_odd,
                                 odd_even: s.odd_even + 1, even_even: s.even_even - 1, empty: false };
            let ni = get_idx(&ns_new);
            if s.empty {
                // Leading zero: stays in same state for one of them
                let cur = mat(&a, si, si);
                mat_set(&mut a, si, si, (cur + 1) % MOD);
                let cur = mat(&a, ni, si);
                mat_set(&mut a, ni, si, (cur + s.even_even as i64 - 1) % MOD);
            } else {
                let cur = mat(&a, ni, si);
                mat_set(&mut a, ni, si, (cur + s.even_even as i64) % MOD);
            }
        }
    }

    // Matrix multiply
    let mat_mul = |a: &[i64], b: &[i64]| -> Vec<i64> {
        let mut c = vec![0i64; ns * ns];
        for i in 0..ns {
            for k in 0..ns {
                let aik = a[i * ns + k];
                if aik == 0 { continue; }
                for j in 0..ns {
                    c[i * ns + j] = (c[i * ns + j] + (aik as i128 * b[k * ns + j] as i128 % MOD as i128) as i64) % MOD;
                }
            }
        }
        c
    };

    // Identify accepting states
    let accepting: Vec<bool> = states.iter().map(|s| s.even_odd == 0 && s.odd_even == 0 && !s.empty).collect();

    let start_idx = 0usize;

    // An = A^2
    let mut an = mat_mul(&a, &a);
    let mut ans: i64 = 0;

    // u=1: An = A^2
    for i in 0..ns {
        if accepting[i] {
            ans = (ans + an[i * ns + start_idx]) % MOD;
        }
    }

    for _u in 2..=n_exp {
        an = mat_mul(&an, &an);
        for i in 0..ns {
            if accepting[i] {
                ans = (ans + an[i * ns + start_idx]) % MOD;
            }
        }
    }

    println!("{}", ans);
}
