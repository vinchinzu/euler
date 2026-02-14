// Project Euler 306: Paper-strip Game
// Compute Grundy numbers, find period, extrapolate.

fn main() {
    const INIT_COMPUTE: usize = 5000;
    const LIMIT: usize = 1_000_000;

    let mut g = vec![0u8; INIT_COMPUTE + 1];
    let max_seen = 256;
    let mut seen = vec![false; max_seen];

    for n in 2..=INIT_COMPUTE {
        for s in seen.iter_mut() { *s = false; }
        for i in 0..n - 1 {
            let left = i;
            let right = n - i - 2;
            let xv = (g[left] ^ g[right]) as usize;
            if xv < max_seen {
                seen[xv] = true;
            }
        }
        let mut mex = 0;
        while mex < max_seen && seen[mex] { mex += 1; }
        g[n] = mex as u8;
    }

    // Find period
    let mut start = 0usize;
    let mut period = 0usize;
    'outer: for p in 1..INIT_COMPUTE / 3 {
        for s in 0..INIT_COMPUTE / 3 {
            let check_len = 100;
            if s + p + check_len > INIT_COMPUTE { continue; }
            let mut valid = true;
            for j in 0..check_len {
                if g[s + j] != g[s + p + j] { valid = false; break; }
            }
            if valid {
                start = s;
                period = p;
                break 'outer;
            }
        }
    }

    let mut count = 0usize;

    // Count before periodic part
    for n in 1..start {
        if g[n] != 0 { count += 1; }
    }

    // Count nonzeros in one period
    let mut nonzeros = 0usize;
    for i in 0..period {
        if g[start + i] != 0 { nonzeros += 1; }
    }

    // Full periods in [start, LIMIT]
    let remaining = LIMIT - start + 1;
    let full = remaining / period;
    let leftover = remaining % period;

    count += full * nonzeros;

    for i in 0..leftover {
        if g[start + i] != 0 { count += 1; }
    }

    println!("{}", count);
}
