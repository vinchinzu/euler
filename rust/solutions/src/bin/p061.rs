// Project Euler 61: Cyclical figurate numbers
// Find the unique cycle of six 4-digit numbers, one from each polygonal type (3-8).

fn polygonal(kind: usize, n: usize) -> usize {
    match kind {
        3 => n * (n + 1) / 2,
        4 => n * n,
        5 => n * (3 * n - 1) / 2,
        6 => n * (2 * n - 1),
        7 => n * (5 * n - 3) / 2,
        8 => n * (3 * n - 2),
        _ => 0,
    }
}

fn main() {
    // Generate all 4-digit polygonal numbers, indexed by prefix (first two digits)
    let mut by_prefix: Vec<Vec<Vec<usize>>> = vec![vec![Vec::new(); 100]; 6];
    let mut all_nums: Vec<Vec<usize>> = vec![Vec::new(); 6];

    for t in 0..6 {
        let kind = t + 3;
        let mut n = 1;
        loop {
            let val = polygonal(kind, n);
            if val >= 10000 { break; }
            if val >= 1000 {
                all_nums[t].push(val);
                by_prefix[t][val / 100].push(val);
            }
            n += 1;
        }
    }

    // DFS to find a 6-chain where each number's last 2 digits = next number's first 2 digits
    let mut chain = [0usize; 6];
    let mut type_used = [false; 6];
    let mut answer = 0u64;

    'outer: for t in 0..6 {
        for &start in &all_nums[t] {
            chain[0] = start;
            type_used[t] = true;

            fn search(
                depth: usize,
                chain: &mut [usize; 6],
                type_used: &mut [bool; 6],
                by_prefix: &[Vec<Vec<usize>>],
                answer: &mut u64,
            ) -> bool {
                if depth == 6 {
                    return chain[5] % 100 == chain[0] / 100;
                }
                let needed = chain[depth - 1] % 100;
                if needed < 10 { return false; }
                for t in 0..6 {
                    if type_used[t] { continue; }
                    type_used[t] = true;
                    for &val in &by_prefix[t][needed] {
                        chain[depth] = val;
                        if search(depth + 1, chain, type_used, by_prefix, answer) {
                            *answer = chain.iter().map(|&x| x as u64).sum();
                            return true;
                        }
                    }
                    type_used[t] = false;
                }
                false
            }

            if search(1, &mut chain, &mut type_used, &by_prefix, &mut answer) {
                break 'outer;
            }
            type_used[t] = false;
        }
    }

    println!("{answer}");
}
