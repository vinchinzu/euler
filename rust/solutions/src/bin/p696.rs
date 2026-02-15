// Project Euler 696 - Mahjong
// NFA-to-DFA DP for single-suit counting, interpolation to large n,
// polynomial exponentiation across suits.

const MOD: i64 = 1_000_000_007;
const T_VAL: usize = 30;
const BIG_N: i64 = 100_000_000;
const BIG_S: i64 = 100_000_000;
const NUM_STATES: usize = 18;
const START_N: usize = 3 * T_VAL + 3;
const NPTS: usize = 2 * T_VAL + 3;
const MAX_N: usize = START_N + NPTS;

fn encode(prev: i32, curr: i32, pair: i32) -> usize {
    (prev * 6 + curr * 2 + pair) as usize
}

fn decode(code: usize) -> (i32, i32, i32) {
    let pair = (code % 2) as i32;
    let curr = ((code / 2) % 3) as i32;
    let prev = (code / 6) as i32;
    (prev, curr, pair)
}

fn dfa_transition(old_mask: u32, k: i32) -> u32 {
    let mut new_mask = 0u32;
    for si in 0..NUM_STATES {
        if old_mask & (1 << si) == 0 { continue; }
        let (prev, curr, pair_used) = decode(si);
        let carry = prev + curr;
        let remain = k - carry;
        if remain < 0 { continue; }
        for nc in 0..=2i32 {
            let r2 = remain - nc;
            if r2 < 0 { break; }
            for pu in 0..=1i32 {
                if 3 * pu > r2 { break; }
                let r3 = r2 - 3 * pu;
                if r3 == 0 {
                    new_mask |= 1 << encode(curr, nc, pair_used);
                } else if r3 == 2 && pair_used == 0 {
                    new_mask |= 1 << encode(curr, nc, 1);
                }
            }
        }
    }
    new_mask
}

fn power(mut base: i64, mut exp: i64) -> i64 {
    let mut result = 1i64;
    base = ((base % MOD) + MOD) % MOD;
    while exp > 0 {
        if exp & 1 == 1 { result = (result as i128 * base as i128 % MOD as i128) as i64; }
        base = (base as i128 * base as i128 % MOD as i128) as i64;
        exp >>= 1;
    }
    result
}

fn inv_mod(a: i64) -> i64 { power(a, MOD - 2) }

fn interpolate(y: &[i64], npts: usize, x_start: i64, target: i64) -> i64 {
    let n = npts;
    let mut prefix = vec![0i64; n + 1];
    let mut suffix = vec![0i64; n + 1];

    prefix[0] = 1;
    for i in 0..n {
        let xi = x_start + i as i64;
        prefix[i + 1] = (prefix[i] as i128 * (((target - xi) % MOD + MOD) % MOD) as i128 % MOD as i128) as i64;
    }

    suffix[n] = 1;
    for i in (0..n).rev() {
        let xi = x_start + i as i64;
        suffix[i] = (suffix[i + 1] as i128 * (((target - xi) % MOD + MOD) % MOD) as i128 % MOD as i128) as i64;
    }

    let mut fact_inv = vec![0i64; n];
    let mut fv = 1i64;
    for i in 1..n { fv = fv * i as i64 % MOD; }
    fact_inv[n - 1] = inv_mod(fv);
    for i in (0..n - 1).rev() {
        fact_inv[i] = fact_inv[i + 1] * (i as i64 + 1) % MOD;
    }

    let mut result = 0i64;
    for i in 0..n {
        let num = (prefix[i] as i128 * suffix[i + 1] as i128 % MOD as i128) as i64;
        let mut den = (fact_inv[i] as i128 * fact_inv[n - 1 - i] as i128 % MOD as i128) as i64;
        if (n - 1 - i) % 2 == 1 { den = (MOD - den) % MOD; }
        result = (result as i128 + (y[i] % MOD) as i128 * num as i128 % MOD as i128 * den as i128 % MOD as i128) as i64 % MOD;
    }
    result
}

