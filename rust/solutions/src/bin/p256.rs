// Project Euler 256: Tatami-Free Rooms
// Find smallest area s with exactly 200 tatami-free rooms.

fn main() {
    const TARGET: i16 = 200;
    const LIMIT: usize = 100_000_000;
    let mut counts = vec![0i16; LIMIT + 1];

    let sq = (LIMIT as f64).sqrt() as usize;

    for a in 3..=sq {
        let max_b = LIMIT / a;
        let max_k = if a >= 5 { (a - 5) / 2 } else { continue };

        for k in 0..=max_k {
            let lo_raw = k * (a + 1) + 2;
            let hi_raw = (k + 1) * (a - 1) - 2;
            let lo = lo_raw.max(a);
            let hi = hi_raw.min(max_b);
            if lo > hi { continue; }

            for b in lo..=hi {
                let s = a * b;
                if s <= LIMIT {
                    counts[s] += 1;
                }
            }
        }
    }

    for s in 1..=LIMIT {
        if counts[s] == TARGET {
            println!("{}", s);
            return;
        }
    }
    println!("0");
}
