// Project Euler 710 - One Million Members
//
// t(n) = 2*t(n-2) - t(n-4) + t(n-6) + 2^(n/2 - 3)
// Find smallest N > 42 such that t(N) % 1000000 == 0.

fn pow_mod(mut base: i64, mut exp: i64, m: i64) -> i64 {
    let mut result: i64 = 1;
    base %= m;
    while exp > 0 {
        if exp & 1 == 1 {
            result = result * base % m;
        }
        base = base * base % m;
        exp >>= 1;
    }
    result
}

fn main() {
    let m: i64 = 1_000_000;

    let mut ts = [0i64; 7];
    ts[0] = 0;
    ts[1] = 0;
    ts[2] = 1;
    ts[3] = 0;
    ts[4] = 2;
    ts[5] = 1;

    let mut n: usize = 6;
    loop {
        let term = (2 * ts[(n - 2) % 7] - ts[(n - 4) % 7] + ts[(n - 6) % 7]
            + pow_mod(2, n as i64 / 2 - 3, m))
            % m;
        let term = ((term % m) + m) % m;
        ts[n % 7] = term;
        if term == 0 {
            println!("{}", n);
            return;
        }
        n += 1;
    }
}
