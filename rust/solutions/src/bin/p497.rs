// Project Euler 497: Drunken Tower of Hanoi
fn powmod(mut base: i64, mut exp: i64, modulus: i64) -> i64 {
    let mut result = 1i64;
    base = base.rem_euclid(modulus);
    while exp > 0 {
        if exp & 1 == 1 {
            result = result * base % modulus;
        }
        base = base * base % modulus;
        exp >>= 1;
    }
    result
}

fn main() {
    let n_max = 10000;
    let m: i64 = 1_000_000_000;

    let mut nm1 = [[[[0i64; 3]; 3]; 3]; 3];
    for start in 0..3usize {
        for end in 0..3usize {
            if start != end {
                nm1[start][end][start][end] = 1;
            }
        }
    }

    let mut prev = nm1;
    let mut ans: i64 = 0;

    for n in 1..=n_max {
        let cur: [[[[i64; 3]; 3]; 3]; 3];
        if n == 1 {
            cur = nm1;
        } else {
            let mut c = [[[[0i64; 3]; 3]; 3]; 3];
            for start in 0..3usize {
                for end in 0..3usize {
                    if start == end { continue; }
                    let other = 3 - start - end;
                    for s in 0..3usize {
                        for e in 0..3usize {
                            c[start][end][s][e] = (c[start][end][s][e] + prev[start][other][s][e]) % m;
                        }
                    }
                    c[start][end][other][start] = (c[start][end][other][start] + 1) % m;
                    c[start][end][start][end] = (c[start][end][start][end] + 1) % m;
                    c[start][end][end][other] = (c[start][end][end][other] + 1) % m;
                    for s in 0..3usize {
                        for e in 0..3usize {
                            c[start][end][s][e] = (c[start][end][s][e] + prev[other][end][s][e]) % m;
                        }
                    }
                }
            }
            cur = c;
        }

        let k = powmod(10, n, m);
        let rods = [powmod(3, n, m), powmod(6, n, m), powmod(9, n, m)];

        for s in 0..3usize {
            for e in 0..3usize {
                let dist;
                if s < e {
                    let re = (rods[e] - 1 + m) % m;
                    let rs = (rods[s] - 1 + m) % m;
                    dist = (re % m * (re % m) % m + m - rs % m * (rs % m) % m) % m;
                } else {
                    let re = (k - rods[e] + m) % m;
                    let rs = (k - rods[s] + m) % m;
                    dist = (re % m * (re % m) % m + m - rs % m * (rs % m) % m) % m;
                }
                let count = (nm1[1][0][s][e] + cur[0][2][s][e]) % m;
                ans = (ans + dist * count) % m;
            }
        }

        prev = cur;
    }

    println!("{}", ans % m);
}
