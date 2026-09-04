// Project Euler 559 - Permutation Matrices
//
// Compute Q(50000) mod 1000000123.
// Per-k generating function is a power-series inverse of (1/(jk)!)^n.
// Small series: O(m^2) DP. Large: Newton inverse via 3-prime NTT + CRT.
// k values are independent; rayon over k with per-thread NTT workspace.

use rayon::prelude::*;

const NVAL: usize = 50_000;
const MOD: u64 = 1_000_000_123;

const P1: u64 = 998_244_353;
const P2: u64 = 985_661_441;
const P3: u64 = 754_974_721;

const MAX_LOG: usize = 17;
const MAX_NTT: usize = 1 << MAX_LOG; // 131072
const DIRECT_THRESHOLD: usize = 128;

// Precomputed CRT modular inverse constants
const INV_P1_MOD_P2: u64 = 657_107_549;
const INV_M12_MOD_P3: u64 = 284_003_040;
const M12_MOD_MOD: u64 = 424_598_615;
const P1_MOD_P3: u64 = 243_269_632;

fn pow_mod_u(mut base: u64, mut exp: u64, m: u64) -> u64 {
    let mut result = 1u64;
    base %= m;
    while exp > 0 {
        if exp & 1 == 1 {
            result = (result * base) % m;
        }
        base = (base * base) % m;
        exp >>= 1;
    }
    result
}

struct FastNtt<const P: u64> {
    sum_e: [u64; 30],
    sum_ie: [u64; 30],
    inv_pow2: [u64; MAX_LOG + 1],
}

impl<const P: u64> FastNtt<P> {
    fn new(g: u64) -> Self {
        let mut pm1 = P - 1;
        let mut v2 = 0usize;
        while pm1 & 1 == 0 {
            pm1 /= 2;
            v2 += 1;
        }

        let mut sum_e = [0u64; 30];
        let mut sum_ie = [0u64; 30];
        let mut es = [0u64; 30];
        let mut ies = [0u64; 30];
        let cnt = v2;
        let mut e = pow_mod_u(g, (P - 1) >> cnt, P);
        let mut ie = pow_mod_u(e, P - 2, P);
        for i in (0..cnt - 1).rev() {
            es[i] = e;
            ies[i] = ie;
            e = (e * e) % P;
            ie = (ie * ie) % P;
        }
        let mut now = 1u64;
        for i in 0..cnt - 2 {
            sum_e[i] = (es[i] * now) % P;
            now = (now * ies[i]) % P;
        }
        let mut inow = 1u64;
        for i in 0..cnt - 2 {
            sum_ie[i] = (ies[i] * inow) % P;
            inow = (inow * es[i]) % P;
        }

        let inv2 = pow_mod_u(2, P - 2, P);
        let mut inv_pow2 = [0u64; MAX_LOG + 1];
        inv_pow2[0] = 1;
        for k in 1..=MAX_LOG {
            inv_pow2[k] = (inv_pow2[k - 1] * inv2) % P;
        }

        FastNtt {
            sum_e,
            sum_ie,
            inv_pow2,
        }
    }

    fn butterfly(&self, a: &mut [u64]) {
        let n = a.len();
        if n <= 1 {
            return;
        }
        let h = n.trailing_zeros() as usize;
        for ph in 1..=h {
            let w = 1 << (ph - 1);
            let p = 1 << (h - ph);
            let mut now = 1u64;
            for s in 0..w {
                let offset = s << (h - ph + 1);
                for i in 0..p {
                    unsafe {
                        let l = *a.get_unchecked(i + offset);
                        let r = (*a.get_unchecked(i + offset + p) * now) % P;
                        *a.get_unchecked_mut(i + offset) = if l + r >= P { l + r - P } else { l + r };
                        *a.get_unchecked_mut(i + offset + p) = if l >= r { l - r } else { l + P - r };
                    }
                }
                let idx = (!s as u32).trailing_zeros() as usize;
                now = (now * self.sum_e[idx]) % P;
            }
        }
    }

