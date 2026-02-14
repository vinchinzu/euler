// Project Euler 53: Combinatoric selections
// Count C(n,r) > 1,000,000 for 1 <= n <= 100.

fn main() {
    let limit: u64 = 1_000_000;
    let mut count = 0u32;

    for n in 1u64..=100 {
        for r in 0..=n {
            let k = if r > n - r { n - r } else { r };
            let mut val = 1u64;
            let mut exceeded = false;
            for i in 1..=k {
                val = val * (n - k + i) / i;
                if val > limit {
                    exceeded = true;
                    break;
                }
            }
            if exceeded {
                count += 1;
            }
        }
    }

    println!("{count}");
}
