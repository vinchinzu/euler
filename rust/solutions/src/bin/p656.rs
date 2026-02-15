// Project Euler 656 - Palindromic sequences
// H_100(sqrt(beta)) for non-square beta <= 1000 using continued fractions.

const M_VAL: i64 = 1_000_000_000_000_000;

fn is_square(n: i32) -> bool {
    let r = (n as f64).sqrt() as i32;
    for t in (r-1).max(0)..=r+1 { if t * t == n { return true; } }
    false
}

fn h(beta: i32, k: usize) -> i64 {
    let a0 = (beta as f64).sqrt() as i32;
    if a0 * a0 == beta { return 0; }
    let mut ns = vec![0i64];
    let mut qs = vec![1i64, 0i64];
    let mut p = a0;
    let mut q = beta - a0 * a0;
    // Process a0
    {
        let a = a0 as i64;
        if qs.len() % 2 == 1 {
            for _ in 0..a {
                if ns.len() > k { break; }
                let nv = (ns.last().unwrap() + qs[qs.len() - 1]) % M_VAL;
                ns.push(nv);
            }
        }
        let new_q = (qs[qs.len() - 1] * a + qs[qs.len() - 2]) % M_VAL;
        qs.push(new_q);
    }
    while ns.len() <= k && q != 0 {
        let a = ((a0 + p) / q) as i64;
        if qs.len() % 2 == 1 {
            for _ in 0..a {
                if ns.len() > k { break; }
                let nv = (ns.last().unwrap() + qs[qs.len() - 1]) % M_VAL;
                ns.push(nv);
            }
        }
        let new_q = (qs[qs.len() - 1] * a + qs[qs.len() - 2]) % M_VAL;
        qs.push(new_q);
        p = a as i32 * q - p;
        q = (beta - p * p) / q;
        if q == 0 { break; }
    }
    let limit = (k + 1).min(ns.len());
    let mut result = 0i64;
    for i in 1..limit { result = (result + ns[i]) % M_VAL; }
    result
}

fn main() {
    let mut ans = 0i64;
    for beta in 1..=1000 {
        if is_square(beta) { continue; }
        ans = (ans + h(beta, 100)) % M_VAL;
    }
    println!("{}", ans);
}