    fn butterfly_inv(&self, a: &mut [u64]) {
        let n = a.len();
        if n <= 1 {
            return;
        }
        let h = n.trailing_zeros() as usize;
        for ph in (1..=h).rev() {
            let w = 1 << (ph - 1);
            let p = 1 << (h - ph);
            let mut inow = 1u64;
            for s in 0..w {
                let offset = s << (h - ph + 1);
                for i in 0..p {
                    unsafe {
                        let l = *a.get_unchecked(i + offset);
                        let r = *a.get_unchecked(i + offset + p);
                        *a.get_unchecked_mut(i + offset) = if l + r >= P { l + r - P } else { l + r };
                        let diff = if l >= r { l - r } else { l + P - r };
                        *a.get_unchecked_mut(i + offset + p) = (diff * inow) % P;
                    }
                }
                let idx = (!s as u32).trailing_zeros() as usize;
                inow = (inow * self.sum_ie[idx]) % P;
            }
        }
        let inv_n = self.inv_pow2[h];
        for x in a.iter_mut() {
            *x = (*x * inv_n) % P;
        }
    }
}

#[inline(always)]
fn fast_crt3(r1: u64, r2: u64, r3: u64) -> u64 {
    let r1p2 = if r1 < P2 { r1 } else { r1 - P2 };
    let diff = if r2 >= r1p2 { r2 - r1p2 } else { r2 + P2 - r1p2 };
    let k = (diff * INV_P1_MOD_P2) % P2;

    let r1p3 = if r1 < P3 { r1 } else { r1 - P3 };
    let x12_mod_p3 = (r1p3 + (k * P1_MOD_P3) % P3) % P3;
    let diff2 = if r3 >= x12_mod_p3 {
        r3 - x12_mod_p3
    } else {
        r3 + P3 - x12_mod_p3
    };
    let k2 = (diff2 * INV_M12_MOD_P3) % P3;

    let term1 = (k * P1) % MOD;
    let term2 = (k2 * M12_MOD_MOD) % MOD;
    (r1 + term1 + term2) % MOD
}

struct HeavyWorkspace {
    buf: Vec<u64>,
    bk: Vec<u64>,
    dp: Vec<u64>,
    t: Vec<u64>,
    r: Vec<u64>,
    ng: Vec<u64>,
}

impl HeavyWorkspace {
    fn new() -> Self {
        HeavyWorkspace {
            buf: vec![0u64; 6 * MAX_NTT],
            bk: vec![0u64; NVAL + 2],
            dp: vec![0u64; NVAL + 2],
            t: vec![0u64; MAX_NTT],
            r: vec![0u64; MAX_NTT],
            ng: vec![0u64; MAX_NTT],
        }
    }
}

struct LightWorkspace {
    bk: Vec<u64>,
    dp: Vec<u64>,
}

