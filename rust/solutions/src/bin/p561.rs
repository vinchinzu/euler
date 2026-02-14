// Project Euler 561 - Divisor Pairs
//
// S((2*3*...*p_K)^n) = tr(n+1)^K - (n+1)^K
// Sum E(K,n) for n=1..N where K=904961, N=10^12.
// The answer reduces to (K+1) * sum of floor(N/4 / 2^i) for i=0,1,...

fn main() {
    let n_val: i64 = 1_000_000_000_000;
    let k: i64 = 904961;

    let mut ans: i64 = 0;
    let mut n = n_val / 4;
    while n > 0 {
        ans += (k + 1) * n;
        n /= 2;
    }

    println!("{}", ans);
}
