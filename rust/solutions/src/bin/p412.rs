// Project Euler 412 - Young Tableaux
// Hook formula with Wilson's theorem for computing large factorials mod prime.

use euler_utils::mod_pow;

const PRIME: u64 = 76543217;

fn main() {
    let n: u64 = 10000;
    let k: u64 = 5000;
    let m = PRIME;

    let l = m - (n * n - k * k);

    // Precompute factorials and inverse factorials up to l
    let l_usize = l as usize;
    let mut fact = vec![0u64; l_usize + 1];
    let mut inv_fact = vec![0u64; l_usize + 1];

    fact[0] = 1;
    for i in 1..=l_usize {
        fact[i] = fact[i - 1] * (i as u64) % m;
    }

    inv_fact[l_usize] = mod_pow(fact[l_usize], m - 2, m);
    for i in (0..l_usize).rev() {
        inv_fact[i] = inv_fact[i + 1] * ((i + 1) as u64) % m;
    }

    // Compute numerator using Wilson's theorem
    // (N^2-K^2)! = (-1)^L * (L-1)!^{-1} (mod M)
    let parity = if l % 2 == 0 { 1u64 } else { m - 1 };
    let mut ans = mod_pow(parity * fact[l_usize - 1] % m, m - 2, m);

    // Top part: rows 0 to K-1 (done twice, s=0 and s=1)
    for _s in 0..2 {
        for i in 0..k {
            let i_usize = i as usize;
            let nk = (n - k + i) as usize;
            ans = ans * inv_fact[nk] % m;
            ans = ans * fact[i_usize] % m;
        }
    }

    // Bottom part: rows K to N-1
    for i in k..n {
        let ni = (n + i) as usize;
        let ki = (k + i) as usize;
        ans = ans * inv_fact[ni] % m;
        ans = ans * fact[ki] % m;
    }

    ans = ((ans % m) + m) % m;
    println!("{}", ans);
}
