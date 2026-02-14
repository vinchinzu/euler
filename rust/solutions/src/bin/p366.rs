// Project Euler 366: Stone game
// Sum of M(n) for n=1 to 10^18, mod 10^8.

const MOD: i64 = 100_000_000;
const MAX_FIBS: usize = 100;

static mut FIBS: [i64; MAX_FIBS] = [0; MAX_FIBS];
static mut NFIBS: usize = 0;

fn gen_fibs(n: i64) {
    unsafe {
        FIBS[0] = 1;
        FIBS[1] = 2;
        NFIBS = 2;
        while FIBS[NFIBS - 1] <= n {
            FIBS[NFIBS] = FIBS[NFIBS - 1] + FIBS[NFIBS - 2];
            NFIBS += 1;
        }
    }
}

fn tr(n: i64) -> i64 {
    if n < 0 { return 0; }
    let n = n % MOD;
    if n % 2 == 0 {
        (n / 2) % MOD * ((n + 1) % MOD) % MOD
    } else {
        (n % MOD) * (((n + 1) / 2) % MOD) % MOD
    }
}

fn sum_range(start: i64, end: i64) -> i64 {
    if start > end { return 0; }

    let mut fibonacci: i64 = 1;
    unsafe {
        for i in 0..NFIBS {
            if FIBS[i] <= start {
                fibonacci = FIBS[i];
            } else {
                break;
            }
        }
    }

    let mut max_identity = (fibonacci - 1) / 2;
    if max_identity > end - fibonacci {
        max_identity = end - fibonacci;
    }

    let mut result = (tr(max_identity) - tr(start - fibonacci - 1) % MOD + MOD) % MOD;
    result = (result + sum_range(max_identity + 1, end - fibonacci)) % MOD;
    result
}

fn main() {
    let n: i64 = 1_000_000_000_000_000_000;
    gen_fibs(n);

    let mut ans: i64 = 0;
    unsafe {
        for i in 0..NFIBS - 1 {
            if FIBS[i] > n { break; }
            let start = FIBS[i];
            let mut end = FIBS[i + 1] - 1;
            if end > n { end = n; }
            if end >= start {
                ans = (ans + sum_range(start, end)) % MOD;
            }
        }
    }

    println!("{}", ans);
}
