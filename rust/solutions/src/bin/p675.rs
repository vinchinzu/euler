// Project Euler 675 - 2^omega(n!)
// Iteratively compute S(i!) by tracking prime exponents.

const N: usize = 10_000_000;
const MOD: i64 = 1_000_000_087;

fn main() {
    let mut ff = vec![0u32; N + 1];
    for i in 0..=N { ff[i] = i as u32; }
    for i in 2..=N {
        if (i as u64) * (i as u64) > N as u64 { break; }
        if ff[i] == i as u32 {
            for j in (i*i..=N).step_by(i) { if ff[j] == j as u32 { ff[j] = i as u32; } }
        }
    }
    let mut invs = vec![0i64; 2 * N + 1];
    invs[1] = 1;
    for i in 2..=2*N { invs[i] = (MOD - (MOD / i as i64) * invs[(MOD % i as i64) as usize] % MOD) % MOD; }
    let mut exponents = vec![0u32; N + 1];
    let mut s = 1i64;
    let mut ans = 0i64;
    for i in 2..=N {
        let mut ii = i;
        while ii > 1 {
            let p = ff[ii] as usize;
            let mut e = 0u32;
            while ii % p == 0 { ii /= p; e += 1; }
            s = (s as i128 * invs[(1 + 2 * exponents[p] as usize)] as i128 % MOD as i128) as i64;
            exponents[p] += e;
            s = (s as i128 * (1 + 2 * exponents[p] as i64) as i128 % MOD as i128) as i64;
        }
        ans = (ans + s) % MOD;
    }
    println!("{}", ans);
}
