// Project Euler 858 - G(N) computation
// Smooth numbers, prime weight recursion, bitmask DP. G(800).

const MOD: i64 = 1_000_000_007;
const N: usize = 800;
const BM_WORDS: usize = 6;

#[derive(Clone, Copy)]
struct Bitmask { w: [u64; BM_WORDS] }

impl Bitmask {
    fn zero() -> Self { Bitmask { w: [0; BM_WORDS] } }
    fn full(nbits: usize) -> Self {
        let mut b = Self::zero();
        for i in 0..nbits { b.w[i / 64] |= 1u64 << (i % 64); }
        b
    }
    fn set(&mut self, i: usize) { self.w[i / 64] |= 1u64 << (i % 64); }
    fn test(&self, i: usize) -> bool { (self.w[i / 64] >> (i % 64)) & 1 == 1 }
    fn and(a: &Bitmask, b: &Bitmask) -> Bitmask {
        let mut r = Bitmask::zero();
        for i in 0..BM_WORDS { r.w[i] = a.w[i] & b.w[i]; }
        r
    }
    fn popcount(&self) -> i32 {
        let mut c = 0;
        for i in 0..BM_WORDS { c += self.w[i].count_ones() as i32; }
        c
    }
    fn popcount_first(&self, lim: usize) -> i32 {
        let mut c = 0;
        for i in 0..lim { if self.test(i) { c += 1; } }
        c
    }
}

fn mod_pow(mut base: i64, mut exp: i64, m: i64) -> i64 {
    let mut r = 1i64;
    base = base.rem_euclid(m);
    while exp > 0 {
        if exp & 1 == 1 { r = r * base % m; }
        base = base * base % m;
        exp >>= 1;
    }
    r
}

fn modinv(a: i64) -> i64 { mod_pow(a, MOD - 2, MOD) }

