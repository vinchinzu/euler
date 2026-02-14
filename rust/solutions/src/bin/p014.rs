// Project Euler 14: Longest Collatz sequence starting under 1 million

fn main() {
    const LIMIT: usize = 1_000_000;
    let mut cache = vec![0u32; LIMIT];
    cache[1] = 1;

    let mut max_len = 0u32;
    let mut best = 1usize;

    for i in 2..LIMIT {
        let mut n = i as u64;
        let mut len = 0u32;

        while n >= LIMIT as u64 || cache[n as usize] == 0 {
            len += 1;
            if n % 2 == 0 {
                n /= 2;
            } else {
                n = 3 * n + 1;
            }
        }
        len += cache[n as usize];
        cache[i] = len;

        if len > max_len {
            max_len = len;
            best = i;
        }
    }

    println!("{best}");
}
