// Project Euler 617 - Mirror Power Sequence
// Count (n,e)-MPS sequences where n <= 10^18

fn isqrt(n: i64) -> i64 {
    let mut x = (n as f64).sqrt() as i64;
    while x * x > n { x -= 1; }
    while (x + 1) * (x + 1) <= n { x += 1; }
    x
}

fn safe_pow(base: i64, exp: i32, limit: i64) -> i64 {
    let mut result = 1i64;
    for _ in 0..exp {
        if result > limit / base { return limit + 1; }
        result *= base;
    }
    result
}

fn main() {
    let n: i64 = 1_000_000_000_000_000_000;
    let mut ans = isqrt(n) - 2;

    let mut a0 = 2i64;
    loop {
        let a0_cubed = safe_pow(a0, 3, n);
        if a0_cubed + a0 > n { break; }

        let mut e = 2;
        loop {
            let a0_e = safe_pow(a0, e, n);
            if a0_e + a0 > n { break; }

            let mut as_list = Vec::new();
            let mut a = a0;
            loop {
                let ae = safe_pow(a, e, n);
                if ae > n { break; }
                as_list.push(a);
                a = ae;
                if as_list.len() >= 100 { break; }
            }

            for start in 0..as_list.len() {
                for end in start..as_list.len() {
                    let ae = safe_pow(as_list[end], e, n);
                    if ae > n { break; }
                    if (e > 2 || end > 0) && ae + as_list[start] <= n {
                        ans += if start == 0 { end as i64 + 1 } else { 1 };
                    }
                }
            }
            e += 1;
        }
        a0 += 1;
    }

    println!("{}", ans);
}
