// Project Euler 159 - Digital root sums of factorisations
// For n from 2 to 999999, find max digital root sum over all factorisations.

fn main() {
    const MAX: usize = 1_000_000;

    let mut dr = vec![0u8; MAX];
    for i in 1..MAX {
        dr[i] = if i % 9 == 0 { 9 } else { (i % 9) as u8 };
    }

    let mut dp = vec![0u8; MAX];
    for n in 2..MAX {
        dp[n] = dr[n];
    }

    for d in 2..MAX {
        let mut m = d * d;
        while m < MAX {
            let q = m / d;
            let v1 = dr[d] as u16 + dp[q] as u16;
            let v2 = dr[q] as u16 + dp[d] as u16;
            let v = v1.max(v2) as u8;
            if v > dp[m] {
                dp[m] = v;
            }
            m += d;
        }
    }

    let sum: u64 = dp[2..].iter().map(|&x| x as u64).sum();
    println!("{}", sum);
}
