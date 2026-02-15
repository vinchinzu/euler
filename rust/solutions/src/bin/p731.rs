// Project Euler 731 - A Stoneham Number
//
// Compute digits of Stoneham number sum 1/(3^i * 10^(3^i)).

fn pow_mod_ull(mut base: u64, mut exp: u64, m: u64) -> u64 {
    if m == 1 {
        return 0;
    }
    let mut result: u64 = 1;
    base %= m;
    while exp > 0 {
        if exp & 1 == 1 {
            result = (result as u128 * base as u128 % m as u128) as u64;
        }
        base = (base as u128 * base as u128 % m as u128) as u64;
        exp >>= 1;
    }
    result
}

fn mod_inv_ll(a: i64, m: i64) -> i64 {
    fn ext_gcd(a: i64, b: i64) -> (i64, i64, i64) {
        if a == 0 {
            return (b, 0, 1);
        }
        let (g, x1, y1) = ext_gcd(b % a, a);
        (g, y1 - (b / a) * x1, x1)
    }
    let (_, x, _) = ext_gcd(((a % m) + m) % m, m);
    ((x % m) + m) % m
}

fn main() {
    let n: u64 = 10_000_000_000_000_000; // 10^16
    let k: u64 = 10;
    let m: u64 = 1_000_000_000_000; // 10^12

    let mut ans: u64 = 0;
    let mut three_power: u64 = 3;

    while three_power < n {
        let exp = n + k + 1 - three_power;
        let term1 = pow_mod_ull(10, exp, m);
        let term2 = pow_mod_ull(10, exp, three_power);

        let inv_three_i = mod_inv_ll((three_power % m) as i64, m as i64);

        let diff = term1 as i64 - term2 as i64;
        let contribution = (diff as i128 * inv_three_i as i128 % m as i128) as i64;
        let contribution = ((contribution % m as i64) + m as i64) % m as i64;

        ans = (ans + contribution as u64) % m;

        if three_power > n / 3 {
            break;
        }
        three_power *= 3;
    }

    let result = ans / 100;
    println!("{}", result);
}
