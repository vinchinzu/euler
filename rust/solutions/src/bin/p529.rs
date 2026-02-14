// Project Euler 529 - 10-substring-friendly Numbers
//
// DP with states (mask, s), BFS to discover states, sparse matrix-vector multiply
// to generate sequence terms, Berlekamp-Massey to find linear recurrence,
// polynomial exponentiation to evaluate at n = 10^18.

const MOD: i64 = 1_000_000_007;
const B_DIGIT: usize = 10;
const MAX_STATES: usize = 6000;
const NUM_TERMS: usize = 5600;

fn power(mut base: i64, mut exp: i64, m: i64) -> i64 {
    let mut result = 1i64;
    base = ((base % m) + m) % m;
    while exp > 0 {
        if exp & 1 == 1 { result = (result as i128 * base as i128 % m as i128) as i64; }
        base = (base as i128 * base as i128 % m as i128) as i64;
        exp >>= 1;
    }
    result
}

fn pack(mask: usize, s: usize) -> usize { mask * 11 + s }

fn berlekamp_massey(s: &[i64]) -> Vec<i64> {
    let n = s.len();
    let mut c_arr = vec![0i64; n + 1];
    let mut b_arr = vec![0i64; n + 1];
    c_arr[0] = 1;
    b_arr[0] = 1;
    let mut l_val = 0usize;
    let mut m = 1usize;
    let mut b = 1i64;

    for i in 0..n {
        let mut d = s[i];
        for j in 1..=l_val {
            d = (d + (c_arr[j] as i128 * s[i - j] as i128 % MOD as i128) as i64) % MOD;
            if d < 0 { d += MOD; }
        }

        if d == 0 {
            m += 1;
        } else if 2 * l_val <= i {
            let t_arr: Vec<i64> = c_arr.clone();
            let t_len = l_val;
            let coeff = (d as i128 * power(b, MOD - 2, MOD) as i128 % MOD as i128) as i64;
            let mut blen = 0;
            for j in (0..b_arr.len()).rev() {
                if b_arr[j] != 0 { blen = j + 1; break; }
            }
            let needed = blen + m;
            if needed > c_arr.len() { c_arr.resize(needed, 0); }
            for j in 0..blen {
                c_arr[j + m] = (c_arr[j + m] - (coeff as i128 * b_arr[j] as i128 % MOD as i128) as i64 % MOD + MOD) % MOD;
            }
            l_val = i + 1 - l_val;
            b_arr = t_arr;
            b = d;
            m = 1;
        } else {
            let coeff = (d as i128 * power(b, MOD - 2, MOD) as i128 % MOD as i128) as i64;
            let mut blen = 0;
            for j in (0..b_arr.len()).rev() {
                if b_arr[j] != 0 { blen = j + 1; break; }
            }
            let needed = blen + m;
            if needed > c_arr.len() { c_arr.resize(needed, 0); }
            for j in 0..blen {
                c_arr[j + m] = (c_arr[j + m] - (coeff as i128 * b_arr[j] as i128 % MOD as i128) as i64 % MOD + MOD) % MOD;
            }
            m += 1;
        }
    }

    let mut rec = vec![0i64; l_val];
    for i in 0..l_val {
        rec[i] = (MOD - c_arr[i + 1]) % MOD;
    }
    rec
}

fn poly_mult_mod(a: &[i64], b: &[i64], rec: &[i64]) -> Vec<i64> {
    let l = rec.len();
    let mut tmp = vec![0i64; 2 * l];

    for i in 0..l {
        if a[i] == 0 { continue; }
        for j in 0..l {
            tmp[i + j] = (tmp[i + j] + (a[i] as i128 * b[j] as i128 % MOD as i128) as i64) % MOD;
        }
    }

    for i in (l..2 * l - 1).rev() {
        if tmp[i] == 0 { continue; }
        let c = tmp[i];
        tmp[i] = 0;
        for j in 0..l {
            tmp[i - l + j] = (tmp[i - l + j] + (c as i128 * rec[l - 1 - j] as i128 % MOD as i128) as i64) % MOD;
        }
    }

    tmp.truncate(l);
    tmp
}

