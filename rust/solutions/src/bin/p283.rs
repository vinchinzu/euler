use rayon::prelude::*;
use std::cell::RefCell;

const NN: usize = 1000;

fn build_spf(limit: usize) -> Vec<usize> {
    let mut spf = (0..=limit).collect::<Vec<_>>();
    let mut i = 2;
    while i * i <= limit {
        if spf[i] == i {
            let mut j = i * i;
            while j <= limit {
                if spf[j] == j { spf[j] = i; }
                j += i;
            }
        }
        i += 1;
    }
    spf
}

fn sq_le(val: i64, product: i64) -> bool {
    (val as i128) * (val as i128) <= product as i128
}

fn smallest_factor_large(n: i64) -> i64 {
    if n % 2 == 0 { return 2; }
    let mut i = 3i64;
    while i * i <= n {
        if n % i == 0 { return i; }
        i += 2;
    }
    n
}

thread_local! {
    static TL_BUF: RefCell<Vec<i64>> = RefCell::new(vec![0i64; 100_000]);
}

fn main() {
    let l = (2.0 * (3.0f64).sqrt() * NN as f64) as usize;
    let max_val = 4 * NN * NN + l * l;
    let spf = build_spf(max_val);

    let ans: i64 = (1..=NN).into_par_iter().map(|r| {
        TL_BUF.with(|buf| {
            let mut divisors_buf = buf.borrow_mut();
            let r2_4 = 4 * r as i64 * r as i64;

            let mut factors = Vec::new();
            {
                let mut tmp = r2_4 as usize;
                while tmp > 1 {
                    let f = spf[tmp];
                    factors.push(f);
                    while tmp % f == 0 { tmp /= f; }
                }
            }

            let mut local_ans: i64 = 0;
            for x in 1..=l as i64 {
                let product = r2_4 * (r2_4 + x * x);
                let mut k = product;
                let mut divisors_size = 1usize;
                divisors_buf[0] = 1;

                for &d in &factors {
                    let d = d as i64;
                    let mut e = 0;
                    while k % d == 0 { k /= d; e += 1; }
                    let old_size = divisors_size;
                    for i in (0..old_size).rev() {
                        let mut mult = d;
                        for _ in 0..e {
                            let val = divisors_buf[i] * mult;
                            if sq_le(val, product) {
                                divisors_buf[divisors_size] = val;
                                divisors_size += 1;
                            }
                            mult *= d;
                        }
                    }
                }

                while k > 1 {
                    let d = if (k as usize) <= max_val {
                        spf[k as usize] as i64
                    } else {
                        smallest_factor_large(k)
                    };
                    let mut e = 0;
                    while k % d == 0 { k /= d; e += 1; }
                    let old_size = divisors_size;
                    for i in (0..old_size).rev() {
                        let mut mult = d;
                        for _ in 0..e {
                            let val = divisors_buf[i] * mult;
                            if sq_le(val, product) {
                                divisors_buf[divisors_size] = val;
                                divisors_size += 1;
                            }
                            mult *= d;
                        }
                    }
                }

                for i in 0..divisors_size {
                    let xy = divisors_buf[i] + r2_4;
                    let xz = product / divisors_buf[i] + r2_4;
                    if xy % x == 0 && xz % x == 0 && x * x <= xy {
                        local_ans += 2 * (x + xy / x + xz / x);
                    }
                }
            }
            local_ans
        })
    }).sum();

    println!("{}", ans);
}
