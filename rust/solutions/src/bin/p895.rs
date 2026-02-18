// Problem 895 — Gold & Silver Coin Game II
//
// Port of the Python solution.
// We compute G(9898) mod 989898989.

fn modinv(a: i64, m: i64) -> i64 {
    let (mut x0, mut x1): (i64, i64) = (1, 0);
    let (mut aa, mut bb) = (a, m);
    while bb != 0 {
        let q = aa / bb;
        let tmp = bb;
        bb = aa - q * bb;
        aa = tmp;
        let tmp = x1;
        x1 = x0 - q * x1;
        x0 = tmp;
    }
    ((x0 % m) + m) % m
}

fn ceil_div(n: i64, d: i64) -> i64 {
    // d > 0
    -((-n).div_euclid(d))
}

fn g_mod(m: usize, modulus: i64) -> i64 {
    let md = modulus;
    let inv2 = modinv(2, md);

    // precompute pow2 and invpow2 up to m
    let mut pow2 = vec![1i64; m + 1];
    for i in 1..=m {
        pow2[i] = pow2[i - 1] * 2 % md;
    }

    let mut invpow2 = vec![1i64; m + 1];
    if m >= 1 {
        invpow2[1] = inv2;
    }
    for i in 2..=m {
        invpow2[i] = invpow2[i - 1] * inv2 % md;
    }

    // prefix sums of invpow2, a*invpow2, a^2*invpow2
    let mut p0 = vec![0i64; m + 1];
    let mut p1 = vec![0i64; m + 1];
    let mut p2 = vec![0i64; m + 1];
    for a in 1..=m {
        let w = invpow2[a];
        let a_mod = a as i64 % md;
        p0[a] = (p0[a - 1] + w) % md;
        p1[a] = (p1[a - 1] + a_mod % md * w % md) % md;
        p2[a] = (p2[a - 1] + a_mod % md * a_mod % md * w % md) % md;
    }

    // interval_sums(l, r) returns (s0, s1, s2)
    let interval_sums = |l: usize, r: usize| -> (i64, i64, i64) {
        if l > r {
            return (0, 0, 0);
        }
        let s0 = (p0[r] - p0[l - 1] + md) % md;
        let s1 = (p1[r] - p1[l - 1] + md) % md;
        let s2 = (p2[r] - p2[l - 1] + md) % md;
        (s0, s1, s2)
    };

    // sum_F_linear(alpha, beta, l, r)
    // sum_{a=l..r} F(alpha*a+beta) * invpow2[a] where F(x) = C(x+2,2)
    let sum_f_linear = |alpha: i64, beta: i64, l: usize, r: usize| -> i64 {
        if l > r {
            return 0;
        }
        let (s0, s1, s2) = interval_sums(l, r);
        let a_mod = ((alpha % md) + md) % md;
        let b_mod = ((beta % md) + md) % md;

        // inv2*(alpha^2*S2 + alpha*(2*beta+3)*S1 + (beta^2+3*beta+2)*S0)
        let term2 = a_mod * a_mod % md;
        let term1 = a_mod % md * ((2 * b_mod % md + 3) % md) % md;
        let term0 = (b_mod * b_mod % md + 3 * b_mod % md + 2) % md;

        let res = (term2 % md * s2 % md + term1 % md * s1 % md + term0 % md * s0 % md) % md;
        res * inv2 % md
    };

    let c2: [i64; 3] = [1, 2, 1]; // C(2, ca)

    // G_pq(b, s, p, q)
    let g_pq = |b: usize, s: i64, p: i64, q: i64| -> i64 {
        let a_max = b as i64 - 1;
        if a_max < 1 {
            return 0;
        }
        let mut total: i64 = 0;
        for ca in 0..=2i64 {
            let mult = c2[ca as usize];
            for cb in 0..=1i64 {
                let sign: i64 = if (ca + cb) & 1 == 1 { -1 } else { 1 };
                let coeff = sign * mult;
                let alpha = p - ca;
                let beta = (q - cb) * b as i64 - s;

                let (l, r);
                if alpha == 0 {
                    if beta < 0 {
                        continue;
                    }
                    l = 1usize;
                    r = a_max as usize;
                } else if alpha > 0 {
                    let l_calc = ceil_div(-beta, alpha);
                    l = std::cmp::max(1, l_calc) as usize;
                    r = a_max as usize;
                    if l > r {
                        continue;
                    }
                } else {
                    // alpha < 0; Python: r = beta // (-alpha) (floor division)
                    let neg_alpha = -alpha;
                    let r_calc = beta.div_euclid(neg_alpha);
                    r = std::cmp::min(a_max as usize, r_calc as usize);
                    l = 1;
                    if r_calc < 1 {
                        continue;
                    }
                }

                let val = sum_f_linear(alpha, beta, l, r);
                total = ((total + coeff * val) % md + md) % md;
            }
        }
        total
    };

    // base_weighted(b, s) -> [base[0], base[1], base[2], base[3]]
    let base_weighted = |b: usize, s: i64| -> [i64; 4] {
        // precompute all G_pq for p in 0..=2, q in 0..=1
        let mut g = [[0i64; 2]; 3];
        for p in 0..=2 {
            for q in 0..=1 {
                g[p][q] = g_pq(b, s, p as i64, q as i64);
            }
        }

        let mut base = [0i64; 4];
        for r in 0..=3usize {
            let mut acc: i64 = 0;
            for nb in 0..=1usize {
                if r < nb {
                    continue;
                }
                let ra = r - nb;
                if ra <= 2 {
                    let mult_sign = 3 * c2[ra];
                    acc = (acc + mult_sign * g[ra][nb]) % md;
                }
            }
            base[r] = acc * pow2[b - 1] % md;
        }
        base
    };

    // Case 0: 3 monochrome
    let case0 = 3 * (m as i64 % md) % md * ((m as i64 - 1) % md + md) % md % md;

    // Case 2: two mixed + one monochrome
    let mut case2: i64 = 0;
    for t in 1..m {
        let n = (m - t) as i64;
        let term = pow2[t - 1] % md * (n % md) % md * ((n - 1) % md + md) % md;
        case2 = (case2 + term) % md;
    }
    case2 = case2 * 6 % md;

    // Case 3: three mixed
    let mut case3: i64 = 0;

    // carry DP for u-bit core
    let mut cur0: Vec<i64> = vec![1]; // u=1
    let mut cur1: Vec<i64> = vec![1];

    for u in 1..m - 1 {
        // u <= m-2
        if u <= m - 2 {
            let b = m - u;

            let base_s1 = base_weighted(b, 1);
            let base_s2 = base_weighted(b, 2);

            for &(s, f, ref base) in &[(1i64, 0usize, base_s1), (2i64, 1usize, base_s2)] {
                for r in 1..=3i64 {
                    let w_target = s - r;
                    let num = w_target + u as i64 + 1 - 4 * f as i64;
                    if num & 1 != 0 {
                        continue;
                    }
                    let c_idx = num / 2;
                    if c_idx < 0 || c_idx as usize > u - 1 {
                        continue;
                    }
                    let numerator_high = if f == 0 {
                        cur0[c_idx as usize]
                    } else {
                        cur1[c_idx as usize]
                    };
                    case3 = (case3 + numerator_high % md * base[r as usize] % md) % md;
                }
            }
        }

        // update DP to u+1 (mod MOD)
        let mut nxt0 = vec![0i64; u + 1];
        let mut nxt1 = vec![0i64; u + 1];
        nxt0[0] = 3 * cur0[0] % md;
        nxt1[0] = cur0[0] % md;
        for c in 1..u {
            nxt0[c] = (3 * cur0[c] + cur1[c - 1]) % md;
            nxt1[c] = (cur0[c] + 3 * cur1[c - 1]) % md;
        }
        if u >= 1 {
            nxt0[u] = cur1[u - 1] % md;
            nxt1[u] = 3 * cur1[u - 1] % md;
        }
        cur0 = nxt0;
        cur1 = nxt1;
    }

    (case0 + case2 + case3) % md
}

fn main() {
    let m = 9898;
    let modulus = 989898989;
    let ans = g_mod(m, modulus);
    println!("{}", ans);
}
