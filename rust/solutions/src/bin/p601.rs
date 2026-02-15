// Project Euler 601 - Divisibility streaks
// streak(n) = max k s.t. 1..k all divide n-1
// P(s,N) = floor((N-2)/lcm(1..s)) - floor((N-2)/lcm(1..s+1))
// Answer = sum_{i=1}^{31} P(i, 4^i)

fn main() {
    let mut ans: u128 = 0;
    for i in 1..=31u32 {
        let four_i: u128 = 1u128 << (2 * i as u128);
        let mut lcm_s: u128 = 1;
        for j in 1..=i as u128 {
            lcm_s = lcm_s / gcd128(lcm_s, j) * j;
        }
        let lcm_s1 = lcm_s / gcd128(lcm_s, (i + 1) as u128) * (i + 1) as u128;
        let n2 = four_i - 2;
        let count = n2 / lcm_s - n2 / lcm_s1;
        ans += count;
    }
    println!("{}", ans);
}

fn gcd128(mut a: u128, mut b: u128) -> u128 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}
