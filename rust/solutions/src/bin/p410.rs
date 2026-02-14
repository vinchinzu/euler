// Project Euler 410: Circle and Tangent Line

fn main() {
    let a: usize = 100_000_000; // 10^8
    let b: i64 = 1_000_000_000; // 10^9

    // Sieve omega: number of distinct prime factors
    let mut omega = vec![0u8; a + 1];
    for i in 2..=a {
        if omega[i] == 0 {
            // i is prime
            let mut j = i;
            while j <= a {
                omega[j] += 1;
                j += i;
            }
        }
    }

    let mut ans: i64 = a as i64 * b; // b=c horizontal line solutions

    for j in 2..=a {
        let res: i64;
        if j % 2 == 0 {
            let aj = (a / j) as i64;
            let bj = b / j as i64;
            let aj_even = aj / 2;
            let bj_even = bj / 2;
            let aj_odd = (aj + 1) / 2;
            let bj_odd = (bj + 1) / 2;
            res = aj_even * bj_even + aj_odd * bj_odd;
        } else {
            res = (a / j) as i64 * (b / j as i64);
        }
        ans += (1i64 << omega[j]) * res;
    }

    ans *= 4;
    println!("{ans}");
}
