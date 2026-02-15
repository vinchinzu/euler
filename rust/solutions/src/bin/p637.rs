// Project Euler 637 - Flexible Digit Sum
// f(n,B) = min steps to reduce to single digit; sum n where f(n,10)==f(n,3)

const N_VAL: usize = 10_000_000;

fn sum_digits_b(mut n: usize, base: usize) -> usize {
    let mut s = 0;
    while n > 0 { s += n % base; n /= base; }
    s
}

fn good(sum_val: usize, remaining: usize, base: usize, sd: &[usize]) -> bool {
    if remaining == 0 { return sd[sum_val] < base; }
    let mut pow_base = base;
    while pow_base <= base * remaining {
        if good(sum_val + remaining % pow_base, remaining / pow_base, base, sd) { return true; }
        if pow_base > remaining { break; }
        pow_base *= base;
    }
    false
}

fn compute_f(n: usize, b: usize) -> Vec<u8> {
    let sd: Vec<usize> = (0..=n).map(|i| sum_digits_b(i, b)).collect();
    let mut f = vec![0u8; n + 1];
    for i in 0..=n {
        if i < b { f[i] = 0; }
        else if sd[i] < b { f[i] = 1; }
        else if good(0, i, b, &sd) { f[i] = 2; }
        else { f[i] = 3; }
    }
    f
}

fn main() {
    let f1 = compute_f(N_VAL, 10);
    let f2 = compute_f(N_VAL, 3);
    let ans: i64 = (1..=N_VAL).filter(|&i| f1[i] == f2[i]).map(|i| i as i64).sum();
    println!("{}", ans);
}
