// Project Euler 355 - Maximal coprime subset sum
// Hungarian algorithm for max-weight bipartite assignment.

const N: usize = 200_000;
const SQRT_N: usize = 448;
const INF_COST: i64 = 1_000_000_000_000;

fn main() {
    // Sieve
    let mut is_prime_arr = vec![true; N + 1];
    is_prime_arr[0] = false;
    is_prime_arr[1] = false;
    {
        let mut i = 2;
        while i * i <= N {
            if is_prime_arr[i] {
                let mut j = i * i;
                while j <= N {
                    is_prime_arr[j] = false;
                    j += i;
                }
            }
            i += 1;
        }
    }

    let mut small_primes = Vec::new();
    let mut base_elements = vec![1i64];

    for i in 2..=N {
        if is_prime_arr[i] {
            if i <= SQRT_N {
                small_primes.push(i as i64);
            } else {
                base_elements.push(i as i64);
            }
        }
    }

    let n_small = small_primes.len();
    let n_base = base_elements.len();

    let base_sum: i64 = base_elements.iter().sum();

    // Build gains matrix
    let mut gains = vec![0i32; n_small * n_base];
    for i in 0..n_small {
        let p = small_primes[i];
        for j in 0..n_base {
            let q = base_elements[j];
            let mut combined = q;
            while combined * p <= N as i64 {
                combined *= p;
            }
            gains[i * n_base + j] = (combined - if q == 1 { 0 } else { q }) as i32;
        }
    }

    // Hungarian algorithm for max-weight assignment
    let row_count = n_small;
    let col_count = n_base;

    let mut u = vec![0i64; row_count + 1];
    let mut v = vec![0i64; col_count + 1];
    let mut p = vec![0usize; col_count + 1];
    let mut way = vec![0usize; col_count + 1];
    let mut minv = vec![0i64; col_count + 1];
    let mut used = vec![false; col_count + 1];

    for i in 1..=row_count {
        p[0] = i;
        let mut j0 = 0;
        for j in 0..=col_count {
            minv[j] = INF_COST;
            used[j] = false;
        }

        loop {
            used[j0] = true;
            let i0 = p[j0];
            let mut delta = INF_COST;
            let mut j1 = 0;

            for j in 1..=col_count {
                if used[j] {
                    continue;
                }
                let cur = -(gains[(i0 - 1) * n_base + (j - 1)] as i64) - u[i0] - v[j];
                if cur < minv[j] {
                    minv[j] = cur;
                    way[j] = j0;
                }
                if minv[j] < delta {
                    delta = minv[j];
                    j1 = j;
                }
            }

            for j in 0..=col_count {
                if used[j] {
                    u[p[j]] += delta;
                    v[j] -= delta;
                } else {
                    minv[j] -= delta;
                }
            }

            j0 = j1;
            if p[j0] == 0 {
                break;
            }
        }

        loop {
            let j1 = way[j0];
            p[j0] = p[j1];
            j0 = j1;
            if j0 == 0 {
                break;
            }
        }
    }

    let mut assignment_gain: i64 = 0;
    for j in 1..=col_count {
        if p[j] != 0 {
            assignment_gain += gains[(p[j] - 1) * n_base + (j - 1)] as i64;
        }
    }

    println!("{}", base_sum + assignment_gain);
}
