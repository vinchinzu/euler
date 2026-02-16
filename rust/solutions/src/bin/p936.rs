// Project Euler Problem 936 - Peerless Trees
// P(n) = number of peerless trees on n unlabelled vertices.
// S(N) = sum P(n) for n=3..N.
// Uses polynomial (2D generating function) approach.
// MAX_N = 50.

const MAX_N: usize = 50;

fn binom(n: i64, k: i64) -> i64 {
    if k < 0 || k > n {
        return 0;
    }
    if k == 0 || k == n {
        return 1;
    }
    let k = k.min(n - k);
    let mut res: i64 = 1;
    for i in 0..k {
        res = res * (n - i) / (i + 1);
    }
    res
}

// 2D polynomial: coeffs[x][y], x=0..MAX_N, y=0..MAX_N
type Poly = Box<[[i64; MAX_N + 1]; MAX_N + 1]>;

fn poly_new() -> Poly {
    let mut p: Poly = Box::new([[0i64; MAX_N + 1]; MAX_N + 1]);
    p[0][0] = 1;
    p
}

fn poly_zero() -> Poly {
    Box::new([[0i64; MAX_N + 1]; MAX_N + 1])
}

// Multiply by (1 - x^s * y)^(-count)
fn poly_multiply_by_inv_factor(p: &mut Poly, s: usize, count: i64) {
    let max_j = MAX_N / s;

    for x in (0..=MAX_N).rev() {
        for y in (0..=MAX_N).rev() {
            let mut term: i64 = 0;
            for j in 1..=max_j {
                if x < j * s || y < j {
                    break;
                }
                let px = x - j * s;
                let py = y - j;
                let c = binom(count + j as i64 - 1, j as i64);
                term += p[px][py] * c;
            }
            p[x][y] += term;
        }
    }
}

// Multiply by (1 - x^s * y)^count
fn poly_multiply_by_factor(p: &mut Poly, s: usize, count: i64) {
    let mut max_j = MAX_N / s;
    if max_j > count as usize {
        max_j = count as usize;
    }

    for x in (0..=MAX_N).rev() {
        for y in (0..=MAX_N).rev() {
            let mut term: i64 = 0;
            for j in 1..=max_j {
                if x < j * s || y < j {
                    break;
                }
                let px = x - j * s;
                let py = y - j;
                let c = binom(count, j as i64);
                if j % 2 == 1 {
                    term -= p[px][py] * c;
                } else {
                    term += p[px][py] * c;
                }
            }
            p[x][y] += term;
        }
    }
}

fn main() {
    let mut a = [[0i64; MAX_N + 1]; MAX_N + 1]; // A[n][k] = rooted trees of size n, root degree k
    let mut g_total = poly_new();

    let mut s_total: i64 = 0;

    for n in 1..=MAX_N {
        // Step 1: Compute A[n][K] for all K
        for big_k in 0..n {
            // Find forbidden items: trees of size s < n with root degree K
            let mut forb_s: Vec<usize> = Vec::new();
            let mut forb_count: Vec<i64> = Vec::new();

            for s in 1..n {
                if a[s][big_k] > 0 {
                    forb_s.push(s);
                    forb_count.push(a[s][big_k]);
                }
            }

            if forb_s.is_empty() {
                a[n][big_k] = g_total[n - 1][big_k];
            } else {
                let mut temp = poly_zero();
                for x in 0..=MAX_N {
                    for y in 0..=MAX_N {
                        temp[x][y] = g_total[x][y];
                    }
                }
                for i in 0..forb_s.len() {
                    poly_multiply_by_factor(&mut temp, forb_s[i], forb_count[i]);
                }
                a[n][big_k] = temp[n - 1][big_k];
            }
        }

        // Step 2: Add newly computed A[n][K] trees to G_total
        for big_k in 0..n {
            if a[n][big_k] > 0 {
                poly_multiply_by_inv_factor(&mut g_total, n, a[n][big_k]);
            }
        }

        // Step 3: Compute P(n)
        if n < 3 {
            continue;
        }

        let mut p_n: i64 = 0;

        // Case 1: Single Centroid
        let limit_s = (n - 1) / 2;
        let mut g_small = poly_new();

        for s in 1..=limit_s {
            for k in 0..s {
                if a[s][k] > 0 {
                    poly_multiply_by_inv_factor(&mut g_small, s, a[s][k]);
                }
            }
        }

        for big_d in 0..n {
            let forbidden_k: i64 = big_d as i64 - 1;
            let mut f2_s: Vec<usize> = Vec::new();
            let mut f2_count: Vec<i64> = Vec::new();

            if forbidden_k >= 0 {
                let fk = forbidden_k as usize;
                for s in 1..=limit_s {
                    if a[s][fk] > 0 {
                        f2_s.push(s);
                        f2_count.push(a[s][fk]);
                    }
                }
            }

            let val: i64;
            if f2_s.is_empty() {
                val = g_small[n - 1][big_d];
            } else {
                let mut temp2 = poly_zero();
                for x in 0..=MAX_N {
                    for y in 0..=MAX_N {
                        temp2[x][y] = g_small[x][y];
                    }
                }
                for i in 0..f2_s.len() {
                    poly_multiply_by_factor(&mut temp2, f2_s[i], f2_count[i]);
                }
                val = temp2[n - 1][big_d];
            }
            p_n += val;
        }

        // Case 2: Bicentroid (only if n is even)
        if n % 2 == 0 {
            let half = n / 2;
            for k1 in 0..=MAX_N {
                if a[half][k1] == 0 {
                    continue;
                }
                for k2 in (k1 + 1)..=MAX_N {
                    if a[half][k2] == 0 {
                        continue;
                    }
                    p_n += a[half][k1] * a[half][k2];
                }
            }
        }

        s_total += p_n;
    }

    println!("{}", s_total);
}
