// Problem 955: Finding Triangles
//
// Sequence a_0=3, a_{n+1} = a_n+1 if a_n is triangular, else 2a_n - a_{n-1} + 1.
// Between consecutive triangle hits, a_{k+j} = T + j(j+1)/2.
// Next triangle hit: find smallest j>0 such that T + j(j+1)/2 is triangular.
// If T = s(s+1)/2, need (m-j)(m+j+1) = s(s+1), i.e., factorizations uv = s(s+1)
// with v-u odd, v-u >= 3. Minimize j = (v-u-1)/2.

fn factorize(mut n: u64) -> Vec<(u64, u32)> {
    let mut factors = Vec::new();
    let mut d = 2u64;
    while d * d <= n {
        if n % d == 0 {
            let mut e = 0u32;
            while n % d == 0 {
                n /= d;
                e += 1;
            }
            factors.push((d, e));
        }
        d += 1;
    }
    if n > 1 {
        factors.push((n, 1));
    }
    factors
}

fn all_divisors(factors: &[(u64, u32)]) -> Vec<u64> {
    let mut divs = vec![1u64];
    for &(p, e) in factors {
        let len = divs.len();
        let mut pk = 1u64;
        for _ in 0..e {
            pk *= p;
            for i in 0..len {
                divs.push(divs[i] * pk);
            }
        }
    }
    divs.sort();
    divs
}


fn main() {
    // s starts at 2 (a_0 = 3 = 2*3/2)
    let mut s: u64 = 2;
    let mut index: u64 = 0;
    let target_count = 70;
    let mut tri_count = 1; // a_0 = 3 is the 1st triangle number in the sequence

    while tri_count < target_count {
        let n = (s as u128) * ((s + 1) as u128);
        // Factorize s*(s+1). Since gcd(s, s+1)=1, factorize them separately.
        let f_s = factorize(s);
        let f_s1 = factorize(s + 1);

        // Combine factors
        let mut all_factors = f_s.clone();
        all_factors.extend_from_slice(&f_s1);

        let divs = all_divisors(&all_factors);

        // Find divisor pair (u, v) with u*v = s*(s+1), v >= u, v-u odd, v-u >= 3
        // minimizing v-u.
        let n_val = n;
        let mut best_j: u64 = u64::MAX;
        let mut best_new_s: u64 = 0;

        // Iterate over divisors as u, compute v = n/u
        for &u in &divs {
            if (u as u128) * (u as u128) > n_val {
                break; // u > sqrt(n), so v < u
            }
            if n_val % (u as u128) != 0 {
                continue;
            }
            let v = (n_val / (u as u128)) as u64;
            let diff = v - u;
            if diff < 3 {
                continue; // need j >= 1
            }
            if diff % 2 == 0 {
                continue; // need v-u odd
            }
            let j = (diff - 1) / 2;
            if j < best_j {
                best_j = j;
                best_new_s = (u + v - 1) / 2; // m = (u+v-1)/2, new s = m
            }
        }

        assert!(best_j < u64::MAX, "No solution found for s={}", s);
        index += best_j;
        s = best_new_s;
        tri_count += 1;
    }

    println!("{}", index);
}
