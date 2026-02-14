// Project Euler 202: Laserbeam
// Inclusion-exclusion over prime factors of target.

fn main() {
    let n: i64 = 12017639147;
    let target = (n + 3) / 2;

    // Factor target
    let mut primes = Vec::new();
    let mut t = target;
    let mut d = 2i64;
    while d * d <= t {
        if t % d == 0 {
            primes.push(d);
            while t % d == 0 {
                t /= d;
            }
        }
        d += 1;
    }
    if t > 1 {
        primes.push(t);
    }

    let np = primes.len();
    let mut ans: i64 = 0;

    for subset in 0..(1u32 << np) {
        let mut prod: i64 = 1;
        let mut bits = 0;
        for i in 0..np {
            if subset & (1 << i) != 0 {
                prod *= primes[i];
                bits += 1;
            }
        }
        let contrib = (target / prod - 2) / 3;
        if bits % 2 == 0 {
            ans += contrib;
        } else {
            ans -= contrib;
        }
    }

    println!("{}", ans);
}
