// Project Euler Problem 936 - Peerless Trees
// P(n) = number of peerless trees on n unlabelled vertices.
// S(N) = sum P(n) for n=3..N.
// 2D generating functions with incremental correction polynomials.

const MAX_N: usize = 50;
const DIM: usize = MAX_N + 1;

type Poly = [[i64; DIM]; DIM];

#[inline(always)]
fn mul_inv(p: &mut Poly, s: usize, count: i64, xmax: usize, ymax: usize) {
    if count == 0 || s > xmax {
        return;
    }
    let mut max_j = xmax / s;
    if max_j > ymax {
        max_j = ymax;
    }
    if max_j == 0 {
        return;
    }

    // (1 - x^s y)^{-1}: one forward unbounded-knapsack pass.
    if count == 1 {
        for x in s..=xmax {
            let y_hi = ymax.min(x);
            for y in 1..=y_hi {
                p[x][y] += p[x - s][y - 1];
            }
        }
        return;
    }

    // Small multiplicity: repeat the count==1 recurrence.
    if count > 0 && count <= 8 {
        for _ in 0..count {
            for x in s..=xmax {
                let y_hi = ymax.min(x);
                for y in 1..=y_hi {
                    p[x][y] += p[x - s][y - 1];
                }
            }
        }
        return;
    }

    if max_j == 1 {
        for x in (s..=xmax).rev() {
            let y_hi = ymax.min(x);
            for y in (1..=y_hi).rev() {
                p[x][y] += count * p[x - s][y - 1];
            }
        }
        return;
    }

    // cs[j] = C(count + j - 1, j)
    let mut cs = [0i64; DIM];
    let mut bin = count;
    cs[1] = bin;
    for j in 2..=max_j {
        bin = bin * (count + j as i64 - 1) / j as i64;
        cs[j] = bin;
    }

    for x in (s..=xmax).rev() {
        let mjx = (x / s).min(max_j);
        let y_hi = ymax.min(x);
        for y in (1..=y_hi).rev() {
            let mj = mjx.min(y);
            let mut term = 0i64;
            for j in 1..=mj {
                term += p[x - j * s][y - j] * cs[j];
            }
            p[x][y] += term;
        }
    }
}

#[inline(always)]
fn mul_factor(p: &mut Poly, s: usize, count: i64, xmax: usize, ymax: usize) {
    if count == 0 || s > xmax {
        return;
    }
    let mut max_j = xmax / s;
    if max_j > ymax {
        max_j = ymax;
    }
    if (count as usize) < max_j {
        max_j = count as usize;
    }
    if max_j == 0 {
        return;
    }

    // (1 - x^s y): reverse subtract of the shifted poly.
    if count == 1 {
        for x in (s..=xmax).rev() {
            let y_hi = ymax.min(x);
            for y in (1..=y_hi).rev() {
                p[x][y] -= p[x - s][y - 1];
            }
        }
        return;
    }

    if max_j == 1 {
        for x in (s..=xmax).rev() {
            let y_hi = ymax.min(x);
            for y in (1..=y_hi).rev() {
                p[x][y] -= count * p[x - s][y - 1];
            }
        }
        return;
    }

    if count > 0 && count <= 8 {
        for _ in 0..count {
            for x in (s..=xmax).rev() {
                let y_hi = ymax.min(x);
                for y in (1..=y_hi).rev() {
                    p[x][y] -= p[x - s][y - 1];
                }
            }
        }
        return;
    }

    // cs[j] = (-1)^j * C(count, j)
    let mut cs = [0i64; DIM];
    let mut bin = count;
    cs[1] = -bin;
    for j in 2..=max_j {
        bin = bin * (count - j as i64 + 1) / j as i64;
        cs[j] = if j & 1 == 1 { -bin } else { bin };
    }

    for x in (s..=xmax).rev() {
        let mjx = (x / s).min(max_j);
        let y_hi = ymax.min(x);
        for y in (1..=y_hi).rev() {
            let mj = mjx.min(y);
            let mut term = 0i64;
            for j in 1..=mj {
                term += p[x - j * s][y - j] * cs[j];
            }
            p[x][y] += term;
        }
    }
}

#[inline(always)]
fn conv_at(g: &Poly, f: &Poly, x: usize, y: usize) -> i64 {
    let mut acc = 0i64;
    for i in 0..=x {
        let fi = &f[i];
        let gi = &g[x - i];
        for j in 0..=y {
            acc += fi[j] * gi[y - j];
        }
    }
    acc
}

fn main() {
    let mut a = [[0i64; DIM]; DIM];
    let mut g_total = [[0i64; DIM]; DIM];
    g_total[0][0] = 1;

    // F[k] = prod_{s already processed} (1 - x^s y)^{A[s][k]}
    // A[n][K] = [x^{n-1} y^K] G_total * F[K]
    let mut f = vec![[[0i64; DIM]; DIM]; DIM];
    for k in 0..DIM {
        f[k][0][0] = 1;
    }

    let mut g_small = [[0i64; DIM]; DIM];
    g_small[0][0] = 1;
    let mut f_small = vec![[[0i64; DIM]; DIM]; DIM];
    for k in 0..DIM {
        f_small[k][0][0] = 1;
    }
    let mut small_lim = 0usize;

    let mut s_total: i64 = 0;

    for n in 1..=MAX_N {
        for big_k in 0..n {
            a[n][big_k] = conv_at(&g_total, &f[big_k], n - 1, big_k);
        }

        for big_k in 0..n {
            let c = a[n][big_k];
            if c > 0 {
                mul_factor(&mut f[big_k], n, c, MAX_N, big_k);
                mul_inv(&mut g_total, n, c, MAX_N, MAX_N);
            }
        }

        if n < 3 {
            continue;
        }

        let limit_s = (n - 1) / 2;
        while small_lim < limit_s {
            small_lim += 1;
            let s = small_lim;
            for k in 0..s {
                let c = a[s][k];
                if c > 0 {
                    mul_inv(&mut g_small, s, c, MAX_N, MAX_N);
                    // P(n) convolves F_small[k] at y = k+1
                    mul_factor(&mut f_small[k], s, c, MAX_N, k + 1);
                }
            }
        }

        let mut p_n: i64 = 0;
        let xmax = n - 1;
        for big_d in 0..n {
            if big_d == 0 {
                p_n += g_small[xmax][0];
            } else {
                p_n += conv_at(&g_small, &f_small[big_d - 1], xmax, big_d);
            }
        }

        if n % 2 == 0 {
            let half = n / 2;
            for k1 in 0..half {
                let a1 = a[half][k1];
                if a1 == 0 {
                    continue;
                }
                for k2 in (k1 + 1)..half {
                    p_n += a1 * a[half][k2];
                }
            }
        }

        s_total += p_n;
    }

    println!("{}", s_total);
}
