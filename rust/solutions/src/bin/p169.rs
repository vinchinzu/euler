// Project Euler 169: Number of ways to express n as sum of powers of 2
fn main() {
    let mut n: u128 = 1;
    for _ in 0..25 {
        n *= 10;
    }

    // Extract binary digits (LSB first)
    let mut bits = Vec::new();
    let mut temp = n;
    while temp > 0 {
        bits.push((temp & 1) as i32);
        temp >>= 1;
    }

    // DP: prev[carry] = number of ways
    let mut prev = [1i64, 0i64];

    for &bit in &bits {
        let mut next = [0i64; 2];
        for carry_in in 0..=1 {
            if prev[carry_in] == 0 { continue; }
            for coeff in 0..=2i32 {
                let diff = coeff + carry_in as i32 - bit;
                if diff == 0 {
                    next[0] += prev[carry_in];
                } else if diff == 2 {
                    next[1] += prev[carry_in];
                }
            }
        }
        prev = next;
    }

    println!("{}", prev[0]);
}
