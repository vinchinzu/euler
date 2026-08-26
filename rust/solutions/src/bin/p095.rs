// Project Euler 95: Amicable chains
// Find the smallest member of the longest amicable chain under 1,000,000.

fn main() {
    const LIMIT: usize = 1_000_000;

    let mut sum_div = vec![0u32; LIMIT + 1];
    for i in 1..=LIMIT {
        let mut j = 2 * i;
        while j <= LIMIT {
            sum_div[j] += i as u32;
            j += i;
        }
    }

    // vis[x] == stamp  => x is on the current path
    // vis[x] != 0      => x was already explored
    let mut vis = vec![0u32; LIMIT + 1];
    let mut pos_in_path = vec![0u32; LIMIT + 1];
    let mut path = Vec::with_capacity(100);
    let mut stamp = 0u32;

    let mut longest = 0usize;
    let mut smallest = 0u32;

    for start in 2..=LIMIT {
        if vis[start] != 0 {
            continue;
        }
        stamp += 1;
        path.clear();
        let mut current = start as u32;

        loop {
            let c = current as usize;
            if c == 0 || c > LIMIT {
                break;
            }
            if vis[c] == stamp {
                let idx = pos_in_path[c] as usize;
                let cycle_len = path.len() - idx;
                if cycle_len > longest {
                    longest = cycle_len;
                    smallest = *path[idx..].iter().min().unwrap();
                }
                break;
            }
            if vis[c] != 0 {
                break;
            }
            vis[c] = stamp;
            pos_in_path[c] = path.len() as u32;
            path.push(current);
            current = sum_div[c];
        }
    }

    println!("{smallest}");
}
