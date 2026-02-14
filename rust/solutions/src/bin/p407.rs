// Project Euler 407 - Idempotents
// For each n, find M(n) = max a in [0,n) with a^2 = a (mod n).
// Uses CRT: for coprime factorization n = d*k, compute CRT(1 mod d, 0 mod k).

const MAXN: usize = 10_000_001;

fn gcd_func(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

fn extgcd(a: i64, b: i64) -> (i64, i64, i64) {
    if b == 0 {
        return (a, 1, 0);
    }
    let (g, x1, y1) = extgcd(b, a % b);
    (g, y1, x1 - (a / b) * y1)
}

fn modinv(a: i64, m: i64) -> Option<i64> {
    let (g, x, _) = extgcd(a, m);
    if g != 1 { return None; }
    Some(((x % m) + m) % m)
}

fn main() {
    let n_max: usize = 10_000_000;

    let mut m = vec![1i64; MAXN];
    m[0] = 0;
    m[1] = 0;

    for d in 2..=n_max {
        let mut n = d as i64 * 2;
        while (n as usize) <= n_max {
            let k = n / d as i64;
            if gcd_func(d as i64, k) == 1 {
                if let Some(inv) = modinv(k, d as i64) {
                    let a = (k * inv) % n;
                    if a > m[n as usize] {
                        m[n as usize] = a;
                    }
                }
            }
            n += d as i64;
        }
    }

    let sum: i64 = m[1..=n_max].iter().sum();
    println!("{}", sum);
}
