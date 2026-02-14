// Project Euler 425 - Prime connection
// A prime P is 2's relative if there's a chain of connected primes from 2 to P,
// none exceeding P. Uses Union-Find.

const N: usize = 10_000_000;

static mut PARENT: [u32; N + 1] = [0; N + 1];
static mut SZ: [u32; N + 1] = [0; N + 1];

fn find(mut x: usize) -> usize {
    unsafe {
        while PARENT[x] as usize != x {
            PARENT[x] = PARENT[PARENT[x] as usize];
            x = PARENT[x] as usize;
        }
    }
    x
}

fn unite(a: usize, b: usize) {
    let mut ra = find(a);
    let mut rb = find(b);
    if ra == rb { return; }
    unsafe {
        if SZ[ra] < SZ[rb] { std::mem::swap(&mut ra, &mut rb); }
        PARENT[rb] = ra as u32;
        SZ[ra] += SZ[rb];
    }
}

fn main() {
    // Sieve
    let mut is_prime = vec![true; N + 1];
    is_prime[0] = false;
    is_prime[1] = false;
    let mut i = 2;
    while i * i <= N {
        if is_prime[i] {
            let mut j = i * i;
            while j <= N {
                is_prime[j] = false;
                j += i;
            }
        }
        i += 1;
    }

    let mut primes = Vec::new();
    for i in 2..=N {
        if is_prime[i] { primes.push(i); }
    }

    // Init union-find
    unsafe {
        for i in 0..=N {
            PARENT[i] = i as u32;
            SZ[i] = 1;
        }
    }

    // Powers of 10
    let mut pow10 = [1usize; 8];
    for i in 1..8 { pow10[i] = pow10[i - 1] * 10; }

    let mut ans: i64 = 0;
    for &p in &primes {
        // Get digits
        let mut digits = [0usize; 8];
        let mut n_digits = 0;
        let mut tmp = p;
        while tmp > 0 {
            digits[n_digits] = tmp % 10;
            n_digits += 1;
            tmp /= 10;
        }

        for i in 0..n_digits {
            let d = digits[i];
            for sub in 1..=d {
                let relative = p - sub * pow10[i];
                if relative >= 2
                    && (n_digits <= 1 || relative >= pow10[n_digits - 1] / 10)
                    && is_prime[relative]
                {
                    unite(p, relative);
                }
            }
        }

        if find(2) != find(p) {
            ans += p as i64;
        }
    }

    println!("{}", ans);
}
