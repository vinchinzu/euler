// Project Euler Problem 948
// Left-Right game on words of L's and R's.
// F(n) = number of words of length n where first mover wins.
// Find F(60).
//
// State: (k, gp, gs) where k = number of proper suffixes of Type L,
// gp = Good Prefix, gs = Good Suffix (booleans).
// Index: k * 4 + gp * 2 + gs

fn main() {
    let n = 60usize;
    let max_k = n + 2;
    let state_size = max_k * 4;

    let mut counts = vec![0i64; state_size];

    let idx = |k: usize, gp: usize, gs: usize| -> usize { k * 4 + gp * 2 + gs };

    // Base cases for length 1: "L" -> (0, 0, 1), "R" -> (0, 0, 0)
    counts[idx(0, 0, 1)] = 1; // "L"
    counts[idx(0, 0, 0)] = 1; // "R"

    for length in 1..n {
        let mut new_counts = vec![0i64; state_size];
        for k in 0..=length {
            for gp in 0..=1 {
                for gs in 0..=1 {
                    let count = counts[idx(k, gp, gs)];
                    if count == 0 {
                        continue;
                    }

                    // Add "L"
                    let nk_l = k + 1;
                    let ngp_l = if gp == 1 || gs == 0 { 1 } else { 0 };
                    let ngs_l = 1;
                    new_counts[idx(nk_l, ngp_l, ngs_l)] += count;

                    // Add "R"
                    let nk_r = if k > 0 { k - 1 } else { 0 };
                    let ngp_r = if gp == 1 || gs == 0 { 1 } else { 0 };
                    let ngs_r = if k > 0 { 1 } else { 0 };
                    new_counts[idx(nk_r, ngp_r, ngs_r)] += count;
                }
            }
        }
        counts = new_counts;
    }

    // Count Type N strings: gp=1 and gs=1
    let mut ans: i64 = 0;
    for k in 0..=n {
        ans += counts[idx(k, 1, 1)];
    }

    println!("{}", ans);
}
