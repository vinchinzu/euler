// Project Euler 315 - Digital root clocks
// Segmented sieve for primes in [10^7, 2*10^7], compute Sam-Max transitions.

const DIGIT_MASK: [u8; 10] = [
    0b1110111, // 0: top|top_left|top_right|bot_left|bot_right|bottom
    0b0100100, // 1: top_right|bot_right
    0b1011101, // 2: top|top_right|middle|bot_left|bottom
    0b1101101, // 3: top|top_right|middle|bot_right|bottom
    0b0101110, // 4: top_left|top_right|middle|bot_right
    0b1101011, // 5: top|top_left|middle|bot_right|bottom
    0b1111011, // 6: top|top_left|middle|bot_left|bot_right|bottom
    0b0100111, // 7: top|top_left|top_right|bot_right
    0b1111111, // 8: all
    0b1101111, // 9: top|top_left|top_right|middle|bot_right|bottom
];

fn popcount(x: u8) -> i32 {
    x.count_ones() as i32
}

fn seg_count(d: usize) -> i32 { popcount(DIGIT_MASK[d]) }
fn diff_count(a: usize, b: usize) -> i32 { popcount(DIGIT_MASK[a] ^ DIGIT_MASK[b]) }

fn sam_minus_max(mut value: i32) -> i64 {
    let mut digits = Vec::new();
    let mut segs = 0i32;
    let mut dsum = 0i32;
    let mut v = value;
    while v > 0 {
        let d = (v % 10) as usize;
        digits.push(d);
        segs += seg_count(d);
        dsum += d as i32;
        v /= 10;
    }

    let mut sam_total = 2 * segs;
    let mut max_total = segs;
    let mut prev_digits = digits.clone();
    let mut next_val = dsum;

    while value >= 10 {
        value = next_val;
        digits.clear();
        segs = 0;
        dsum = 0;
        v = value;
        if v == 0 {
            digits.push(0);
            segs = seg_count(0);
        } else {
            while v > 0 {
                let d = (v % 10) as usize;
                digits.push(d);
                segs += seg_count(d);
                dsum += d as i32;
                v /= 10;
            }
        }
        sam_total += 2 * segs;

        let shared = prev_digits.len().min(digits.len());
        let mut trans = 0i32;
        for i in 0..shared {
            trans += diff_count(prev_digits[i], digits[i]);
        }
        for i in shared..prev_digits.len() {
            trans += seg_count(prev_digits[i]);
        }
        for i in shared..digits.len() {
            trans += seg_count(digits[i]);
        }
        max_total += trans;

        prev_digits = digits.clone();
        next_val = dsum;
    }

    max_total += segs; // final turn-off
    (sam_total - max_total) as i64
}

fn main() {
    let lower = 10_000_000;
    let upper = 20_000_000;
    let root = (upper as f64).sqrt() as usize + 1;

    let mut small_sieve = vec![true; root + 1];
    small_sieve[0] = false;
    small_sieve[1] = false;
    for i in 2..=root {
        if small_sieve[i] {
            let mut j = i * i;
            while j <= root { small_sieve[j] = false; j += i; }
        }
    }
    let small_primes: Vec<usize> = (2..=root).filter(|&i| small_sieve[i]).collect();

    let seg_size = 1_000_000;
    let mut total: i64 = 0;

    let mut low = lower;
    while low <= upper {
        let high = (low + seg_size - 1).min(upper);
        let size = high - low + 1;
        let mut seg = vec![true; size];

        for &p in &small_primes {
            let mut start = ((low + p - 1) / p) * p;
            if start < p * p { start = p * p; }
            if start > high { continue; }
            let mut j = start - low;
            while j < size { seg[j] = false; j += p; }
        }

        for i in 0..size {
            if seg[i] {
                total += sam_minus_max((low + i) as i32);
            }
        }

        low += seg_size;
    }

    println!("{}", total);
}
