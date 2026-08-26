// Project Euler 122: Efficient exponentiation.
// Star (Brauer) addition chains, DFS, prune non-minimal prefixes.

const LIMIT: usize = 200;

fn dfs(chain: &mut [u16; 16], len: usize, m: &mut [u16; LIMIT + 1]) {
    let top = chain[len - 1] as usize;
    let depth = (len - 1) as u16;
    if depth > m[top] {
        return;
    }
    m[top] = depth;
    if depth == 11 {
        return;
    }
    for i in (0..len).rev() {
        let v = top + chain[i] as usize;
        if v > LIMIT {
            continue;
        }
        chain[len] = v as u16;
        dfs(chain, len + 1, m);
    }
}

fn main() {
    let mut m = [u16::MAX; LIMIT + 1];
    m[1] = 0;
    let mut chain = [0u16; 16];
    chain[0] = 1;
    dfs(&mut chain, 1, &mut m);
    let total: u32 = m[1..=LIMIT].iter().map(|&x| x as u32).sum();
    println!("{}", total);
}
