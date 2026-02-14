// Project Euler 168: Number Rotations
// Find last 5 digits of sum of all integers with 2-100 digits that divide their right rotation.

fn main() {
    let n = 100;
    let modulus: u64 = 100_000;
    let mut total: u64 = 0;

    for mult in 1..=9u8 {
        for last_digit in 1..=9u8 {
            // b as big decimal digits (least significant concept handled via string)
            let mut b: Vec<u8> = vec![last_digit];
            // b[0] is most significant digit

            for i in 1..=n {
                // Compute multb = mult * b as digits
                let mut multb: Vec<u8> = Vec::with_capacity(b.len() + 2);
                let mut carry: u16 = 0;
                for j in (0..b.len()).rev() {
                    let d = b[j] as u16 * mult as u16 + carry;
                    carry = d / 10;
                    multb.push((d % 10) as u8);
                }
                while carry > 0 {
                    multb.push((carry % 10) as u8);
                    carry /= 10;
                }
                multb.reverse();

                // Check rotation: rotated = multb[1:] + multb[0]
                if multb.len() == b.len() && b[0] != 0 && i > 1 {
                    let mut rotated = multb[1..].to_vec();
                    rotated.push(multb[0]);

                    if rotated == b {
                        // Add b mod MOD to total
                        let start = if b.len() > 5 { b.len() - 5 } else { 0 };
                        let mut val: u64 = 0;
                        for j in start..b.len() {
                            val = (val * 10 + b[j] as u64) % modulus;
                        }
                        total = (total + val) % modulus;
                    }
                }

                // Next b: take last i chars of multb, append last_digit
                let start_pos = if multb.len() > i { multb.len() - i } else { 0 };
                let mut new_b: Vec<u8> = multb[start_pos..].to_vec();
                new_b.push(last_digit);
                b = new_b;
            }
        }
    }

    println!("{}", total % modulus);
}
