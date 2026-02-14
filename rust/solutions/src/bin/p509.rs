// Project Euler 509 - Divisor Nim
// Sprague-Grundy values depend only on v2(n). Count winning triples via XOR != 0.

fn main() {
    let n: i64 = 123456787654321;
    let m: i64 = 1234567890;

    let mut counts = Vec::new();
    let mut k = 0u32;
    loop {
        let count = ((n >> k) - (n >> (k + 1))) % m;
        counts.push(count);
        if count == 0 { break; }
        k += 1;
    }

    let nc = counts.len();
    let mut ans: i64 = 0;
    for k1 in 0..nc {
        for k2 in 0..nc {
            for k3 in 0..nc {
                if (k1 ^ k2 ^ k3) != 0 {
                    ans = (ans + (counts[k1] % m) * (counts[k2] % m) % m * (counts[k3] % m)) % m;
                }
            }
        }
    }

    println!("{}", ans);
}