fn main() {
    // Sieve
    let mut is_prime_arr = vec![true; N + 1];
    is_prime_arr[0] = false;
    is_prime_arr[1] = false;
    let mut p = 2;
    while p * p <= N { if is_prime_arr[p] { let mut i = p * p; while i <= N { is_prime_arr[i] = false; i += p; } } p += 1; }

    let sqrt_n = 28usize; // floor(sqrt(800))
    let mut small_primes: Vec<usize> = Vec::new();
    let mut large_primes: Vec<usize> = Vec::new();
    let mut all_primes: Vec<usize> = Vec::new();
    for p in 2..=N {
        if !is_prime_arr[p] { continue; }
        all_primes.push(p);
        if p <= sqrt_n { small_primes.push(p); } else { large_primes.push(p); }
    }
    let nsmall = small_primes.len();

    // Generate smooth numbers
    let mut smooths: Vec<i32> = Vec::new();
    fn gen_smooth(idx: usize, current: i64, small_primes: &[usize], n: usize, smooths: &mut Vec<i32>) {
        if idx == small_primes.len() {
            if current <= n as i64 { smooths.push(current as i32); }
            return;
        }
        let p = small_primes[idx] as i64;
        let mut pe = 1i64;
        while current * pe <= n as i64 {
            gen_smooth(idx + 1, current * pe, small_primes, n, smooths);
            pe *= p;
        }
    }
    gen_smooth(0, 1, &small_primes, N, &mut smooths);
    smooths.sort();
    let num_smooths = smooths.len();

    // W_vals
    let mut w_vals = vec![0i64; N + 1];
    for &p in &all_primes {
        let mut pe = 1i64;
        while pe * p as i64 <= N as i64 { pe *= p as i64; }
        w_vals[p] = pe;
    }

    // C = product W_p mod MOD
    let mut c_val = 1i64;
    for &p in &all_primes { c_val = c_val * w_vals[p] % MOD; }

    // Small prime options and masks
    let mut sp_masks: Vec<Vec<Bitmask>> = Vec::new();
    let mut sp_options: Vec<Vec<(usize, i64)>> = Vec::new(); // (k, weight)

    for si in 0..nsmall {
        let p = small_primes[si];
        let mut max_e = 0;
        let mut pe = 1i64;
        while pe <= N as i64 { pe *= p as i64; max_e += 1; }
        max_e -= 1;

        let mut masks = Vec::new();
        // k=0: all bits set
        masks.push(Bitmask::full(num_smooths));

        for k in 1..=max_e {
            let mut m = Bitmask::zero();
            for i in 0..num_smooths {
                let mut y = smooths[i];
                let mut v = 0;
                while y > 0 && y % p as i32 == 0 { v += 1; y /= p as i32; }
                if v < k { m.set(i); }
            }
            masks.push(m);
        }

        let mut opts: Vec<(usize, i64)> = Vec::new();
        opts.push((0, 1));
        for k in 1..=max_e {
            let phi = (mod_pow(p as i64, k as i64, MOD) - mod_pow(p as i64, k as i64 - 1, MOD) + MOD) % MOD;
            let num = (MOD - phi) % MOD;
            let w = num * modinv(w_vals[p]) % MOD;
            opts.push((k, w));
        }

        sp_masks.push(masks);
        sp_options.push(opts);
    }

    // Group lookups for large primes
    struct GroupLookup {
        limit: usize,
        table: Vec<i64>,
    }

    let mut groups: Vec<(usize, Vec<usize>)> = Vec::new(); // (K, primes)
    let mut group_map: std::collections::HashMap<usize, usize> = std::collections::HashMap::new();

    for &p in &large_primes {
        let k = N / p;
        if let Some(&gi) = group_map.get(&k) {
            groups[gi].1.push(p);
        } else {
            group_map.insert(k, groups.len());
            groups.push((k, vec![p]));
        }
    }

    let bisect_right = |val: i32| -> usize {
        let mut lo = 0usize;
        let mut hi = num_smooths;
        while lo < hi {
            let mid = (lo + hi) / 2;
            if smooths[mid] <= val { lo = mid + 1; } else { hi = mid; }
        }
        lo
    };

    let mut grp_lookups: Vec<GroupLookup> = Vec::new();
    for &(k, ref p_list) in &groups {
        let limit_idx = bisect_right(k as i32);
        let mut table = Vec::new();
        for cnt in 0..=limit_idx {
            let p2 = mod_pow(2, cnt as i64, MOD);
            let mut val = 1i64;
            for &pp in p_list {
                let phi = (pp as i64 - 1) % MOD;
                let w = (MOD - phi) % MOD * modinv(w_vals[pp]) % MOD;
                let term = (p2 + w) % MOD;
                val = val * term % MOD;
            }
            table.push(val);
        }
        grp_lookups.push(GroupLookup { limit: limit_idx, table });
    }

    // Powers of 2
    let mut pow2_all = vec![0i64; num_smooths + 1];
    for i in 0..=num_smooths { pow2_all[i] = mod_pow(2, i as i64, MOD); }

    // Recurse
    let mut total_sum = 0i64;

    fn recurse(
        idx: usize, current_mask: &Bitmask, current_weight: i64,
        sp_masks: &[Vec<Bitmask>], sp_options: &[Vec<(usize, i64)>],
        grp_lookups: &[GroupLookup], pow2_all: &[i64],
        nsmall: usize, total_sum: &mut i64,
    ) {
        if idx == nsmall {
            let mut term = current_weight;
            let m_count = current_mask.popcount();
            term = term * pow2_all[m_count as usize] % MOD;
            for g in grp_lookups {
                let cnt = current_mask.popcount_first(g.limit);
                term = term * g.table[cnt as usize] % MOD;
            }
            *total_sum = (*total_sum + term) % MOD;
            return;
        }
        for &(k, w) in &sp_options[idx] {
            let new_mask = Bitmask::and(current_mask, &sp_masks[idx][k]);
            let new_weight = current_weight * w % MOD;
            recurse(idx + 1, &new_mask, new_weight, sp_masks, sp_options,
                    grp_lookups, pow2_all, nsmall, total_sum);
        }
    }

    let initial_mask = Bitmask::full(num_smooths);
    recurse(0, &initial_mask, 1, &sp_masks, &sp_options, &grp_lookups, &pow2_all, nsmall, &mut total_sum);

    let ans = total_sum * c_val % MOD;
    println!("{}", ans);
}
