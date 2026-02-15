// Project Euler 452: Long products

const N: i64 = 1_000_000_000;
const M: i64 = 1_234_567_891;

fn mod_pow(mut base: i64, mut exp: i64) -> i64 {
    let mut res = 1i64;
    base %= M;
    while exp > 0 {
        if exp & 1 == 1 { res = (res as i128 * base as i128 % M as i128) as i64; }
        base = (base as i128 * base as i128 % M as i128) as i64;
        exp >>= 1;
    }
    res
}

static mut ANS: i64 = 0;
static mut PRODS: [i64; 64] = [0; 64];
static mut INV_FACT: [i64; 64] = [0; 64];

fn helper(min_val: i64, n: i64, prev: i32, num_elements: usize, num_perm: i64) {
    unsafe {
        if prev != 1 {
            ANS = (ANS + (num_perm as i128 * PRODS[num_elements] as i128 % M as i128) as i64) % M;
        }
        if min_val <= N / n {
            let term = (num_perm as i128 * PRODS[num_elements + 1] as i128 % M as i128
                * ((N / n - min_val + 1) as i128) % M as i128) as i64;
            ANS = (ANS + term % M) % M;
        }

        let mut i = min_val;
        while i * i <= N / n {
            let mut count = 1;
            let mut new_n = n * i;
            while new_n <= N {
                let new_perm = (num_perm as i128 * INV_FACT[count] as i128 % M as i128) as i64;
                helper(i + 1, new_n, count as i32, num_elements + count, new_perm);
                count += 1;
                if new_n > N / i { break; }
                new_n *= i;
            }
            i += 1;
        }
    }
}

fn main() {
    unsafe {
        let mut l = 0usize;
        let mut tmp = N;
        while tmp > 0 { l += 1; tmp >>= 1; }
        l += 1;

        PRODS[0] = 1;
        for i in 1..=l {
            PRODS[i] = (PRODS[i - 1] as i128 * ((N + 1 - i as i64) as i128) % M as i128) as i64;
        }

        let mut fact = 1i64;
        for i in 1..=l {
            fact = (fact as i128 * i as i128 % M as i128) as i64;
        }
        INV_FACT[l] = mod_pow(fact, M - 2);
        for i in (1..=l).rev() {
            INV_FACT[i - 1] = (INV_FACT[i] as i128 * i as i128 % M as i128) as i64;
        }

        ANS = 0;
        helper(2, 1, 0, 0, 1);

        println!("{}", ANS % M);
    }
}
