// Project Euler 038: Pandigital Multiples
// Largest 1-9 pandigital formed as concatenated product of k with (1,2,...,n), n>1.

fn main() {
    let mut max_pan: u64 = 0;

    for k in 2u64..=9999 {
        let mut concat = String::new();
        for n in 1..=9 {
            concat.push_str(&(k * n).to_string());
            if concat.len() > 9 {
                break;
            }
            if concat.len() == 9 && is_pandigital(&concat) {
                let val: u64 = concat.parse().unwrap();
                if val > max_pan {
                    max_pan = val;
                }
                break;
            }
        }
    }

    println!("{max_pan}");
}

fn is_pandigital(s: &str) -> bool {
    if s.len() != 9 {
        return false;
    }
    let mut seen = [false; 10];
    for b in s.bytes() {
        let d = (b - b'0') as usize;
        if d == 0 || seen[d] {
            return false;
        }
        seen[d] = true;
    }
    true
}
