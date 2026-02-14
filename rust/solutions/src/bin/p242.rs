// Project Euler 242: Odd Triplets
fn main() {
    let big_n: u64 = 1_000_000_000_000;
    let mut n = (big_n + 3) / 4;
    let mut e = 0u32;
    let mut ans: u64 = 0;

    while n > 0 {
        let r = n % 2;
        ans = (r + 1) * ans + r * 3u64.pow(e);
        n /= 2;
        e += 1;
    }

    println!("{}", ans);
}
