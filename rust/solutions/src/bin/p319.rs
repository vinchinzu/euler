// Project Euler 319: Bounded Sequences
use euler_utils::mod_mul;

fn mod_pow128(mut base: i64, mut exp: i64, m: i64) -> i64 {
    if m == 1 { return 0; }
    let mut result = 1i64;
    base = ((base % m) + m) % m;
    while exp > 0 {
        if exp & 1 == 1 {
            result = mod_mul(result as u64, base as u64, m as u64) as i64;
        }
        base = mod_mul(base as u64, base as u64, m as u64) as i64;
        exp >>= 1;
    }
    result
}

fn main() {
    let n: i64 = 10_000_000_000;
    let m: i64 = 1_000_000_000;
    let m2: i64 = 2 * m;

    let mut values = Vec::new();
    let mut k = 1i64;
    while k <= n {
        let v = n / k;
        values.push(v);
        k = n / v + 1;
    }
    values.reverse();
    let nv = values.len();

    let mut sq_n = (n as f64).sqrt() as i64;
    while (sq_n + 1) * (sq_n + 1) <= n { sq_n += 1; }
    while sq_n * sq_n > n { sq_n -= 1; }

    let mut t_arr = vec![0i64; nv];

    for idx in 0..nv {
        let ni = values[idx];
        let mut l = (ni as f64).sqrt() as i64;
        while (l + 1) * (l + 1) <= ni { l += 1; }
        while l * l > ni { l -= 1; }

        let p3 = mod_pow128(3, ni + 1, m2);
        let val3 = (p3 - 1) / 2;
        let val2 = (mod_pow128(2, ni + 1, m) - 1 + m) % m;
        let mut result = ((val3 - val2) % m + m) % m;

        for kk in 2..=l {
            let nk = ni / kk;
            let idx2 = if nk <= sq_n {
                (nk - 1) as usize
            } else {
                nv - (n / nk) as usize
            };
            result = (result - t_arr[idx2] + m) % m;
        }

        let upper = if l > 0 { ni / l } else { 1 };
        for q in 1..upper {
            let count = ni / q - ni / (q + 1);
            let sub = (count % m) * t_arr[(q - 1) as usize] % m;
            result = (result - sub + m) % m;
        }

        t_arr[idx] = result;
    }

    println!("{}", t_arr[nv - 1]);
}
