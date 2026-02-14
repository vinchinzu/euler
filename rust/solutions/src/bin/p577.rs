// Project Euler 577 - Counting Hexagons
//
// Find sum H(n) for n=3..12345 where H(n) = number of regular hexagons
// with all vertices on triangular grid.
//
// H(n) = sum_{k=1}^{floor((n+1)/3)} k * T(n+1-3k) where T(m) = m*(m+1)/2
// S(N) = sum_{n=3}^{N} H(n)
// = 1 * N*(N-1)*(N-2)/6  [k=1 term]
// + sum_{k=2}^{floor((N+1)/3)} k * (N+1-3k)*(N+2-3k)*(N+3-3k)/6  [k>=2 terms]

fn main() {
    let n: i128 = 12345;
    let k_max = (n + 1) / 3;

    let mut total: i128 = 0;

    // k=1 term
    let b = n - 2;
    total += b * (b + 1) * (b + 2) / 6;

    // k>=2 terms
    for k in 2..=k_max {
        let b = n + 1 - 3 * k;
        if b < 0 {
            break;
        }
        total += k * b * (b + 1) * (b + 2) / 6;
    }

    println!("{}", total);
}
