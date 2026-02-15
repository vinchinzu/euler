// Project Euler 338: Cutting Rectangles

const M: i64 = 100_000_000; // 10^8

fn isqrt(n: i64) -> i64 {
    if n < 0 { return 0; }
    if n < 2 { return n; }
    let mut x = n;
    let mut y = (x + 1) / 2;
    while y < x {
        x = y;
        y = (x + n / x) / 2;
    }
    x
}

fn sum_floor_quotients(m: i64) -> i64 {
    let s = isqrt(m);
    let mut result: i64 = 0;
    for k in 1..=s {
        result = (result + m / k) % M;
    }
    result = ((2 * result - (s % M) * (s % M) % M) % M + M) % M;
    result
}

fn num_triplets_mod(n: i64) -> i64 {
    let mut total: i64 = 0;
    let mut a: i64 = 1;
    while a <= n {
        let v = n / a;
        let a_end = n / v;
        let count = (a_end - a + 1) % M;
        let dfq = sum_floor_quotients(v);
        total = (total + (count as i128 * dfq as i128 % M as i128) as i64) % M;
        a = a_end + 1;
    }
    total
}

fn main() {
    let n: i64 = 1_000_000_000_000; // 10^12
    let l = isqrt(n);

    let mut ans: i64 = 0;

    // Part 1: for k = 2 to L
    for k in 2..=l {
        let nk = n / k % M;
        let nkm1 = n / (k - 1) % M;
        ans = (ans + (nk as i128 * nkm1 as i128 % M as i128) as i64) % M;
    }

    // For k > L, group by t = floor(N/k)
    for t in 1..(n / l) {
        let block = n / t - n / (t + 1);
        let val = ((((block - 1) % M) as i128 * ((t % M) as i128 * (t % M) as i128 % M as i128) % M as i128
            + (t % M) as i128 * ((t + 1) % M) as i128 % M as i128) % M as i128 + M as i128) % M as i128;
        ans = (ans + val as i64) % M;
    }

    let triplets = num_triplets_mod(n);
    let sfq = sum_floor_quotients(n);

    let result = ((ans - triplets + sfq) % M + 2 * M) % M;
    println!("{}", result);
}
