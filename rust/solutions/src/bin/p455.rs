// Project Euler 455: Powers with trailing digits
fn pow_mod(mut base: i64, mut exp: i64, modulus: i64) -> i64 {
    let mut result = 1i64;
    base = base.rem_euclid(modulus);
    while exp > 0 {
        if exp & 1 == 1 {
            result = (result as u128 * base as u128 % modulus as u128) as i64;
        }
        base = (base as u128 * base as u128 % modulus as u128) as i64;
        exp >>= 1;
    }
    result
}

fn main() {
    let n = 1_000_000;
    let k: i64 = 1_000_000_000;
    let mut ans: i64 = 0;

    for n_val in 2..=n {
        if n_val % 10 == 0 { continue; }
        let mut f: i64 = 2;
        loop {
            let nf = pow_mod(n_val, f, k);
            if nf == f { break; }
            f = nf;
        }
        ans += f;
    }

    println!("{}", ans);
}