impl LightWorkspace {
    fn new() -> Self {
        LightWorkspace {
            bk: vec![0u64; DIRECT_THRESHOLD + 2],
            dp: vec![0u64; DIRECT_THRESHOLD + 2],
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn poly_mul(
    a: &[u64],
    b: &[u64],
    out_len: usize,
    ntt1: &FastNtt<P1>,
    ntt2: &FastNtt<P2>,
    ntt3: &FastNtt<P3>,
    ntt_buf: &mut [u64],
    out: &mut [u64],
) {
    let na = a.len();
    let nb = b.len();
    let need = na + nb - 1;
    let ntt_len = need.next_power_of_two();

    // Prime 1
    let (left1, right1) = ntt_buf[0..2 * MAX_NTT].split_at_mut(MAX_NTT);
    let fa1 = &mut left1[..ntt_len];
    let fb1 = &mut right1[..ntt_len];
    fa1[..na].copy_from_slice(a);
    fa1[na..].fill(0);
    fb1[..nb].copy_from_slice(b);
    fb1[nb..].fill(0);
    ntt1.butterfly(fa1);
    ntt1.butterfly(fb1);
    for i in 0..ntt_len {
        unsafe {
            *fa1.get_unchecked_mut(i) = (*fa1.get_unchecked(i) * *fb1.get_unchecked(i)) % P1;
        }
    }
    ntt1.butterfly_inv(fa1);

    // Prime 2
    let (left2, right2) = ntt_buf[2 * MAX_NTT..4 * MAX_NTT].split_at_mut(MAX_NTT);
    let fa2 = &mut left2[..ntt_len];
    let fb2 = &mut right2[..ntt_len];
    fa2[..na].copy_from_slice(a);
    fa2[na..].fill(0);
    fb2[..nb].copy_from_slice(b);
    fb2[nb..].fill(0);
    ntt2.butterfly(fa2);
    ntt2.butterfly(fb2);
    for i in 0..ntt_len {
        unsafe {
            *fa2.get_unchecked_mut(i) = (*fa2.get_unchecked(i) * *fb2.get_unchecked(i)) % P2;
        }
    }
    ntt2.butterfly_inv(fa2);

    // Prime 3
    let (left3, right3) = ntt_buf[4 * MAX_NTT..6 * MAX_NTT].split_at_mut(MAX_NTT);
    let fa3 = &mut left3[..ntt_len];
    let fb3 = &mut right3[..ntt_len];
    fa3[..na].copy_from_slice(a);
    fa3[na..].fill(0);
    fb3[..nb].copy_from_slice(b);
    fb3[nb..].fill(0);
    ntt3.butterfly(fa3);
    ntt3.butterfly(fb3);
    for i in 0..ntt_len {
        unsafe {
            *fa3.get_unchecked_mut(i) = (*fa3.get_unchecked(i) * *fb3.get_unchecked(i)) % P3;
        }
    }
    ntt3.butterfly_inv(fa3);

    let take = out_len.min(need);
    for i in 0..take {
        unsafe {
            let r1 = *ntt_buf.get_unchecked(i);
            let r2 = *ntt_buf.get_unchecked(2 * MAX_NTT + i);
            let r3 = *ntt_buf.get_unchecked(4 * MAX_NTT + i);
            *out.get_unchecked_mut(i) = fast_crt3(r1, r2, r3);
        }
    }
    if take < out_len {
        out[take..].fill(0);
    }
}

fn naive_inverse_into(bk: &[u64], q: usize, dp: &mut [u64]) {
    dp[0] = 1;
    unsafe {
        let bk_p = bk.as_ptr();
        let dp_p = dp.as_mut_ptr();
        for i in 1..=q {
            let mut acc = 0u128;
            let mut j = 1usize;
            while j + 3 <= i {
                acc += *bk_p.add(j) as u128 * *dp_p.add(i - j) as u128;
                acc += *bk_p.add(j + 1) as u128 * *dp_p.add(i - j - 1) as u128;
                acc += *bk_p.add(j + 2) as u128 * *dp_p.add(i - j - 2) as u128;
                acc += *bk_p.add(j + 3) as u128 * *dp_p.add(i - j - 3) as u128;
                j += 4;
            }
            while j <= i {
                acc += *bk_p.add(j) as u128 * *dp_p.add(i - j) as u128;
                j += 1;
            }
            let v = (acc % MOD as u128) as u64;
            *dp_p.add(i) = if v == 0 { 0 } else { MOD - v };
        }
    }
}

fn poly_inv_start_b(
    n: usize,
    b: usize,
    ntt1: &FastNtt<P1>,
    ntt2: &FastNtt<P2>,
    ntt3: &FastNtt<P3>,
    ws: &mut HeavyWorkspace,
) {
    let HeavyWorkspace {
        buf,
        bk,
        dp,
        t,
        r,
        ng,
    } = ws;
    let start_m = b.min(n);
    naive_inverse_into(&bk[..start_m], start_m - 1, &mut dp[..start_m]);
    let mut m = start_m;
    while m < n {
        let want = (2 * m).min(n);
        poly_mul(
            &bk[..want],
            &dp[..m],
            want,
            ntt1,
            ntt2,
            ntt3,
            buf,
            &mut t[..want],
        );
        r[0] = (2 + MOD - t[0]) % MOD;
        for i in 1..want {
            r[i] = if t[i] == 0 { 0 } else { MOD - t[i] };
        }
        poly_mul(
            &dp[..m],
            &r[..want],
            want,
            ntt1,
            ntt2,
            ntt3,
            buf,
            &mut ng[..want],
        );
        dp[..want].copy_from_slice(&ng[..want]);
        m = want;
    }
}

fn compute_pk_heavy(
    k: usize,
    n: usize,
    pif: &[u64],
    ntt1: &FastNtt<P1>,
    ntt2: &FastNtt<P2>,
    ntt3: &FastNtt<P3>,
    ws: &mut HeavyWorkspace,
) -> u64 {
    let q = n / k;
    let r = n % k;

    ws.bk[0] = 1;
    unsafe {
        for j in 1..=q {
            *ws.bk.get_unchecked_mut(j) = *pif.get_unchecked(j * k);
        }
    }

    poly_inv_start_b(q + 1, DIRECT_THRESHOLD, ntt1, ntt2, ntt3, ws);

    let last = if r == 0 {
        ws.dp[q]
    } else {
        let mut acc = 0u128;
        unsafe {
            for s in 0..=q {
                acc += *pif.get_unchecked(n - s * k) as u128 * *ws.dp.get_unchecked(s) as u128;
            }
        }
        let v = (acc % MOD as u128) as u64;
        if v == 0 {
            0
        } else {
            MOD - v
        }
    };

    let np = if r == 0 { q + 1 } else { q + 2 };
    if (np + 1) % 2 == 0 {
        last
    } else if last == 0 {
        0
    } else {
        MOD - last
    }
}

fn compute_pk_light(k: usize, n: usize, pif: &[u64], ws: &mut LightWorkspace) -> u64 {
    let q = n / k;
    let r = n % k;

    let bk = &mut ws.bk[..=q];
    let dp = &mut ws.dp[..=q];
    bk[0] = 1;
    unsafe {
        for j in 1..=q {
            *bk.get_unchecked_mut(j) = *pif.get_unchecked(j * k);
        }
    }

    naive_inverse_into(bk, q, dp);

    let last = if r == 0 {
        dp[q]
    } else {
        let mut acc = 0u128;
        unsafe {
            for s in 0..=q {
                acc += *pif.get_unchecked(n - s * k) as u128 * *dp.get_unchecked(s) as u128;
            }
        }
        let v = (acc % MOD as u128) as u64;
        if v == 0 {
            0
        } else {
            MOD - v
        }
    };

    let np = if r == 0 { q + 1 } else { q + 2 };
    if (np + 1) % 2 == 0 {
        last
    } else if last == 0 {
        0
    } else {
        MOD - last
    }
}

thread_local! {
    static HEAVY_WS: std::cell::RefCell<HeavyWorkspace> = std::cell::RefCell::new(HeavyWorkspace::new());
    static LIGHT_WS: std::cell::RefCell<LightWorkspace> = std::cell::RefCell::new(LightWorkspace::new());
}

fn main() {
    let n = NVAL;

    let mut factorials = vec![0u64; n + 1];
    factorials[0] = 1;
    for i in 1..=n {
        factorials[i] = factorials[i - 1] * i as u64 % MOD;
    }

    let mut inv_factorials = vec![0u64; n + 1];
    inv_factorials[n] = pow_mod_u(factorials[n], MOD - 2, MOD);
    for i in (0..n).rev() {
        inv_factorials[i] = inv_factorials[i + 1] * (i as u64 + 1) % MOD;
    }

    let mut pow_inv_fact = vec![0u64; n + 1];
    pow_inv_fact
        .par_iter_mut()
        .enumerate()
        .for_each(|(i, slot)| {
            *slot = pow_mod_u(inv_factorials[i], n as u64, MOD);
        });

    let ntt1 = FastNtt::<P1>::new(3);
    let ntt2 = FastNtt::<P2>::new(3);
    let ntt3 = FastNtt::<P3>::new(11);
    let pif = &pow_inv_fact[..];

    let k_heavy = n / DIRECT_THRESHOLD;

    let pk_sum_heavy: u64 = (1..k_heavy + 1)
        .into_par_iter()
        .with_min_len(1)
        .with_max_len(1)
        .map(|k| {
            HEAVY_WS.with(|cell| {
                let mut ws = cell.borrow_mut();
                compute_pk_heavy(k, n, pif, &ntt1, &ntt2, &ntt3, &mut ws)
            })
        })
        .sum();

    let pk_sum_light: u64 = ((k_heavy + 1)..=n)
        .into_par_iter()
        .map(|k| {
            LIGHT_WS.with(|cell| {
                let mut ws = cell.borrow_mut();
                compute_pk_light(k, n, pif, &mut ws)
            })
        })
        .sum();

    let pk_sum = (pk_sum_heavy + pk_sum_light) % MOD;
    let ans = (pk_sum % MOD) * pow_mod_u(factorials[n], n as u64, MOD) % MOD;
    println!("{ans}");
}
