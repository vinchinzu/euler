// Find the correct doubling formulas
// S_k(2m) = a * S_{k+1}(m) + b * S_k(m) + ...?

fn main() {
    let limit = 200;
    let mut p = vec![0i64; limit + 1];
    p[0] = 1;
    for n in 1..=limit {
        if n % 2 == 1 {
            p[n] = p[n - 1];
        } else {
            p[n] = p[n - 1] + p[n / 2];
        }
    }

    let k_max = 8;
    let mut s = vec![vec![0i64; limit + 1]; k_max + 1];
    for n in 0..=limit {
        s[0][n] = p[n];
    }
    for k in 1..=k_max {
        let mut cumsum = 0i64;
        for n in 0..=limit {
            cumsum += s[k - 1][n];
            s[k][n] = cumsum;
        }
    }

    // For each k, try to express S_k(2m) as linear combination of S_0(m)..S_{k+1}(m)
    // Using several test points to solve the system
    for k in 0..=6 {
        println!("k={}: ", k);
        for m in 0..=10 {
            print!("S_{}({}): lhs={}, ", k, 2*m, s[k][2*m]);
            for j in 0..=k+2 {
                print!("S_{}({})={} ", j, m, s[j][m]);
            }
            println!();
        }
        println!();
    }
}
