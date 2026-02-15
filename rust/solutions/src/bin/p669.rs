// Project Euler 669 - The King's Banquet
// Fibonacci numbers and modular arithmetic with u128.

fn main() {
    let n: u128 = 99_194_853_094_755_497;
    let k: u128 = 10_000_000_000_000_000;
    let mut fibs = vec![0u128; 200];
    fibs[0] = 1; fibs[1] = 1;
    let mut nfib = 2;
    while fibs[nfib - 1] < n {
        fibs[nfib] = fibs[nfib - 1] + fibs[nfib - 2];
        nfib += 1;
    }
    let a = if fibs[nfib - 1] >= n { fibs[nfib - 2] } else { fibs[nfib - 1] };
    let ans = if (n - k) % 2 == 0 {
        let d = (n - k) / 2;
        let da = d * a;
        let rem = da % n;
        if rem == 0 { 0 } else { n - rem }
    } else {
        let d = (n + 1 - k) / 2;
        (d * a) % n
    };
    println!("{}", ans);
}
