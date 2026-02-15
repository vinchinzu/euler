// Project Euler 866 - Number Caterpillar
// E[k] = (2k-1) * sum(E[i]*E[k-1-i]) with E[0]=1

const MOD: i64 = 987654319;

fn main() {
    let n = 100;
    let mut e = vec![0i64; n + 1];
    e[0] = 1;

    for k in 1..=n {
        let mut sum_val: i64 = 0;
        for i in 0..k {
            sum_val = (sum_val as i128 + e[i] as i128 * e[k - 1 - i] as i128 % MOD as i128) as i64 % MOD;
        }
        let factor = (2 * k as i64 - 1) % MOD;
        e[k] = (factor as i128 * sum_val as i128 % MOD as i128) as i64;
    }

    println!("{}", e[n]);
}
