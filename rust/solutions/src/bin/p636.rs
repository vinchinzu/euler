// Project Euler 636 - Restricted Factorisations
// Partition enumeration + coin-change DP over prime exponents of N!

const N_FACT: usize = 1_000_000;
const MOD: i64 = 1_000_000_007;

fn power(mut base: i64, mut exp: i64, m: i64) -> i64 {
    let mut r = 1i64;
    base = ((base % m) + m) % m;
    while exp > 0 {
        if exp & 1 == 1 { r = (r as i128 * base as i128 % m as i128) as i64; }
        base = (base as i128 * base as i128 % m as i128) as i64;
        exp >>= 1;
    }
    r
}

fn factorial_val(n: usize) -> i64 { (2..=n as i64).product::<i64>() }

type GC = [i32; 4];

fn init_group_comps() -> Vec<GC> {
    let mut gcs = Vec::new();
    for n1 in 0..=1i32 {
        for n2 in 0..=2 {
            for n3 in 0..=3 {
                for n4 in 0..=4 {
                    if n1 + n2 + n3 + n4 > 0 {
                        gcs.push([n1, n2, n3, n4]);
                    }
                }
            }
        }
    }
    gcs
}

fn multinomial(n: usize, groups: &[i32]) -> i64 {
    let mut r = factorial_val(n);
    for &g in groups { r /= factorial_val(g as usize); }
    r
}

#[derive(Clone)]
struct Profile { jumps: Vec<i32>, coeff: i64 }

fn main() {
    let gcs = init_group_comps();

    // Enumerate partitions of (1,2,3,4) into groups
    let mut all_parts: Vec<Vec<GC>> = Vec::new();

    fn enum_parts(rem: [i32; 4], min_idx: usize, cur: &mut Vec<GC>, gcs: &[GC], all_parts: &mut Vec<Vec<GC>>) {
        if rem == [0, 0, 0, 0] {
            let mut p = cur.clone();
            p.sort();
            if !all_parts.contains(&p) { all_parts.push(p); }
            return;
        }
        for idx in min_idx..gcs.len() {
            let gc = &gcs[idx];
            if gc[0] <= rem[0] && gc[1] <= rem[1] && gc[2] <= rem[2] && gc[3] <= rem[3] {
                let new_rem = [rem[0]-gc[0], rem[1]-gc[1], rem[2]-gc[2], rem[3]-gc[3]];
                cur.push(*gc);
                enum_parts(new_rem, idx, cur, gcs, all_parts);
                cur.pop();
            }
        }
    }

    let mut cur = Vec::new();
    enum_parts([1, 2, 3, 4], 0, &mut cur, &gcs, &mut all_parts);

    // Build jump profiles
    let mut jump_map: Vec<(Vec<i32>, i64)> = Vec::new();
    for p in &all_parts {
        let m = p.len();
        let sign: i64 = if (10 - m) % 2 == 0 { 1 } else { -1 };
        let bf: i64 = p.iter().map(|g| factorial_val((g[0]+g[1]+g[2]+g[3]) as usize - 1)).product();

        let mut ways = 1i64;
        let tot = [1, 2, 3, 4];
        for t in 0..4 {
            let gs: Vec<i32> = p.iter().map(|g| g[t]).collect();
            ways *= multinomial(tot[t] as usize, &gs);
        }

        // Divide by count! for each distinct group
        let mut i = 0;
        while i < m {
            let mut cnt = 1;
            while i + cnt < m && p[i + cnt] == p[i] { cnt += 1; }
            ways /= factorial_val(cnt);
            i += cnt;
        }

        let coeff = sign * bf * ways;
        let mut jumps: Vec<i32> = p.iter().map(|g| g[0] + 2*g[1] + 3*g[2] + 4*g[3]).collect();
        jumps.sort();

        if let Some(pos) = jump_map.iter().position(|(k, _)| k == &jumps) {
            jump_map[pos].1 += coeff;
        } else {
            jump_map.push((jumps, coeff));
        }
    }

    let profiles: Vec<Profile> = jump_map.into_iter()
        .filter(|(_, c)| *c != 0)
        .map(|(j, c)| Profile { jumps: j, coeff: c })
        .collect();

    // Sieve primes, compute exponents in N_FACT!
    let mut is_prime = vec![true; N_FACT + 1];
    is_prime[0] = false; is_prime[1] = false;
    let mut i = 2;
    while i * i <= N_FACT { if is_prime[i] { let mut j = i*i; while j <= N_FACT { is_prime[j] = false; j += i; } } i += 1; }

    let mut max_e = 0usize;
    let mut exp_count: Vec<u32> = Vec::new();

    for p in 2..=N_FACT {
        if !is_prime[p] { continue; }
        let mut e = 0usize;
        let mut pk = p as u64;
        while pk <= N_FACT as u64 {
            e += N_FACT / pk as usize;
            if pk > N_FACT as u64 / p as u64 { break; }
            pk *= p as u64;
        }
        if e > max_e { max_e = e; }
        if e >= exp_count.len() { exp_count.resize(e + 1, 0); }
        exp_count[e] += 1;
    }

    let distinct: Vec<(usize, u32)> = (1..=max_e).filter(|&e| e < exp_count.len() && exp_count[e] > 0)
        .map(|e| (e, exp_count[e])).collect();

    // For each profile, coin-change DP
    let mut dp = vec![0u32; max_e + 1];
    let m_val = MOD as u32;
    let mut answer = 0i64;

    for prof in &profiles {
        for v in dp.iter_mut() { *v = 0; }
        dp[0] = 1;
        for &c in &prof.jumps {
            let c = c as usize;
            for i in c..=max_e {
                let v = dp[i] + dp[i - c];
                dp[i] = if v >= m_val { v - m_val } else { v };
            }
        }
        let mut prod = 1i64;
        for &(e, mult) in &distinct {
            prod = (prod as i128 * power(dp[e] as i64, mult as i64, MOD) as i128 % MOD as i128) as i64;
        }
        let c = ((prof.coeff % MOD) + MOD) % MOD;
        answer = (answer + (c as i128 * prod as i128 % MOD as i128) as i64) % MOD;
    }

    // Divide by 1!*2!*3!*4! = 288
    answer = (answer as i128 * power(288, MOD - 2, MOD) as i128 % MOD as i128) as i64;
    println!("{}", answer);
}
