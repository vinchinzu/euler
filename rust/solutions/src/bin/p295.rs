// Project Euler 295: Lenticular Holes
use std::collections::HashMap;

const N_MAX: i64 = 100000;
const N2: i64 = N_MAX * N_MAX;

fn main() {
    // Stern-Brocot tree: enumerate chords and compute min_r2 per len2
    let mut cht: HashMap<i64, i64> = HashMap::new();

    let mut stack: Vec<(i32, i32, i32, i32)> = vec![(0, 1, 1, 0)];
    while let Some((p1, q1, p2, q2)) = stack.pop() {
        let p = p1 + p2;
        let q = q1 + q2;
        if p as i64 * p as i64 + q as i64 * q as i64 > 4 * N_MAX { continue; }

        if (p + q) % 2 == 0 {
            let len2 = p as i64 * p as i64 + q as i64 * q as i64;
            let f1 = p1 as i64 * p1 as i64 + q1 as i64 * q1 as i64;
            let f2 = p2 as i64 * p2 as i64 + q2 as i64 * q2 as i64;
            let min_r2 = f1 * f2 * len2 / 4;
            let entry = cht.entry(len2).or_insert(min_r2);
            if min_r2 < *entry { *entry = min_r2; }
        }

        stack.push((p1, q1, p, q));
        stack.push((p, q, p2, q2));
    }

    // Collect groups and generate (rv, gi) pairs
    let groups: Vec<(i64, i64)> = cht.into_iter().collect(); // (len2, min_r2)
    let ngroups = groups.len();

    struct RVPair { rv: i64, gi: usize }
    let mut all_pairs: Vec<RVPair> = Vec::new();
    let mut group_size = vec![0i64; ngroups];

    for (gi, &(len2, min_r2)) in groups.iter().enumerate() {
        let half_len2 = len2 / 2;
        let mut k: i64 = 0;
        loop {
            let rv = (2 * k * k + 2 * k + 1) * half_len2;
            if rv > N2 { break; }
            if rv >= min_r2 {
                all_pairs.push(RVPair { rv, gi });
                group_size[gi] += 1;
            }
            k += 1;
        }
    }

    // Naive count
    let mut naive: i64 = 0;
    for gi in 0..ngroups {
        let n = group_size[gi];
        naive += n * (n + 1) / 2;
    }

    // Sort pairs by (rv, gi)
    all_pairs.sort_by(|a, b| a.rv.cmp(&b.rv).then(a.gi.cmp(&b.gi)));

    // Inclusion-exclusion
    let mut inter2: HashMap<i64, i64> = HashMap::new();
    let mut inter3: HashMap<i64, i64> = HashMap::new();
    let mut inter4: HashMap<i64, i64> = HashMap::new();

    let mut idx = 0;
    while idx < all_pairs.len() {
        let mut j = idx;
        while j < all_pairs.len() && all_pairs[j].rv == all_pairs[idx].rv { j += 1; }

        if j - idx >= 2 {
            let mut gs: Vec<usize> = Vec::new();
            for k in idx..j {
                let gi = all_pairs[k].gi;
                if !gs.contains(&gi) { gs.push(gi); }
            }
            gs.sort();

            if gs.len() >= 2 {
                for a in 0..gs.len() {
                    for b in (a + 1)..gs.len() {
                        let pk = gs[a] as i64 * 20001 + gs[b] as i64;
                        *inter2.entry(pk).or_insert(0) += 1;
                        for c in (b + 1)..gs.len() {
                            let tk = pk * 20001 + gs[c] as i64;
                            *inter3.entry(tk).or_insert(0) += 1;
                            for d in (c + 1)..gs.len() {
                                let qk = tk * 20001 + gs[d] as i64;
                                *inter4.entry(qk).or_insert(0) += 1;
                            }
                        }
                    }
                }
            }
        }
        idx = j;
    }

    let correction = |map: &HashMap<i64, i64>| -> i64 {
        map.values().map(|&n| n * (n + 1) / 2).sum::<i64>()
    };

    let corr2 = correction(&inter2);
    let corr3 = correction(&inter3);
    let corr4 = correction(&inter4);

    println!("{}", naive - corr2 + corr3 - corr4);
}
