// Project Euler 333: Special partitions
use euler_utils::sieve;

const LIMIT: usize = 1_000_000;

struct Term {
    value: usize,
    exp2: u32,
    exp3: u32,
}

fn main() {
    let is_prime = sieve(LIMIT);

    // Generate terms 2^i * 3^j
    let mut terms: Vec<Term> = Vec::new();
    let mut v2 = 1usize;
    let mut e2 = 0u32;
    while v2 <= LIMIT {
        let mut v3 = v2;
        let mut e3 = 0u32;
        while v3 <= LIMIT {
            if v3 > 1 {
                terms.push(Term { value: v3, exp2: e2, exp3: e3 });
            }
            v3 *= 3;
            e3 += 1;
        }
        v2 *= 2;
        e2 += 1;
    }
    terms.sort_by(|a, b| a.exp2.cmp(&b.exp2).then(b.exp3.cmp(&a.exp3)));

    // Build predecessors - flatten to single vec with offsets
    let nterms = terms.len();
    let mut pred_data: Vec<usize> = Vec::new();
    let mut pred_offsets: Vec<usize> = Vec::with_capacity(nterms + 1);
    pred_offsets.push(0);
    
    for j in 0..nterms {
        for i in 0..j {
            if terms[i].exp2 < terms[j].exp2 && terms[i].exp3 > terms[j].exp3 {
                pred_data.push(i);
            }
        }
        pred_offsets.push(pred_data.len());
    }

    // DP: for each term, store sparse (sum, count) pairs
    let mut dp: Vec<Vec<(usize, i32)>> = vec![Vec::new(); nterms];
    let mut counts = vec![0i32; LIMIT + 1];
    let mut temp = vec![0i32; LIMIT + 1];
    let mut active: Vec<usize> = Vec::new();

    for idx in 0..nterms {
        let value = terms[idx].value;

        if value <= LIMIT {
            temp[value] = 1;
            active.push(value);
        }

        let start = pred_offsets[idx];
        let end = pred_offsets[idx + 1];
        for &pred in &pred_data[start..end] {
            for &(s, c) in &dp[pred] {
                let new_sum = s + value;
                if new_sum <= LIMIT {
                    if temp[new_sum] == 0 {
                        active.push(new_sum);
                    }
                    temp[new_sum] += c;
                }
            }
        }

        let mut pairs = Vec::with_capacity(active.len());
        for &s in &active {
            // SAFETY: s <= LIMIT guaranteed by outer DP logic
            unsafe {
                let cnt = *temp.get_unchecked(s);
                pairs.push((s, cnt));
                *counts.get_unchecked_mut(s) += cnt;
                *temp.get_unchecked_mut(s) = 0;
            }
        }
        active.clear();
        dp[idx] = pairs;
    }

    let mut total: i64 = 0;
    for p in 2..LIMIT {
        if is_prime[p] && counts[p] == 1 {
            total += p as i64;
        }
    }

    println!("{}", total);
}
