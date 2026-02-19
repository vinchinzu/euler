// Project Euler 319: Bounded Sequences
use euler_utils::mod_pow;

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

        let p3 = mod_pow(3, (ni + 1) as u64, m2 as u64) as i64;
        let val3 = (p3 - 1) / 2;
        let val2 = (mod_pow(2, (ni + 1) as u64, m as u64) as i64 - 1 + m) % m;
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
