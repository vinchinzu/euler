// Project Euler 303: Multiples with small digits
// BFS over remainders mod n.

use std::collections::VecDeque;

fn find_min_multiple(n: u32) -> u128 {
    if n == 1 { return 1; }

    let mut visited = vec![false; n as usize];
    let mut queue: VecDeque<(u32, u128)> = VecDeque::new();

    for d in 1..=2u32 {
        let r = d % n;
        if r == 0 { return d as u128; }
        if !visited[r as usize] {
            visited[r as usize] = true;
            queue.push_back((r, d as u128));
        }
    }

    while let Some((rem, val)) = queue.pop_front() {
        for d in 0..=2u32 {
            let nv = val * 10 + d as u128;
            let nr = (rem * 10 + d) % n;
            if nr == 0 { return nv; }
            if !visited[nr as usize] {
                visited[nr as usize] = true;
                queue.push_back((nr, nv));
            }
        }
    }

    0
}

fn main() {
    let mut total: i64 = 0;
    for n in 1..=10000u32 {
        let f = find_min_multiple(n);
        total += (f / n as u128) as i64;
    }
    println!("{}", total);
}
