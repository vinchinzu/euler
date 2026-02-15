// Project Euler 440: GCD and Tiling
use euler_utils::gcd;

const NN: usize = 2000;
const KK: i64 = 10;
const M: i64 = 987898789;

fn mat_mul(x: [i64; 4], y: [i64; 4]) -> [i64; 4] {
    [
        (x[0] * y[0] + x[1] * y[2]) % M,
        (x[0] * y[1] + x[1] * y[3]) % M,
        (x[2] * y[0] + x[3] * y[2]) % M,
        (x[2] * y[1] + x[3] * y[3]) % M,
    ]
}

fn mat_pow(mut base: [i64; 4], mut exp: i32) -> [i64; 4] {
    let mut result = [1, 0, 0, 1];
    while exp > 0 {
        if exp & 1 != 0 { result = mat_mul(result, base); }
        base = mat_mul(base, base);
        exp >>= 1;
    }
    result
}

fn main() {
    let mut mults = vec![0i64; NN + 1];
    for a in 1..=NN {
        for b in 1..=NN {
            let g = gcd(a as u64, b as u64) as usize;
            if (a / g) % 2 == 1 && (b / g) % 2 == 1 {
                mults[g] += 1;
            } else {
                mults[0] += 1;
            }
        }
    }

    let mut ans: i64 = 0;
    for c in 1..=NN as i32 {
        ans = (ans + mults[0] * if c % 2 == 0 { 1 } else { KK }) % M;

        let mut a = [KK, 1, 1, 0i64];
        for g in 1..=NN {
            a = mat_pow(a, c);
            ans = (ans + mults[g] * a[0]) % M;
        }
    }

    println!("{ans}");
}
