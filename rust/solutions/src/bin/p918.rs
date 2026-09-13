// Project Euler 918 - Recursive Sequence
// a_1=1, a_(2n)=2*a_n, a_(2n+1)=a_n - 3*a_(n+1)
// S(N) = sum a_n for n=1..N
// For N>=10, S(N) = 4 - a(N/2)
// Optimization: factor out powers of 2 since a(2^k*m) = 2^k*a(m)

use fxhash::FxHashMap;

fn main() {
    let n: i64 = 1_000_000_000_000; // 10^12

    let mut x = n / 2;
    let mut power_of_2 = 1i64;
    while x % 2 == 0 {
        x /= 2;
        power_of_2 *= 2;
    }
    
    let a_val = power_of_2 * get_a(x);
    let result = 4 - a_val;
    println!("{}", result);
}

fn get_a(k: i64) -> i64 {
    let mut memo: FxHashMap<i64, i64> = FxHashMap::with_capacity_and_hasher(1 << 20, Default::default());
    memo.insert(1, 1);

    let mut stack: Vec<i64> = vec![k];

    while let Some(&cur) = stack.last() {
        if memo.contains_key(&cur) {
            stack.pop();
            continue;
        }
        if cur == 1 {
            memo.insert(1, 1);
            stack.pop();
            continue;
        }
        if cur % 2 == 0 {
            let half = cur / 2;
            if let Some(&hv) = memo.get(&half) {
                memo.insert(cur, 2 * hv);
                stack.pop();
            } else {
                stack.push(half);
            }
        } else {
            let m = cur / 2;
            let mv = memo.get(&m).copied();
            let m1v = memo.get(&(m + 1)).copied();
            match (mv, m1v) {
                (Some(a), Some(b)) => {
                    memo.insert(cur, a - 3 * b);
                    stack.pop();
                }
                _ => {
                    if mv.is_none() {
                        stack.push(m);
                    }
                    if m1v.is_none() {
                        stack.push(m + 1);
                    }
                }
            }
        }
    }

    memo[&k]
}
