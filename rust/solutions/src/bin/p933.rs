// Project Euler 933 - Paper Cutting
//
// D(W,H) = sum_{w=2..W} sum_{h=2..H} C(w,h)
// C(w,h) = # winning first-player moves on w x h paper (Sprague-Grundy game).
// Find D(123, 1234567).
//
// Key insight: G(w,h) stabilizes to constant G_inf(w) for h >= H_w.
// Once stable, C(w,h) is linear in h, allowing analytic summation.

fn main() {
    let w_max: usize = 123;
    let h_target: usize = 1234567;

    // Phase 1: Compute Grundy values.
    // Need a large enough h_budget so all w up to 123 stabilize.
    // Start with 2000 and verify.
    let h_budget: usize = 2000;

    let mut g: Vec<Vec<u8>> = vec![vec![0u8; h_budget + 1]; w_max + 1];

    eprintln!("Computing Grundy values up to w={}, h={}...", w_max, h_budget);
    for h in 2..=h_budget {
        for w in 2..=w_max {
            let mut seen = [false; 256];
            for a in 1..w {
                for b in 1..h {
                    let xor = g[a][b] ^ g[a][h - b] ^ g[w - a][b] ^ g[w - a][h - b];
                    seen[xor as usize] = true;
                }
            }
            let mut mex = 0u8;
            while seen[mex as usize] { mex += 1; }
            g[w][h] = mex;
        }
        if h % 500 == 0 { eprintln!("  h={}", h); }
    }

    // Find stabilization: require at least 200 consecutive equal values at the end
    let required_run = 200;
    let mut h_stable = vec![0usize; w_max + 1];
    let mut g_inf = vec![0u8; w_max + 1];
    let mut max_h_stable = 0usize;

    for w in 2..=w_max {
        let val = g[w][h_budget];
        g_inf[w] = val;
        let mut first = h_budget;
        for h in (2..=h_budget).rev() {
            if g[w][h] == val { first = h; } else { break; }
        }
        h_stable[w] = first;
        let run = h_budget - first + 1;
        if run < required_run {
            eprintln!("WARNING: w={} only has run of {} at end (val={})", w, run, val);
        }
        if first > max_h_stable { max_h_stable = first; }
    }

    eprintln!("Max stabilization point: {} (need {}+ run at end)", max_h_stable, required_run);

    // H_global: the max of all h_stable values
    let h_global = max_h_stable;

    // Phase 2: Compute D using transient C values + linear formula.
    let mut total_d: i64 = 0;

    for w in 2..=w_max {
        // Precompute L_a(b) = G(a,b) ^ G(w-a,b) for each a=1..w-1
        // L_inf(a) = G_inf(a) ^ G_inf(w-a)
        let num_a = w; // a ranges from 1 to w-1

        let mut l_inf_a = vec![0u8; num_a];
        for a in 1..w {
            l_inf_a[a] = g_inf[a] ^ g_inf[w - a];
        }

        // low_a[a] = #{b in [1, h_global-1] : L_a(b) = L_inf(a)}
        let mut sum_low_a: i64 = 0;
        for a in 1..w {
            let li = l_inf_a[a];
            let mut cnt = 0i64;
            for b in 1..h_global {
                if (g[a][b] ^ g[w - a][b]) == li {
                    cnt += 1;
                }
            }
            sum_low_a += cnt;
        }

        // For h >= 2*h_global:
        // C(w, h) = (w-1)*(h - 2*h_global + 1) + 2*sum_low_a
        let k_w: i64 = 2 * sum_low_a - (w as i64 - 1) * (2 * h_global as i64 - 1);

        // Linear regime: h from h0 = 2*h_global to h_target
        let h0 = 2 * h_global;
        if h0 <= h_target {
            let n_terms = (h_target - h0 + 1) as i64;
            let sum_h = (h_target as i64 + h0 as i64) * n_terms / 2;
            total_d += (w as i64 - 1) * sum_h + k_w * n_terms;
        }

        // Transient regime: h from 2 to min(2*h_global - 1, h_target)
        let h_direct_max = std::cmp::min(2 * h_global - 1, h_target);
        for h in 2..=h_direct_max {
            let mut c = 0i64;
            for a in 1..w {
                for b in 1..h {
                    let lb = g[a][b] ^ g[w - a][b];
                    let lhb = g[a][h - b] ^ g[w - a][h - b];
                    if lb == lhb { c += 1; }
                }
            }
            total_d += c;
        }

        if w <= 5 || w % 20 == 0 || w == w_max {
            eprintln!("w={}: h_stable={}, g_inf={}, sum_low={}, k_w={}",
                     w, h_stable[w], g_inf[w], sum_low_a, k_w);
        }
    }

    eprintln!("D(123, 1234567) = {}", total_d);
    println!("{}", total_d);
}
