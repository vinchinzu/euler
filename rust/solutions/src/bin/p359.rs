// Project Euler 359: Hilbert's New Hotel
// P(f, r) closed-form formulas, sum over divisor pairs of 71328803586048.

const TARGET: i64 = 71328803586048;
const MOD: i64 = 100_000_000;

fn p(f: i64, r: i64) -> i64 {
    let m2 = 2 * MOD;

    if f == 1 {
        let rm = r % m2;
        let rp1m = (r + 1) % m2;
        let prod = (rm as i128 * rp1m as i128 % m2 as i128) as i64;
        return (prod / 2) % MOD;
    }

    let fm = f % m2;
    let rm = r % m2;
    let fpr = (fm + rm) % m2;
    let sq = (fpr as i128 * fpr as i128 % m2 as i128) as i64;

    if f % 2 == 0 {
        if r % 2 == 1 {
            let val = (sq - 2 * fm - rm + 4 * m2) % m2;
            (val / 2) % MOD
        } else {
            let val = (sq - rm + m2) % m2;
            (val / 2) % MOD
        }
    } else {
        if r % 2 == 1 {
            let val = (sq - 2 * fm - 3 * rm + 1 + 6 * m2) % m2;
            (val / 2) % MOD
        } else {
            let val = (sq - 4 * fm - 3 * rm + 3 + 8 * m2) % m2;
            (val / 2) % MOD
        }
    }
}

fn main() {
    let n = TARGET;
    let mut total: i64 = 0;

    let mut d: i64 = 1;
    while d * d <= n {
        if n % d == 0 {
            let f = d;
            let r = n / d;
            total = (total + p(f, r)) % MOD;
            if f != r {
                total = (total + p(r, f)) % MOD;
            }
        }
        d += 1;
    }

    println!("{}", total);
}
