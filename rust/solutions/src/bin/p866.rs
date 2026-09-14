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
            // SAFETY: i < k <= n and k-1-i < k <= n, so both indices < n+1 (array size)
            let a = unsafe { *e.get_unchecked(i) };
            let b = unsafe { *e.get_unchecked(k - 1 - i) };
            
            sum_val += a * b % MOD;
            
            if i & 3 == 3 {
                sum_val %= MOD;
            }
        }
        
        sum_val %= MOD;
        
        let factor = (2 * k as i64 - 1);
        let result = factor * sum_val % MOD;
        
        // SAFETY: k <= n, so k < n+1 (array size)
        unsafe {
            *e.get_unchecked_mut(k) = result;
        }
    }

    println!("{}", e[n]);
}
