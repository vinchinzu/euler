// Project Euler 481: Chef Showdown
const MAXN: usize = 14;
const MAXMASK: usize = 1 << MAXN;

fn popcount(x: usize) -> usize {
    x.count_ones() as usize
}

fn kth_bit(mask: usize, k: usize) -> usize {
    let mut count = 0;
    for i in 0..MAXN {
        if mask & (1 << i) != 0 {
            if count == k { return i; }
            count += 1;
        }
    }
    unreachable!()
}

fn linear_solve(n: usize, a: &mut [[f64; MAXN]; MAXN], b: &mut [f64; MAXN]) {
    for col in 0..n {
        let mut pivot = col;
        for row in (col + 1)..n {
            if a[row][col].abs() > a[pivot][col].abs() { pivot = row; }
        }
        for j in 0..n { let t = a[col][j]; a[col][j] = a[pivot][j]; a[pivot][j] = t; }
        let t = b[col]; b[col] = b[pivot]; b[pivot] = t;
        for row in (col + 1)..n {
            let factor = a[row][col] / a[col][col];
            for j in col..n { a[row][j] -= factor * a[col][j]; }
            b[row] -= factor * b[col];
        }
    }
    for row in (0..n).rev() {
        for j in (row + 1)..n { b[row] -= a[row][j] * b[j]; }
        b[row] /= a[row][row];
    }
}

fn fibonacci(n: usize) -> i64 {
    if n == 0 { return 0; }
    if n == 1 { return 1; }
    let (mut a, mut b) = (0i64, 1i64);
    for _ in 2..=n { let c = a + b; a = b; b = c; }
    b
}

fn main() {
    let n = 14;

    let mut s = [0.0f64; MAXN];
    let fib_n1 = fibonacci(n + 1) as f64;
    for i in 0..n {
        s[i] = fibonacci(i + 1) as f64 / fib_n1;
    }

    // W[subset][i][j] and E[subset][i]
    let mut w = vec![vec![[0.0f64; MAXN]; MAXN]; MAXMASK];
    let mut e = vec![[0.0f64; MAXN]; MAXMASK];

    for subset in 1..(1 << n) {
        let nn = popcount(subset);
        let mut chefs = [0usize; MAXN];
        let mut ci = 0;
        for i in 0..n {
            if subset & (1 << i) != 0 { chefs[ci] = i; ci += 1; }
        }

        if nn == 1 {
            w[subset][chefs[0]][chefs[0]] = 1.0;
            continue;
        }

        let mut bs_mat = [[0.0f64; MAXN]; MAXN];
        let mut b_vec = [0.0f64; MAXN];

        for start in 0..nn {
            let mut best_prob = -1e18f64;
            let mut all_probs = [0.0f64; MAXN];
            let mut expected = 0.0f64;

            for ee in 1..nn {
                let j = (start + ee) % nn;
                let next = (start + if ee == 1 { 2 } else { 1 }) % nn;
                let new_subset = subset & !(1 << chefs[j]);

                let prob = w[new_subset][chefs[next]][chefs[start]];
                if prob > best_prob {
                    best_prob = prob;
                    for i in 0..nn {
                        all_probs[i] = w[new_subset][chefs[next]][chefs[i]];
                    }
                    expected = e[new_subset][chefs[next]];
                }
            }

            for i in 0..nn {
                bs_mat[i][start] = s[chefs[start]] * all_probs[i];
            }
            b_vec[start] = 1.0 + s[chefs[start]] * expected;
        }

        // Solve for W
        for i in 0..nn {
            let mut a = [[0.0f64; MAXN]; MAXN];
            let mut rhs = [0.0f64; MAXN];
            for start in 0..nn {
                a[start][start] = 1.0;
                a[start][(start + 1) % nn] = s[chefs[start]] - 1.0;
                rhs[start] = bs_mat[i][start];
            }
            linear_solve(nn, &mut a, &mut rhs);
            for start in 0..nn {
                w[subset][chefs[start]][chefs[i]] = rhs[start];
            }
        }

        // Solve for E
        {
            let mut a = [[0.0f64; MAXN]; MAXN];
            let mut rhs = [0.0f64; MAXN];
            for start in 0..nn {
                a[start][start] = 1.0;
                a[start][(start + 1) % nn] = s[chefs[start]] - 1.0;
                rhs[start] = b_vec[start];
            }
            linear_solve(nn, &mut a, &mut rhs);
            for start in 0..nn {
                e[subset][chefs[start]] = rhs[start];
            }
        }
    }

    let ans = e[(1 << n) - 1][0];
    println!("{:.8}", ans);
}
