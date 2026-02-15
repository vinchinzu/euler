// Project Euler 604 - Convex path in square
// Maximum lattice points on increasing strictly convex function in N x N square

fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 { let t = b; b = a % b; a = t; }
    a
}

fn main() {
    let n: i64 = 1_000_000_000_000_000_000;
    let limit = 3_000_000usize;

    let mut phi = vec![0i64; limit + 1];
    for i in 0..=limit { phi[i] = i as i64; }
    for i in 2..=limit {
        if phi[i] == i as i64 {
            for j in (i..=limit).step_by(i) {
                phi[j] -= phi[j] / i as i64;
            }
        }
    }

    let mut ans: i64 = 1;
    let mut width: i64 = 0;
    let mut k: usize = 2;

    while width + (k as i64) * phi[k] / 2 <= n {
        ans += phi[k];
        width += (k as i64) * phi[k] / 2;
        k += 1;
    }

    let num_additions = (n - width) / k as i64;
    ans += 2 * num_additions;
    width += k as i64 * num_additions;

    let mut found = false;
    let mut kk = k as i64;
    while !found && width + (kk + 1) / 2 <= n {
        for big in (kk + 1) / 2..=(n - width) {
            if gcd(kk, big) == 1 {
                ans += 1;
                found = true;
                break;
            }
        }
        kk += 1;
    }

    println!("{}", ans);
}
