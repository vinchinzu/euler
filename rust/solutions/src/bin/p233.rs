// Project Euler 233 - Lattice points on a circle
// Find sum of all n <= 10^11 with exactly 420 lattice points.

fn main() {
    let n_val: i64 = 100_000_000_000;
    let plimit: usize = 5_000_000;
    let max_mult: usize = 3_000_000;

    // Sieve
    let mut is_prime_arr = vec![true; plimit + 1];
    is_prime_arr[0] = false;
    if plimit >= 1 { is_prime_arr[1] = false; }
    let mut i = 2;
    while i * i <= plimit {
        if is_prime_arr[i] {
            let mut j = i * i;
            while j <= plimit { is_prime_arr[j] = false; j += i; }
        }
        i += 1;
    }

    let primes_1mod4: Vec<i64> = (2..=plimit)
        .filter(|&i| is_prime_arr[i] && i % 4 == 1)
        .map(|i| i as i64)
        .collect();
    let p1_count = primes_1mod4.len();

    // Build valid sums (numbers with no prime factor ≡ 1 mod 4)
    let mut is_valid = vec![true; max_mult + 1];
    is_valid[0] = false;
    for i in 2..=plimit.min(max_mult) {
        if is_prime_arr[i] && i % 4 == 1 {
            let mut j = i;
            while j <= max_mult { is_valid[j] = false; j += i; }
        }
    }

    let mut valid_sum = vec![0i64; max_mult + 1];
    let mut v: i64 = 0;
    for m in 1..=max_mult {
        if is_valid[m] { v += m as i64; }
        valid_sum[m] = v;
    }

    let s = |l: i64| -> i64 {
        if l <= 0 { return 0; }
        let l = l.min(max_mult as i64) as usize;
        valid_sum[l]
    };

    let mut total: i64 = 0;

    // Pattern (1,2,3): three distinct primes ≡ 1 mod 4, all 6 permutations
    let perms3: [[i32; 3]; 6] = [[1,2,3],[1,3,2],[2,1,3],[2,3,1],[3,1,2],[3,2,1]];
    for perm in &perms3 {
        let (ep, eq, er) = (perm[0], perm[1], perm[2]);
        for ip in 0..p1_count {
            let p = primes_1mod4[ip];
            let mut pp = 1i64;
            for _ in 0..ep { pp = pp.saturating_mul(p); }
            if pp >= n_val { break; }
            let remain_p = n_val / pp;

            for iq in (ip + 1)..p1_count {
                let q = primes_1mod4[iq];
                let mut qq = 1i64;
                for _ in 0..eq { qq = qq.saturating_mul(q); }
                if qq > remain_p { break; }
                let remain_pq = remain_p / qq;

                let max_r: i64 = if er == 1 {
                    remain_pq
                } else if er == 2 {
                    (remain_pq as f64).sqrt() as i64
                } else {
                    let mut mr = (remain_pq as f64).cbrt() as i64;
                    while (mr + 1) * (mr + 1) * (mr + 1) <= remain_pq { mr += 1; }
                    while mr > 0 && mr * mr * mr > remain_pq { mr -= 1; }
                    mr
                };

                if max_r <= primes_1mod4[iq] { continue; }

                let end_ir = primes_1mod4[(iq+1)..].partition_point(|&x| x <= max_r) + iq + 1;

                for ir in (iq + 1)..end_ir {
                    let r = primes_1mod4[ir];
                    let mut rr = 1i64;
                    for _ in 0..er { rr = rr.saturating_mul(r); }
                    let core = pp.saturating_mul(qq).saturating_mul(rr);
                    if core > n_val { break; }
                    let l = n_val / core;
                    if l <= 0 { break; }
                    total += core * s(l);
                }
            }
        }
    }

    // Pattern (2,10)
    let perms2a: [[i32; 2]; 2] = [[2, 10], [10, 2]];
    for perm in &perms2a {
        let (ep, eq) = (perm[0], perm[1]);
        for ip in 0..p1_count {
            let p = primes_1mod4[ip];
            let mut pp = 1i64;
            let mut overflow = false;
            for _ in 0..ep {
                pp = pp.saturating_mul(p);
                if pp > n_val { overflow = true; break; }
            }
            if overflow || pp >= n_val { break; }
            let remain = n_val / pp;

            for iq in (ip + 1)..p1_count {
                let q = primes_1mod4[iq];
                let mut qq = 1i64;
                let mut ov = false;
                for _ in 0..eq {
                    qq = qq.saturating_mul(q);
                    if qq > remain { ov = true; break; }
                }
                if ov || qq > remain { break; }
                let core = pp * qq;
                total += core * s(n_val / core);
            }
        }
    }

    // Pattern (3,7)
    let perms2b: [[i32; 2]; 2] = [[3, 7], [7, 3]];
    for perm in &perms2b {
        let (ep, eq) = (perm[0], perm[1]);
        for ip in 0..p1_count {
            let p = primes_1mod4[ip];
            let mut pp = 1i64;
            let mut overflow = false;
            for _ in 0..ep {
                pp = pp.saturating_mul(p);
                if pp > n_val { overflow = true; break; }
            }
            if overflow || pp >= n_val { break; }
            let remain = n_val / pp;

            for iq in (ip + 1)..p1_count {
                let q = primes_1mod4[iq];
                let mut qq = 1i64;
                let mut ov = false;
                for _ in 0..eq {
                    qq = qq.saturating_mul(q);
                    if qq > remain { ov = true; break; }
                }
                if ov || qq > remain { break; }
                let core = pp * qq;
                total += core * s(n_val / core);
            }
        }
    }

    println!("{}", total);
}
