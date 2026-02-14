fn main() {
    let n: i64 = 100_000;
    let l = n / 6;

    let mut ans: i64 = 0;
    let (mut old_p, mut old_q) = (0i64, 1i64);
    let (mut p, mut q) = (1i64, l);

    while p <= q {
        let k_big = n / (p + q);
        let limit_k = (p * k_big) / (p + 2 * q);

        for k in 1..=limit_k {
            let max_y = (k_big - k) / 2;
            let min_y = (k * q + p - 1) / p;
            let term = max_y - min_y + 1;
            if term > 0 { ans += term; }
        }

        let med = (l + old_q) / q;
        let new_p = med * p - old_p;
        let new_q = med * q - old_q;
        old_p = p;
        old_q = q;
        p = new_p;
        q = new_q;
    }

    println!("{}", ans);
}