fn main() {
    let n_target: i64 = 1_000_000_000_000_000_000;

    // Phase 1: Discover states via BFS
    let mut state_map = vec![-1i32; 1024 * 11];
    let mut states_mask = Vec::new();
    let mut states_s = Vec::new();
    let mut queue = Vec::new();

    let init = pack(1, 0);
    state_map[init] = 0;
    states_mask.push(1usize);
    states_s.push(0usize);
    queue.push(init);
    let mut qfront = 0;

    while qfront < queue.len() {
        let p = queue[qfront];
        qfront += 1;
        let mask = p / 11;
        let s = p % 11;

        for d in 0..B_DIGIT {
            if d > B_DIGIT - s { break; }
            let new_suf = ((mask << d) & ((1 << B_DIGIT) - 1)) | 1;
            let new_s = if mask & (1 << (B_DIGIT - d)) != 0 { 0 } else { d + s };

            let np = pack(new_suf, new_s);
            if state_map[np] == -1 {
                state_map[np] = states_mask.len() as i32;
                states_mask.push(new_suf);
                states_s.push(new_s);
                if states_mask.len() >= MAX_STATES {
                    panic!("Too many states");
                }
                queue.push(np);
            }
        }
    }

    let nstates = states_mask.len();

    // Phase 2: Build transition list
    let mut trans_list = Vec::new();
    let mut trans_offset = vec![0usize; nstates + 1];

    for i in 0..nstates {
        trans_offset[i] = trans_list.len();
        let mask = states_mask[i];
        let s = states_s[i];

        for d in 0..B_DIGIT {
            if d > B_DIGIT - s { break; }
            let new_suf = ((mask << d) & ((1 << B_DIGIT) - 1)) | 1;
            let new_s = if mask & (1 << (B_DIGIT - d)) != 0 { 0 } else { d + s };
            let np = pack(new_suf, new_s);
            let j = state_map[np];
            if j >= 0 {
                trans_list.push(j as usize);
            }
        }
    }
    trans_offset[nstates] = trans_list.len();

    // Target: states with s == 0
    let target: Vec<bool> = (0..nstates).map(|i| states_s[i] == 0).collect();

    // Phase 3: Generate sequence via sparse matrix-vector multiply
    let mut cur = vec![0i64; nstates];
    let init_idx = state_map[pack(1, 0)] as usize;
    cur[init_idx] = 1;
    let mut seq = vec![0i64; NUM_TERMS];

    for t in 0..NUM_TERMS {
        let mut val = 0i64;
        for i in 0..nstates {
            if target[i] && cur[i] != 0 {
                val = (val + cur[i]) % MOD;
            }
        }
        seq[t] = val;

        let mut nxt = vec![0i64; nstates];
        for i in 0..nstates {
            if cur[i] == 0 { continue; }
            for tt in trans_offset[i]..trans_offset[i + 1] {
                let j = trans_list[tt];
                nxt[j] = (nxt[j] + cur[i]) % MOD;
            }
        }
        cur = nxt;
    }

    // Phase 4: Berlekamp-Massey
    let rec = berlekamp_massey(&seq);
    let l = rec.len();

    // Phase 5: Polynomial exponentiation
    let mut base_poly = vec![0i64; l];
    let mut result_poly = vec![0i64; l];
    if l > 1 { base_poly[1] = 1; }
    result_poly[0] = 1;

    let mut exp = n_target;
    while exp > 0 {
        if exp & 1 == 1 {
            result_poly = poly_mult_mod(&result_poly, &base_poly, &rec);
        }
        base_poly = poly_mult_mod(&base_poly, &base_poly, &rec);
        exp >>= 1;
    }

    let mut ans = 0i64;
    for i in 0..l {
        ans = (ans + (result_poly[i] as i128 * seq[i] as i128 % MOD as i128) as i64) % MOD;
    }

    // T(N) = seq[N] - 1
    ans = (ans - 1 + MOD) % MOD;
    println!("{ans}");
}
