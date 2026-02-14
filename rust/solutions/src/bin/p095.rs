// Project Euler 95: Amicable chains
// Find the smallest member of the longest amicable chain under 1,000,000.

fn main() {
    const LIMIT: usize = 1_000_000;

    // Compute sum of proper divisors via sieve
    let mut sum_div = vec![0u32; LIMIT + 1];
    for i in 1..=LIMIT {
        let mut j = 2 * i;
        while j <= LIMIT {
            sum_div[j] += i as u32;
            j += i;
        }
    }

    let mut longest_chain = 0;
    let mut smallest_member = 0;

    for start in 2..=LIMIT {
        let mut chain = Vec::with_capacity(100);
        let mut current = start as u32;

        while chain.len() < 100 {
            if current as usize > LIMIT || current == 0 {
                break;
            }
            if chain.contains(&current) {
                break;
            }
            chain.push(current);
            current = sum_div[current as usize];
        }

        // Check if we formed a cycle back to start
        if current == start as u32 && chain.len() > 1 {
            if chain.len() > longest_chain {
                longest_chain = chain.len();
                smallest_member = *chain.iter().min().unwrap() as usize;
            }
        }
    }

    println!("{smallest_member}");
}
