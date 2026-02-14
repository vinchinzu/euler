// Project Euler 143: Torricelli point of a triangle
// Find sum of distinct p+q+r <= 120000 for Torricelli triangles.

use std::collections::HashSet;
use euler_utils::gcd;

const L: usize = 120_000;

fn main() {
    let mut pairs_map: Vec<Vec<usize>> = vec![Vec::new(); L + 1];

    let max_m = (L as f64).sqrt() as usize + 10;

    for m in 2..=max_m {
        for n in 1..m {
            if gcd(m as u64, n as u64) != 1 { continue; }
            if (m - n) % 3 == 0 { continue; }

            let a = 2 * m * n + n * n;
            let b = m * m - n * n;

            for k in 1.. {
                let pk = k * a;
                let qk = k * b;
                if pk + qk > L { break; }

                let mn = pk.min(qk);
                let mx = pk.max(qk);

                pairs_map[mn].push(mx);
            }
        }
    }

    // Sort partner lists
    for i in 0..=L {
        pairs_map[i].sort_unstable();
    }

    let mut sums = HashSet::new();

    for p in 1..=L {
        if pairs_map[p].is_empty() { continue; }
        let qs = &pairs_map[p];

        for (qi, &q) in qs.iter().enumerate() {
            if pairs_map[q].is_empty() { continue; }

            // Intersect pairs_map[p][qi+1..] with pairs_map[q], elements > q
            let rs_p = &pairs_map[p][(qi + 1)..];
            let rs_q = &pairs_map[q];

            let mut ip = 0;
            let mut iq = 0;

            while ip < rs_p.len() && iq < rs_q.len() {
                if rs_p[ip] == rs_q[iq] {
                    let r = rs_p[ip];
                    let s = p + q + r;
                    if s <= L {
                        sums.insert(s);
                    }
                    ip += 1;
                    iq += 1;
                } else if rs_p[ip] < rs_q[iq] {
                    ip += 1;
                } else {
                    iq += 1;
                }
            }
        }
    }

    let total_sum: usize = sums.iter().sum();
    println!("{}", total_sum);
}
