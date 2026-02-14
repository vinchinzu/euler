// Project Euler 310: Nim Square

const MAX_HEAP: usize = 100_000;

fn main() {
    // Precompute squares
    let sq_limit = (MAX_HEAP as f64).sqrt() as usize + 1;
    let squares: Vec<usize> = (1..=sq_limit).map(|i| i * i).collect();

    // Compute Grundy values
    let mut grundy = vec![0u16; MAX_HEAP + 1];
    let max_cand = squares.len() + 2;
    let mut seen = vec![false; max_cand];

    for n in 1..=MAX_HEAP {
        for &sq in &squares {
            if sq > n { break; }
            let g = grundy[n - sq] as usize;
            if g < max_cand { seen[g] = true; }
        }
        let mut mex = 0;
        while mex < max_cand && seen[mex] { mex += 1; }
        grundy[n] = mex as u16;

        // Reset
        for &sq in &squares {
            if sq > n { break; }
            let g = grundy[n - sq] as usize;
            if g < max_cand { seen[g] = false; }
        }
    }

    // Find max Grundy value
    let max_g = *grundy.iter().max().unwrap() as usize;

    // Build frequency table
    let mut freq = vec![0u32; max_g + 1];
    for i in 0..=MAX_HEAP {
        freq[grundy[i] as usize] += 1;
    }

    // Build sorted position arrays by Grundy value
    let mut positions: Vec<Vec<u32>> = vec![Vec::new(); max_g + 1];
    for i in 0..=MAX_HEAP {
        positions[grundy[i] as usize].push(i as u32);
    }

    // Count triples (a,b,c) with a<=b<=c and g[a]^g[b]^g[c]==0
    let mut total: i64 = 0;

    for ga in 0..=max_g {
        if freq[ga] == 0 { continue; }
        let pos_a = &positions[ga];
        let na = pos_a.len();

        for gb in 0..=max_g {
            if freq[gb] == 0 { continue; }
            let gc = ga ^ gb;
            if gc > max_g || freq[gc] == 0 { continue; }

            let pos_b = &positions[gb];
            let nb = pos_b.len();
            let pos_c = &positions[gc];
            let nc = pos_c.len();

            let mut ia = 0usize;
            let mut ic_start = 0usize;
            for ib in 0..nb {
                let b = pos_b[ib];
                while ia < na && pos_a[ia] <= b { ia += 1; }
                let num_a = ia;

                while ic_start < nc && pos_c[ic_start] < b { ic_start += 1; }
                let num_c = nc - ic_start;

                if num_a > 0 && num_c > 0 {
                    total += num_a as i64 * num_c as i64;
                }
            }
        }
    }

    println!("{}", total);
}
