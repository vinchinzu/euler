// Project Euler 648 - Skipping Squares
// Power series expansion with DP

const NN: usize = 1000;
const MOD: i64 = 1_000_000_000;

fn isqrt(n: i32) -> i32 {
    let mut r = (n as f64).sqrt() as i32;
    while r > 0 && r * r > n { r -= 1; }
    while (r + 1) * (r + 1) <= n { r += 1; }
    r
}

fn is_square(n: i32) -> bool {
    let r = isqrt(n);
    r * r == n
}

fn main() {
    let max_s = (NN as i32 / 2) * (NN as i32 / 2);

    let mut jump1 = vec![0i64; NN + 1];
    let mut jump2 = vec![0i64; NN + 1];
    let mut f = vec![0i64; NN + 1];

    for s in (0..=max_s as usize).rev() {
        for k in 0..=NN { f[k] = 0; }

        if s > 0 && is_square(s as i32) {
            f[0] = (isqrt(s as i32) - 1) as i64;
        } else {
            f[0] = jump2[0];
            for k in 1..=NN {
                f[k] = ((jump2[k] + jump1[k - 1] - jump2[k - 1]) % MOD + MOD) % MOD;
            }
        }

        jump2.copy_from_slice(&jump1);
        jump1.copy_from_slice(&f);
    }

    let mut ans = 0i64;
    for k in 0..=NN {
        ans = (ans + jump1[k]) % MOD;
    }
    println!("{}", ans);
}
