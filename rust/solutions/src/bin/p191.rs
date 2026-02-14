// Project Euler 191: Prize Strings

fn main() {
    let n = 30;
    // prev[consec_a][l_count]
    let mut prev = [[0i64; 2]; 3];
    prev[0][0] = 1;

    for _ in 0..n {
        let mut curr = [[0i64; 2]; 3];
        for ca in 0..3 {
            for lc in 0..2 {
                let ways = prev[ca][lc];
                if ways == 0 { continue; }
                // On time (O)
                curr[0][lc] += ways;
                // Absent (A)
                if ca < 2 { curr[ca + 1][lc] += ways; }
                // Late (L)
                if lc == 0 { curr[0][1] += ways; }
            }
        }
        prev = curr;
    }

    let total: i64 = prev.iter().flat_map(|r| r.iter()).sum();
    println!("{}", total);
}
