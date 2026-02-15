// Project Euler 154: Exploring Pascal's pyramid
const N_VAL: usize = 200000;
const K: i32 = 12;

fn main() {
    let mut f = vec![0i32; N_VAL + 1]; // v5(i!) cumulative
    let mut t = vec![0i32; N_VAL + 1]; // v2(i!) cumulative
    for i in 1..=N_VAL {
        let mut v5 = 0;
        let mut v2 = 0;
        let mut n = i;
        while n % 5 == 0 { v5 += 1; n /= 5; }
        n = i;
        while n % 2 == 0 { v2 += 1; n /= 2; }
        f[i] = f[i - 1] + v5;
        t[i] = t[i - 1] + v2;
    }

    let fn_val = f[N_VAL];
    let tn_val = t[N_VAL];
    let mut ans: i64 = 0;

    let fp = f.as_ptr();
    let tp = t.as_ptr();

    for a in 0..=N_VAL / 3 {
        let temp_f = unsafe { *fp.add(a) } + K - fn_val;
        let temp_t = unsafe { *tp.add(a) } + K - tn_val;

        let b_lo = a + 1;
        let b_hi = if a < N_VAL { (N_VAL - a - 1) / 2 } else { 0 };
        if b_lo > b_hi { continue; }

        for b in b_lo..=b_hi {
            let c = N_VAL - a - b;
            unsafe {
                let d5 = *fp.add(b) + *fp.add(c) + temp_f;
                let d2 = *tp.add(b) + *tp.add(c) + temp_t;
                if d5 <= 0 && d2 <= 0 {
                    ans += 6;
                }
            }
        }

        // Case a == b: c = N - 2a
        {
            let c = N_VAL - 2 * a;
            if c > a {
                let d5 = unsafe { *fp.add(a) + *fp.add(c) } + temp_f;
                let d2 = unsafe { *tp.add(a) + *tp.add(c) } + temp_t;
                if d5 <= 0 && d2 <= 0 {
                    ans += 3;
                }
            }
        }

        // Case b == c: b = c = (N-a)/2
        if (N_VAL - a) % 2 == 0 {
            let half = (N_VAL - a) / 2;
            if half > a {
                let d5 = 2 * unsafe { *fp.add(half) } + temp_f;
                let d2 = 2 * unsafe { *tp.add(half) } + temp_t;
                if d5 <= 0 && d2 <= 0 {
                    ans += 3;
                }
            }
        }
    }

    println!("{ans}");
}
