// Project Euler 818 - SET
// Sum of S(C)^4 over all 12-card subsets of the 81-card SET deck

const B: usize = 3;
const D: usize = 4;
const NCARDS: usize = 81;
const NN: usize = 12;
const KK: usize = 4;

fn main() {
    // Generate cards
    let mut cards = [[0u8; D]; NCARDS];
    for i in 0..NCARDS {
        let mut v = i;
        for d in (0..D).rev() {
            cards[i][d] = (v % B) as u8;
            v /= B;
        }
    }

    fn card_index(c: &[u8; D]) -> usize {
        let mut idx = 0;
        for d in 0..D {
            idx = idx * B + c[d] as usize;
        }
        idx
    }

    // Find all SETs
    let mut sets = Vec::new();
    for i in 0..NCARDS {
        for j in (i + 1)..NCARDS {
            let mut c3 = [0u8; D];
            for d in 0..D {
                c3[d] = ((B as i32 * 2 - cards[i][d] as i32 - cards[j][d] as i32) % B as i32) as u8;
            }
            let k = card_index(&c3);
            if k > j {
                sets.push([i, j, k]);
            }
        }
    }
    let num_sets = sets.len();

    // Precompute nCr
    let mut ncr_table = vec![vec![0i64; NN + 1]; NCARDS + 1];
    for n in 0..=NCARDS {
        ncr_table[n][0] = 1;
        for r in 1..=NN.min(n) {
            ncr_table[n][r] = ncr_table[n - 1][r - 1] + ncr_table[n - 1][r];
        }
    }

    let mut ans: i64 = 0;

    for e in 1..=KK {
        let mut num_distinct_counts = vec![0i64; NN + 1];
        let mut card_counts = vec![0i32; NCARDS];

        // Start with set 0
        for &ci in &sets[0] {
            card_counts[ci] = 1;
        }

        // Recursive helper
        fn helper(
            min_set_idx: usize,
            num_remaining: usize,
            num_distinct: usize,
            sets: &[[usize; 3]],
            card_counts: &mut Vec<i32>,
            num_distinct_counts: &mut Vec<i64>,
        ) {
            if num_remaining == 0 {
                if num_distinct <= NN {
                    num_distinct_counts[num_distinct] += 1;
                }
                return;
            }
            for si in min_set_idx..sets.len() {
                let mut new_distinct = num_distinct;
                for &ci in &sets[si] {
                    card_counts[ci] += 1;
                    if card_counts[ci] == 1 { new_distinct += 1; }
                }
                if new_distinct <= NN {
                    helper(si + 1, num_remaining - 1, new_distinct, sets, card_counts, num_distinct_counts);
                }
                for &ci in &sets[si] {
                    card_counts[ci] -= 1;
                }
            }
        }

        helper(1, e - 1, B, &sets, &mut card_counts, &mut num_distinct_counts);

        // Compute num_shapes via inclusion-exclusion
        let mut num_shapes: i64 = 0;
        for i in 0..=e {
            let sign = if i % 2 == 0 { 1i64 } else { -1 };
            let mut binom = 1i64;
            for j in 0..i {
                binom = binom * (e - j) as i64 / (j + 1) as i64;
            }
            let mut pw = 1i64;
            for _ in 0..KK {
                pw *= (e - i) as i64;
            }
            num_shapes += sign * binom * pw;
        }

        for k in 0..=NN {
            if num_distinct_counts[k] == 0 { continue; }
            ans += (num_shapes / e as i64)
                * num_sets as i64
                * ncr_table[NCARDS - k][NN - k]
                * num_distinct_counts[k];
        }

        // Reset card_counts
        for &ci in &sets[0] {
            card_counts[ci] = 0;
        }
    }

    println!("{}", ans);
}
