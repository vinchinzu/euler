// Project Euler 491: Double pandigital numbers divisible by 11
fn n_cr(n: i32, k: i32) -> i64 {
    if k < 0 || k > n { return 0; }
    let k = k.min(n - k) as usize;
    let mut result: i64 = 1;
    for i in 0..k {
        result = result * (n as i64 - i as i64) / (i as i64 + 1);
    }
    result
}

fn g_ncr(counts: &[i32]) -> i64 {
    let mut total: i32 = counts.iter().sum();
    let mut result: i64 = 1;
    for &c in counts {
        result *= n_cr(total, c);
        total -= c;
    }
    result
}

fn main() {
    let b = 10;
    let mut ans: i64 = 0;

    for mask in 0..59049 {
        let mut tmp = mask;
        let mut counts = [0i32; 10];
        let mut num = 0;
        let mut sum_val = 0;
        for i in 0..b {
            counts[i] = tmp % 3;
            tmp /= 3;
            num += counts[i];
            sum_val += i as i32 * counts[i];
        }

        if num == b as i32 && (45 - sum_val) % (b as i32 + 1) == 0 {
            let res1 = g_ncr(&counts);
            let mut bs = [0i32; 10];
            for i in 0..b { bs[i] = 2 - counts[i]; }
            let mut res2 = g_ncr(&bs);
            if bs[0] > 0 {
                bs[0] -= 1;
                res2 -= g_ncr(&bs);
                bs[0] += 1;
            }
            ans += res1 * res2;
        }
    }

    println!("{ans}");
}
