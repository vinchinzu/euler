// Project Euler 815 - Grouping Cards
// K=4 copies of N=60 values, find expected max distinct values present
//
// Recurrence is independent of current max except via max(m, distinct).
// Compute the entire 61-wide E(c, ·) row per composition so inner loops
// are sequential and SIMD-friendly. Occupancy is a u8 per composition
// (not NaN in a 310MB f64 table).

const N: usize = 60;
const K: usize = 4;
const MAX_VAL: usize = N + 1;

fn index_from_counts(c: &[usize; K + 1], table: &[[i64; K + 2]]) -> usize {
    let mut total: i32 = -1;
    let mut idx: usize = 0;
    for i in 0..K {
        total += c[i] as i32 + 1;
        if total >= (i + 1) as i32 {
            idx += table[total as usize][i + 1] as usize;
        }
    }
    idx
}

fn solve_row(
    c: &mut [usize; K + 1],
    cache: &mut [f64],
    seen: &mut [u8],
    ncr_table: &[[i64; K + 2]],
) -> [f64; MAX_VAL] {
    let idx = index_from_counts(c, ncr_table);
    if unsafe { *seen.get_unchecked(idx) } != 0 {
        let mut row = [0.0f64; MAX_VAL];
        let base = idx * MAX_VAL;
        unsafe {
            let src = cache.as_ptr().add(base);
            for m in 0..MAX_VAL {
                row[m] = *src.add(m);
            }
        }
        return row;
    }

    let mut remaining = 0usize;
    for i in 0..K {
        remaining += (K - i) * c[i];
    }
    let mut acc = [0.0f64; MAX_VAL];
    if remaining == 0 {
        for m in 0..MAX_VAL {
            acc[m] = m as f64;
        }
    } else {
        for t in 0..K {
            if c[t] > 0 {
                let count = c[t];
                c[t] -= 1;
                c[t + 1] += 1;
                let distinct = N - c[0] - c[K];
                let child = solve_row(c, cache, seen, ncr_table);
                let w = (K - t) as f64 * count as f64;
                let d = distinct.min(MAX_VAL - 1);
                let cd = child[d];
                for m in 0..d {
                    acc[m] += w * cd;
                }
                for m in d..MAX_VAL {
                    acc[m] += w * child[m];
                }
                c[t] += 1;
                c[t + 1] -= 1;
            }
        }
        let inv = 1.0 / remaining as f64;
        for m in 0..MAX_VAL {
            acc[m] *= inv;
        }
    }

    let base = idx * MAX_VAL;
    unsafe {
        let dst = cache.as_mut_ptr().add(base);
        for m in 0..MAX_VAL {
            *dst.add(m) = acc[m];
        }
        *seen.get_unchecked_mut(idx) = 1;
    }
    acc
}

fn main() {
    let mut ncr_table = vec![[0i64; K + 2]; N + K + 1];
    for n in 0..=N + K {
        ncr_table[n][0] = 1;
        for r in 1..=K + 1 {
            if r <= n {
                ncr_table[n][r] = ncr_table[n - 1][r - 1] + ncr_table[n - 1][r];
            }
        }
    }

    const MAX_INDEX: usize = 635376;
    let mut cache = vec![0.0f64; MAX_INDEX * MAX_VAL];
    let mut seen = vec![0u8; MAX_INDEX];

    let mut c = [0usize; K + 1];
    c[0] = N;

    let row = solve_row(&mut c, &mut cache, &mut seen, &ncr_table);
    println!("{:.8}", row[0]);
}
