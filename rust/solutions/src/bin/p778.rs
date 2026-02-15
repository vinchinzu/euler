// Project Euler 778 - Per-Digit Mod Product
// Matrix exponentiation on 10x10 transition matrices per digit position.

fn main() {
    const B: usize = 10;
    const MOD: i64 = 1_000_000_009;

    let n: i64 = 765432;
    let k: i64 = 234567;

    type Mat = [[i64; B]; B];

    fn mat_mul(a: &Mat, b: &Mat) -> Mat {
        let mut t = [[0i64; B]; B];
        for i in 0..B {
            for j in 0..B {
                let mut acc = 0i128;
                for kk in 0..B {
                    acc += a[i][kk] as i128 * b[kk][j] as i128;
                }
                t[i][j] = (acc % MOD as i128) as i64;
            }
        }
        t
    }

    fn mat_pow(base: &Mat, mut exp: i64) -> Mat {
        let mut res = [[0i64; B]; B];
        for i in 0..B { res[i][i] = 1; }
        let mut b = *base;
        while exp > 0 {
            if exp & 1 == 1 { res = mat_mul(&res, &b); }
            b = mat_mul(&b, &b);
            exp >>= 1;
        }
        res
    }

    let mut ans: i64 = 0;
    let mut pow_b: i64 = 1;

    while pow_b <= n {
        let mut counts = [0i64; B];
        for d in 0..B as i64 {
            let digit_at_pos = (n / pow_b) % B as i64;
            let base_count = (n / B as i64) / pow_b;
            let diff = d - digit_at_pos;
            if diff > 0 {
                counts[d as usize] = base_count * pow_b;
            } else if diff == 0 {
                counts[d as usize] = base_count * pow_b + (n % pow_b) + 1;
            } else {
                counts[d as usize] = (base_count + 1) * pow_b;
            }
        }

        let mut a = [[0i64; B]; B];
        for d in 0..B {
            for d2 in 0..B {
                a[(d * d2) % B][d] = (a[(d * d2) % B][d] + counts[d2]) % MOD;
            }
        }

        let a_pow = mat_pow(&a, k);

        for d in 0..B {
            ans = (ans + (a_pow[d][1] as i128 * (pow_b % MOD) as i128 % MOD as i128 * d as i128 % MOD as i128) as i64) % MOD;
        }

        pow_b *= B as i64;
    }

    println!("{}", ans);
}