fn main() {
    use std::collections::HashMap;

    let max_tiles = 3 * T_VAL + 2;
    let mut f = vec![vec![vec![0i64; NPTS + 1]; T_VAL + 1]; 2];

    // Hash table entries: (tiles, mask) -> count
    type State = HashMap<(i32, u32), i64>;
    let mut state_a: State = HashMap::new();
    state_a.insert((0, 1u32 << encode(0, 0, 0)), 1);

    for step in 1..=MAX_N {
        let mut state_b: State = HashMap::new();

        for (&(tiles, mask), &count) in &state_a {
            if count == 0 { continue; }
            for k in 0..=4i32 {
                if tiles + k > max_tiles as i32 { break; }
                let nm = dfa_transition(mask, k);
                if nm != 0 {
                    *state_b.entry((tiles + k, nm)).or_insert(0) += count;
                    let e = state_b.get_mut(&(tiles + k, nm)).unwrap();
                    *e %= MOD;
                }
            }
        }

        state_a = state_b;

        if step >= START_N && step < START_N + NPTS {
            let idx = step - START_N;
            let fs0 = encode(0, 0, 0);
            let fs1 = encode(0, 0, 1);
            for (&(tiles, mask), &count) in &state_a {
                if mask & (1 << fs1) != 0 {
                    if tiles >= 2 && (tiles - 2) % 3 == 0 {
                        let t = ((tiles - 2) / 3) as usize;
                        if t <= T_VAL {
                            f[1][t][idx] = (f[1][t][idx] + count) % MOD;
                        }
                    }
                }
                if mask & (1 << fs0) != 0 {
                    if tiles % 3 == 0 {
                        let t = (tiles / 3) as usize;
                        if t <= T_VAL {
                            f[0][t][idx] = (f[0][t][idx] + count) % MOD;
                        }
                    }
                }
            }
        }
    }

    let mut g0 = vec![0i64; T_VAL + 1];
    let mut g1 = vec![0i64; T_VAL + 1];
    for t in 0..=T_VAL {
        let npts0 = (2 * t + 1).min(NPTS).max(1);
        g0[t] = interpolate(&f[0][t], npts0, START_N as i64, BIG_N);

        let npts1 = (2 * t + 3).min(NPTS).max(1);
        g1[t] = interpolate(&f[1][t], npts1, START_N as i64, BIG_N);
    }

    // Polynomial exponentiation
    let mut poly = vec![0i64; T_VAL + 1];
    let mut base = vec![0i64; T_VAL + 1];
    poly[0] = 1;
    for i in 0..=T_VAL { base[i] = g0[i]; }

    let mut exp = BIG_S - 1;
    while exp > 0 {
        if exp & 1 == 1 {
            let mut temp = vec![0i64; T_VAL + 1];
            for i in 0..=T_VAL {
                if poly[i] == 0 { continue; }
                for j in 0..=T_VAL - i {
                    temp[i + j] = (temp[i + j] as i128 + poly[i] as i128 * base[j] as i128 % MOD as i128) as i64 % MOD;
                }
            }
            poly = temp;
        }
        let mut temp = vec![0i64; T_VAL + 1];
        for i in 0..=T_VAL {
            if base[i] == 0 { continue; }
            for j in 0..=T_VAL - i {
                temp[i + j] = (temp[i + j] as i128 + base[i] as i128 * base[j] as i128 % MOD as i128) as i64 % MOD;
            }
        }
        base = temp;
        exp >>= 1;
    }

    let mut ans = 0i64;
    for i in 0..=T_VAL {
        let j = T_VAL - i;
        if j <= T_VAL {
            ans = (ans as i128 + poly[i] as i128 * g1[j] as i128 % MOD as i128) as i64 % MOD;
        }
    }
    ans = (ans as i128 * (BIG_S % MOD) as i128 % MOD as i128) as i64;

    println!("{}", ans);
}
