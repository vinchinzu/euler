// Project Euler 560 - Coprime Nim
//
// Nimber of pile s: 0 if even, 1 if s=1, pi(p)+1 for smallest prime p|s otherwise.
// Use Walsh-Hadamard XOR convolution for K-fold XOR convolution of counts.

const N_VAL: usize = 10_000_000;
const K_VAL: i64 = 10_000_000;
const MOD: i64 = 1_000_000_007;

fn main() {
    // Sieve smallest prime factor
    let mut spf = vec![0u32; N_VAL + 1];
    for i in 2..=N_VAL {
        if spf[i] == 0 {
            let mut j = i;
            while j <= N_VAL {
                if spf[j] == 0 { spf[j] = i as u32; }
                j += i;
            }
        }
    }

    // Collect primes and their indices
    let mut primes: Vec<usize> = Vec::new();
    let mut prime_idx = vec![0usize; N_VAL + 1];
    for i in 2..=N_VAL {
        if spf[i] == i as u32 {
            prime_idx[i] = primes.len();
            primes.push(i);
        }
    }

    let nprimes = primes.len();
    let max_nimber = nprimes;
    let mut counts = vec![0i64; max_nimber + 2];

    for s in 1..N_VAL {
        let nim = if s % 2 == 0 {
            0
        } else if s == 1 {
            1
        } else {
            prime_idx[spf[s] as usize] + 1
        };
        counts[nim] += 1;
    }

    // Find next power of 2
    let mut sz = 1;
    while sz <= max_nimber { sz <<= 1; }

    let mut arr = vec![0i64; sz];
    for i in 0..=max_nimber {
        arr[i] = counts[i] % MOD;
    }

    // Forward FWHT
    let mut len = 1;
    while len < sz {
        let mut i = 0;
        while i < sz {
            for j in 0..len {
                let u = arr[i + j];
                let v = arr[i + j + len];
                arr[i + j] = (u + v) % MOD;
                arr[i + j + len] = ((u - v) % MOD + MOD) % MOD;
            }
            i += len << 1;
        }
        len <<= 1;
    }

    // Raise to K-th power
    for i in 0..sz {
        let mut base = arr[i];
        let mut exp = K_VAL;
        let mut result: i64 = 1;
        base %= MOD;
        while exp > 0 {
            if exp & 1 == 1 { result = (result as i128 * base as i128 % MOD as i128) as i64; }
            base = (base as i128 * base as i128 % MOD as i128) as i64;
            exp >>= 1;
        }
        arr[i] = result;
    }

    // Inverse FWHT
    len = 1;
    while len < sz {
        let mut i = 0;
        while i < sz {
            for j in 0..len {
                let u = arr[i + j];
                let v = arr[i + j + len];
                arr[i + j] = (u + v) % MOD;
                arr[i + j + len] = ((u - v) % MOD + MOD) % MOD;
            }
            i += len << 1;
        }
        len <<= 1;
    }

    // Divide by sz
    let inv_sz = {
        let mut base = sz as i64;
        let mut exp = MOD - 2;
        let mut result: i64 = 1;
        while exp > 0 {
            if exp & 1 == 1 { result = (result as i128 * base as i128 % MOD as i128) as i64; }
            base = (base as i128 * base as i128 % MOD as i128) as i64;
            exp >>= 1;
        }
        result
    };
    for i in 0..sz {
        arr[i] = (arr[i] as i128 * inv_sz as i128 % MOD as i128) as i64;
    }

    println!("{}", arr[0]);
}
