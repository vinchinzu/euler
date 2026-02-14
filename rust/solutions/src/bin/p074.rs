// Project Euler 74: Digit factorial chains
// Count starting numbers below 1,000,000 that produce a chain of exactly 60 non-repeating terms.

const FACTORIALS: [u32; 10] = [1, 1, 2, 6, 24, 120, 720, 5040, 40320, 362880];

fn digit_factorial_sum(mut n: u32) -> u32 {
    if n == 0 { return 1; }
    let mut s = 0u32;
    while n > 0 {
        s += FACTORIALS[(n % 10) as usize];
        n /= 10;
    }
    s
}

fn main() {
    let limit = 1_000_000usize;
    let cache_size = 3_000_000usize;
    let mut cache = vec![0u16; cache_size];

    for i in 1..limit {
        if cache[i] != 0 { continue; }

        let mut path = Vec::with_capacity(64);
        let mut current = i as u32;

        loop {
            if (current as usize) < cache_size && cache[current as usize] != 0 {
                // Fill in path backwards
                let known_len = cache[current as usize] as u32;
                for (j, &node) in path.iter().enumerate().rev() {
                    let len = (path.len() - j) as u32 + known_len;
                    if (node as usize) < cache_size {
                        cache[node as usize] = len as u16;
                    }
                }
                break;
            }

            // Check if current is already in path (cycle)
            if let Some(pos) = path.iter().position(|&x| x == current) {
                let cycle_len = (path.len() - pos) as u16;
                for &node in &path[pos..] {
                    if (node as usize) < cache_size {
                        cache[node as usize] = cycle_len;
                    }
                }
                for j in 0..pos {
                    let len = (pos - j) as u16 + cycle_len;
                    if (path[j] as usize) < cache_size {
                        cache[path[j] as usize] = len;
                    }
                }
                break;
            }

            path.push(current);
            current = digit_factorial_sum(current);
        }
    }

    let count = (1..limit).filter(|&i| cache[i] == 60).count();
    println!("{count}");
}
