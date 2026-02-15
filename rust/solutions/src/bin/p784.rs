// Project Euler 784 - Reciprocal Pairs
// Factor p^2-1 via SPF sieve, DFS to enumerate divisors up to limit.

const N: usize = 2_000_000;

static mut SPF: [i32; N + 2] = [0i32; N + 2];

fn compute_spf() {
    unsafe {
        for i in 0..=N + 1 { SPF[i] = i as i32; }
        let mut i = 2;
        while (i as u64) * (i as u64) <= (N + 1) as u64 {
            if SPF[i] == i as i32 {
                let mut j = i * i;
                while j <= N + 1 {
                    if SPF[j] == j as i32 { SPF[j] = i as i32; }
                    j += i;
                }
            }
            i += 1;
        }
    }
}

fn dfs(f: &[(i64, i32)], idx: usize, d: i64, m: i64, limit: i64, ans: &mut u64) {
    if d > limit { return; }
    if idx == f.len() {
        *ans += (m / d) as u64;
        return;
    }
    let (pp, e) = f[idx];
    let mut dd = d;
    for _ in 0..=e {
        if dd > limit { break; }
        dfs(f, idx + 1, dd, m, limit, ans);
        dd *= pp;
    }
}

fn main() {
    compute_spf();

    let mut ans: u64 = 0;

    for p in 2..=N as i64 {
        let m = p * p - 1;
        let limit = (p - 1) / 2;

        let mut factors: Vec<(i64, i32)> = Vec::new();
        let mut pm = (p - 1) as i32;
        let mut pp = (p + 1) as i32;

        let tc = pm.trailing_zeros() as i32;
        pm >>= tc;
        let tc2 = pp.trailing_zeros() as i32;
        pp >>= tc2;
        let tc_total = tc + tc2;
        if tc_total > 0 {
            factors.push((2, tc_total));
        }

        while pm > 1 {
            let q = unsafe { SPF[pm as usize] };
            let mut e = 0;
            while pm % q == 0 { pm /= q; e += 1; }
            factors.push((q as i64, e));
        }
        while pp > 1 {
            let q = unsafe { SPF[pp as usize] };
            let mut e = 0;
            while pp % q == 0 { pp /= q; e += 1; }
            factors.push((q as i64, e));
        }

        // Sort by decreasing prime for better pruning
        factors.sort_by(|a, b| b.0.cmp(&a.0));

        let mut la: u64 = 0;
        dfs(&factors, 0, 1, m, limit, &mut la);
        ans += la;
    }

    println!("{}", ans);
}
