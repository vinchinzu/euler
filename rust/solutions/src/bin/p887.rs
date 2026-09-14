// Project Euler 887
// N = 7^10, K = 7. Binary representation analysis.

const N: i64 = 282_475_249; // 7^10

#[inline(always)]
fn compute_sum(d: i64) -> i64 {
    let mut sum = 0i64;
    let mut prev_k = 1i64;
    let mut t = 1i64;
    let mut power_t = 2i64; // 2^t
    
    while prev_k < N {
        let kv = if t <= d {
            power_t.min(N)
        } else {
            let power_t_minus_d = 1i64 << (t - d);
            let adjustment = t + 1 - d - power_t_minus_d;
            (power_t + adjustment).min(N)
        };
        sum += (kv - prev_k) * t;
        prev_k = kv;
        t += 1;
        power_t <<= 1; // power_t *= 2
    }
    sum
}

fn main() {
    let mut ans = (N - 1) * N / 2;
    ans += compute_sum(1);
    ans += compute_sum(2);
    ans += compute_sum(3);
    ans += compute_sum(4);
    ans += compute_sum(5);
    ans += compute_sum(6);
    ans += compute_sum(7);
    println!("{}", ans);
}
