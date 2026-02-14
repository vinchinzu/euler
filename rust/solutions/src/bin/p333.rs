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

    // Build predecessors
    let nterms = terms.len();
    let mut preds: Vec<Vec<usize>> = vec![Vec::new(); nterms];
    for j in 0..nterms {
        for i in 0..j {
            if terms[i].exp2 < terms[j].exp2 && terms[i].exp3 > terms[j].exp3 {
                preds[j].push(i);
            }
        }
    }

    // DP: for each term, store sparse (sum, count) pairs
    let mut dp: Vec<Vec<(usize, i32)>> = vec![Vec::new(); nterms];
    let mut counts = vec![0i32; LIMIT + 1];

    for idx in 0..nterms {
        let value = terms[idx].value;

        let mut temp = vec![0i32; LIMIT + 1];
        if value <= LIMIT {
            temp[value] = 1;
        }

        for &pred in &preds[idx] {
            for &(s, c) in &dp[pred] {
                let new_sum = s + value;
                if new_sum <= LIMIT {
                    temp[new_sum] += c;
                }
            }
        }

        let mut pairs = Vec::new();
        for s in 1..=LIMIT {
            if temp[s] > 0 {
                pairs.push((s, temp[s]));
                counts[s] += temp[s];
            }
        }
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
