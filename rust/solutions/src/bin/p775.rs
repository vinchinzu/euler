// Project Euler 775 - Paper Wrapping
// Compute sum_{n=1}^N g(n) via rectangular prism face tracking.

const N: u64 = 10_000_000_000_000_000; // 10^16
const M: u64 = 1_000_000_007;
// M is prime; M*M < 2^64 so u64 mulmod is exact.
const INV2: u64 = 500_000_004; // 2^{-1} mod M
const INV6: u64 = 166_666_668; // 6^{-1} mod M

#[inline]
fn mul(a: u64, b: u64) -> u64 {
    a * b % M
}

fn tr(n: u64) -> u64 {
    mul(mul(n % M, (n + 1) % M), INV2)
}

fn sum_sq(d: u64) -> u64 {
    if d == 0 {
        return 0;
    }
    let a = d % M;
    let b = (d + 1) % M;
    let c = (2 * d + 1) % M;
    mul(mul(mul(a, b), c), INV6)
}

// C(d2+2, 3) = d2*(d2+1)*(d2+2)/6
fn ncr3(d2: u64) -> u64 {
    let a = d2 % M;
    let b = (d2 + 1) % M;
    let c = (d2 + 2) % M;
    mul(mul(mul(a, b), c), INV6)
}

fn main() {
    let mut sides = [1u64, 1, 1];
    let mut index: u64 = 1;
    let n_mod = N % M;

    let tr_n = tr(N);
    let mut ans = 6 * ((tr_n + M - n_mod) % M) % M;

    loop {
        let side1 = sides[1];
        let side2 = sides[2];
        let d1_lim = (N - index - 1).isqrt();
        let d1 = d1_lim.min(side2 - 1);
        let d2_lim = (4 * (N - index)).isqrt().saturating_sub(1) / 2;
        let d2 = d2_lim.min(side1 - 1);

        let ni_mod = (N - index) % M;

        ans = (ans + M - 4 * ni_mod % M) % M;

        {
            let term = (mul(ni_mod, d1 % M) + M - sum_sq(d1)) % M;
            ans = (ans + M - 2 * term % M) % M;
        }

        {
            let c3 = ncr3(d2);
            let term = (mul(ni_mod, d2 % M) + M - 2 * c3 % M) % M;
            ans = (ans + M - 2 * term % M) % M;
        }

        index += side1 * side2;
        if index >= N {
            break;
        }

        let front = sides[0] + 1;
        sides[0] = sides[1];
        sides[1] = sides[2];
        sides[2] = front;
    }

    println!("{}", ans);
}
