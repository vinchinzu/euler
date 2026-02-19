// Project Euler 933 - Paper Cutting
//
// D(W,H) = sum_{w=2..W} sum_{h=2..H} C(w,h)
// C(w,h) = # winning first-player moves on w x h paper (Sprague-Grundy game).
// Find D(123, 1234567).
//
// Key insights:
// 1. G(w,h) = mex over all cuts (a,b) of G(a,b)^G(a,h-b)^G(w-a,b)^G(w-a,h-b)
// 2. C(w,h) = #{(a,b) : L_a(b) = L_a(h-b)} where L_a(b) = G(a,b)^G(w-a,b)
//    C(w,h) only depends on G values for widths < w.
// 3. G(w,h) stabilizes to G_inf(w) for h >= H_stab(w), so L_a also stabilizes.
// 4. Rewrite sum over h as: S(a,c,H) = |{(b1,b2): b1,b2>=1, b1+b2<=H, L(b1)=L(b2)}|
//    For large H, this decomposes into prefix (transient) and tail (stabilized) parts
//    with a closed-form formula, avoiding enumeration over all h.

fn main() {
    let w_max: usize = 123;
    let h_target: usize = 1234567;

    // Phase 1: Compute Grundy values G(w,h) for w=2..123, h until stabilization.
    // Empirically, max stabilization point for w<=123 is ~3320.
    let h_budget: usize = 5000;
    let stride = h_budget + 1;

    // Flat contiguous array for cache-friendly access: g[w * stride + h]
    let mut g = vec![0u8; (w_max + 1) * stride];

    let mut h_stab = vec![0usize; w_max + 1];
    let mut g_inf = vec![0u8; w_max + 1];

    // Base cases: G(0,h) = G(1,h) = 0, stabilized from h=1
    h_stab[0] = 1;
    h_stab[1] = 1;

    for w in 2..=w_max {
        for h in 2..=h_budget {
            let mut seen = [false; 256];
            for a in 1..w {
                let wa = w - a;
                for b in 1..h {
                    let hb = h - b;
                    // SAFETY: a,wa < w_max+1 and b,hb <= h_budget, so indices are in bounds
                    unsafe {
                        let xor = *g.get_unchecked(a * stride + b)
                            ^ *g.get_unchecked(a * stride + hb)
                            ^ *g.get_unchecked(wa * stride + b)
                            ^ *g.get_unchecked(wa * stride + hb);
                        *seen.get_unchecked_mut(xor as usize) = true;
                    }
                }
            }
            let mut mex = 0u8;
            while seen[mex as usize] {
                mex += 1;
            }
            // SAFETY: w * stride + h is in bounds
            unsafe {
                *g.get_unchecked_mut(w * stride + h) = mex;
            }
        }

        // Find stabilization: longest constant run at the end of the computed range
        let val = g[w * stride + h_budget];
        let mut first = h_budget;
        for h in (2..=h_budget).rev() {
            if g[w * stride + h] == val {
                first = h;
            } else {
                break;
            }
        }
        h_stab[w] = first;
        g_inf[w] = val;
    }

    // Phase 2: Compute D(W,H) using the analytical pair-counting formula.
    //
    // D(W,H) = sum_{w=2}^W sum_{a=1}^{w-1} S(a, w-a, H)
    //
    // S(a, c, H) = |{(b1,b2): 1<=b1, 1<=b2, b1+b2<=H, L(b1)=L(b2)}|
    // where L(b) = G(a,b) ^ G(c,b), stabilizing to l_inf for b >= b_stab.
    //
    // Decompose b-values into:
    //   Prefix: b in [1, b_stab-1]  (L may vary)
    //   Tail:   b in [b_stab, H-1]  (L = l_inf)
    //
    // For H >> b_stab (which holds since H=1234567 >> max b_stab ~ 3320):
    //   S = sum_{v != l_inf} count_v^2          (prefix-prefix, same non-stable value)
    //     + p^2                                   (prefix-prefix, stable value)
    //     + 2*(p*(H - bs + 1) - sum_P)           (prefix-tail cross terms)
    //     + M*(M+1)/2                             (tail-tail, M = H - 2*bs + 1)
    // where p = #{b in prefix : L(b) = l_inf}, sum_P = sum of those b values,
    //       count_v = #{b in prefix : L(b) = v}, bs = b_stab.

    let mut total_d: i64 = 0;

    for w in 2..=w_max {
        for a in 1..w {
            let c = w - a;
            // b_stab: smallest b such that L(b') = l_inf for all b' >= b_stab.
            // Must be >= 1 since b indices start at 1.
            let b_stab = std::cmp::max(std::cmp::max(h_stab[a], h_stab[c]), 1);
            let l_inf = g_inf[a] ^ g_inf[c];

            // Count prefix values and sum of b-positions with L(b) = l_inf
            let mut count_map = [0i64; 256];
            let mut prefix_b_sum_linf: i64 = 0;
            for b in 1..b_stab {
                // SAFETY: a,c <= w_max and b < b_stab <= h_budget
                let lb = unsafe {
                    *g.get_unchecked(a * stride + b) ^ *g.get_unchecked(c * stride + b)
                };
                count_map[lb as usize] += 1;
                if lb == l_inf {
                    prefix_b_sum_linf += b as i64;
                }
            }

            let p = count_map[l_inf as usize];
            let sum_p = prefix_b_sum_linf;
            let h = h_target as i64;
            let bs = b_stab as i64;

            // Contribution from non-l_inf prefix pairs (all pairs satisfy b1+b2 <= H)
            let mut s: i64 = 0;
            for v in 0..256u16 {
                if v as u8 != l_inf {
                    let cnt = count_map[v as usize];
                    s += cnt * cnt;
                }
            }

            // l_inf contributions
            s += p * p; // prefix-prefix
            s += 2 * (p * (h - bs + 1) - sum_p); // prefix-tail cross
            let m = h - 2 * bs + 1;
            if m > 0 {
                s += m * (m + 1) / 2; // tail-tail
            }

            total_d += s;
        }
    }

    println!("{}", total_d);
}
