// Project Euler 362: Squarefree factors
// Sum of Fsf(k) for k=1 to N=10^10.
// Uses Mobius function, square-free counting, and iterative DFS.

use std::collections::HashMap;

const N_VAL: i64 = 10_000_000_000;

fn main() {
    let l = (N_VAL as f64).sqrt() as usize;

    // Compute Mobius, is_prime, is_square_free, cumulative square-free count
    let mut mu = vec![1i8; l + 1];
    let mut is_prime_arr = vec![true; l + 1];
    let mut is_square_free = vec![true; l + 1];
    is_prime_arr[0] = false;
    is_prime_arr[1] = false;
    is_square_free[0] = false;
    is_square_free[1] = false;

    {
        let mut i = 2;
        while i * i <= l {
            if is_prime_arr[i] {
                let mut j = i * i;
                while j <= l {
                    is_prime_arr[j] = false;
                    j += i;
                }
                let sq = i * i;
                let mut j = sq;
                while j <= l {
                    mu[j] = 0;
                    is_square_free[j] = false;
                    j += sq;
                }
            }
            i += 1;
        }
    }

    for i in 2..=l {
        if mu[i] == 0 { continue; }
        if is_prime_arr[i] {
            let mut j = i;
            while j <= l {
                mu[j] = -mu[j];
                j += i;
            }
        }
    }

    // Build square-free list and cumulative count
    let mut sf_cumul = vec![0i64; l + 2];
    let mut square_frees = Vec::new();
    for i in 2..=l {
        sf_cumul[i] = sf_cumul[i - 1] + if is_square_free[i] { 1 } else { 0 };
        if is_square_free[i] {
            square_frees.push(i as i64);
        }
    }

    let count_square_free = |x: i64| -> i64 {
        if x < 1 { return 0; }
        let mut total = 0i64;
        let mut d = 1i64;
        while d * d <= x {
            if (d as usize) <= l && mu[d as usize] != 0 {
                total += mu[d as usize] as i64 * (x / (d * d));
            }
            d += 1;
        }
        total
    };

    let num_square_free_up_to = |x: i64| -> i64 {
        if x < 2 { return 0; }
        if x <= l as i64 { return sf_cumul[x as usize]; }
        count_square_free(x) - 1 // exclude 1
    };

    // Hash map for quotient values
    let mut qmap: HashMap<i64, i64> = HashMap::new();
    for k in 1..=l as i64 {
        let q = N_VAL / k;
        qmap.entry(q).or_insert_with(|| num_square_free_up_to(q));
    }

    let get_num_sf = |x: i64, qmap: &HashMap<i64, i64>| -> i64 {
        if x < 2 { return 0; }
        if x <= l as i64 { return sf_cumul[x as usize]; }
        if let Some(&v) = qmap.get(&x) { return v; }
        num_square_free_up_to(x)
    };

    let num_sf = square_frees.len();

    let find_last_index = |start_index: usize, limit: i64| -> Option<usize> {
        if start_index >= num_sf { return None; }
        if square_frees[start_index] > limit { return None; }
        let mut lo = start_index;
        let mut hi = num_sf - 1;
        while lo < hi {
            let mid = (lo + hi + 1) / 2;
            if square_frees[mid] <= limit { lo = mid; } else { hi = mid - 1; }
        }
        Some(lo)
    };

    // Iterative DFS
    struct StackEntry {
        prev_index: usize,
        prod: i64,
    }

    let mut stack: Vec<StackEntry> = Vec::with_capacity(10_000_000);
    stack.push(StackEntry { prev_index: 0, prod: 1 });

    let mut ans: i64 = 0;

    while let Some(entry) = stack.pop() {
        let prev_index = entry.prev_index;
        let prod = entry.prod;

        let max_last = N_VAL / prod;
        let min_sf = if prev_index < num_sf { square_frees[prev_index] } else { 2 };

        if max_last >= min_sf {
            let contrib = get_num_sf(max_last, &qmap) - get_num_sf(min_sf - 1, &qmap);
            ans += contrib;
        }

        let max_sf_for_next = ((N_VAL / prod) as f64).sqrt() as i64;
        if max_sf_for_next < 2 { continue; }

        if let Some(last_valid) = find_last_index(prev_index, max_sf_for_next) {
            for index in (prev_index..=last_valid).rev() {
                let sf = square_frees[index];
                let new_prod = prod * sf;
                if new_prod * sf > N_VAL { continue; }
                stack.push(StackEntry { prev_index: index, prod: new_prod });
            }
        }
    }

    println!("{}", ans);
}
