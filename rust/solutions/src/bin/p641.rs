// Project Euler 641 - A Long Row of Dice
// Count numbers <= 10^18 with exactly 6k divisors using Lucy prime counting

const K: i64 = 6;

fn isqrt(n: i64) -> i64 {
    let mut x = (n as f64).sqrt() as i64;
    while x > 0 && x * x > n { x -= 1; }
    while (x + 1) * (x + 1) <= n { x += 1; }
    x
}

fn nthrt(n: i64, r: i32) -> i64 {
    if r == 1 { return n; }
    if n <= 0 { return 0; }
    if r == 2 { return isqrt(n); }
    let dr = 1.0 / r as f64;
    let x = (n as f64).powf(dr) as i64;
    for t in (x.saturating_sub(2)..=x + 2).rev() {
        if t <= 0 { continue; }
        let mut pw = 1i64;
        let mut ok = true;
        for _ in 0..r {
            if pw > n / t + 1 { ok = false; break; }
            pw = pw.saturating_mul(t);
            if pw > n { ok = false; break; }
        }
        if ok && pw <= n {
            let mut pw2 = 1i64;
            let mut over = false;
            for _ in 0..r {
                if pw2 > n / (t + 1) + 1 { over = true; break; }
                pw2 = pw2.saturating_mul(t + 1);
                if pw2 > n { over = true; break; }
            }
            if over { return t; }
        }
    }
    x
}

fn safe_pow(p: i64, e: i32) -> i64 {
    let mut result = 1i64;
    for _ in 0..e {
        if result > 1_000_000_000_000_000_000 / p { return -1; }
        result *= p;
    }
    result
}

fn main() {
    let n_global: i64 = 1_000_000_000_000_000_000;

    let l_val = {
        let mut lv = (n_global as f64).powf(0.4) as i64;
        while safe_pow(lv + 1, 5) > 0 && safe_pow(lv + 1, 5) <= n_global { lv += 1; }
        lv as usize
    };

    // Sieve small primes
    let mut is_p = vec![true; l_val + 1];
    is_p[0] = false; if l_val >= 1 { is_p[1] = false; }
    let mut i = 2;
    while i * i <= l_val { if is_p[i] { let mut j = i*i; while j <= l_val { is_p[j] = false; j += i; } } i += 1; }
    let primes: Vec<i64> = (2..=l_val).filter(|&i| is_p[i]).map(|i| i as i64).collect();
    let mut pc_arr = vec![0i64; l_val + 1];
    let mut cnt = 0i64;
    for i in 0..=l_val { if i >= 2 && is_p[i] { cnt += 1; } pc_arr[i] = cnt; }

    // Lucy prime counting
    let r = isqrt(n_global) as usize;
    let mut v_arr = Vec::with_capacity(2 * r + 2);
    for i in 1..=r { v_arr.push(n_global / i as i64); }
    let last = *v_arr.last().unwrap();
    for v in (1..last).rev() { v_arr.push(v); }
    let v_len = v_arr.len();
    let mut s_arr = vec![0i64; v_len];
    for i in 0..v_len { s_arr[i] = v_arr[i] - 1; }

    let val_to_idx = |v: i64| -> usize {
        if v <= v_arr[v_len - 1] { v_len - v as usize }
        else { (n_global / v) as usize - 1 }
    };

    for p in 2..=r {
        let pidx = val_to_idx(p as i64);
        let pm1idx = val_to_idx(p as i64 - 1);
        if s_arr[pidx] <= s_arr[pm1idx] { continue; }
        let sp = s_arr[pm1idx];
        let p2 = p as i64 * p as i64;
        for i in 0..v_len {
            if v_arr[i] < p2 { break; }
            let vi_p_idx = val_to_idx(v_arr[i] / p as i64);
            s_arr[i] -= s_arr[vi_p_idx] - sp;
        }
    }

    let get_pc = |v: i64| -> i64 { if v < 2 { 0 } else { s_arr[val_to_idx(v)] } };

    // DFS
    let mut ans = 1i64; // count 1 which has 1 divisor (6*0 divisors case for k=0, but problem says 6k for k>=1... let's keep C logic)

    struct Frame { min_index: usize, n: i64, num_divisors: i64 }
    let mut stack = vec![Frame { min_index: 0, n: 1, num_divisors: 1 }];

    while let Some(f) = stack.pop() {
        // Leaf: count primes p where p^(e/2) <= N/n
        let mut e = if f.num_divisors % K == 1 { K } else { K - 2 };
        loop {
            let half_e = e / 2;
            let bound = nthrt(n_global / f.n, half_e as i32);
            if f.min_index >= primes.len() || bound < primes[f.min_index] { break; }
            let cnt = if bound > l_val as i64 { get_pc(bound) } else { pc_arr[bound as usize] }
                - f.min_index as i64;
            ans += cnt;
            e += K;
        }

        // Recurse with higher prime powers
        for index in f.min_index..primes.len() {
            let p = primes[index];
            let pk2 = safe_pow(p, (K - 2) as i32);
            if pk2 < 0 || f.n > n_global / pk2 { break; }

            for start_e_idx in 0..2 {
                let se = if start_e_idx == 0 { K - 2 } else { K };
                let mut ee = se;
                let half_se = ee / 2;
                let ppow = safe_pow(p, half_se as i32);
                if ppow < 0 || f.n > n_global / ppow { continue; }
                let mut new_n = f.n * ppow;
                while new_n < n_global {
                    stack.push(Frame { min_index: index as usize + 1, n: new_n, num_divisors: f.num_divisors * (ee + 1) });
                    ee += K;
                    let pmul = safe_pow(p, (K / 2) as i32);
                    if pmul < 0 || new_n > n_global / pmul { break; }
                    new_n *= pmul;
                }
            }
        }
    }

    println!("{}", ans);
}
