// Project Euler 150: Searching a triangular array for sub-triangles with minimum sum
//
// Uses prefix sums along right-slanting and left-slanting directions.
// Flat 1D arrays instead of Vec<Vec<>> for cache-friendly access.

fn main() {
    const NN: usize = 1000;
    let total = NN * (NN + 1) / 2;

    // Generate S[k] for k = 1..total
    let mut s = vec![0i64; total + 1];
    let mut t: i64 = 0;
    for k in 1..=total {
        t = (615949 * t + 797807) % (1 << 20);
        s[k] = t - (1 << 19);
    }

    // Flat arrays: right_sums[j * NN + i] and left_sums[i * NN + j]
    let mut right_sums = vec![0i64; NN * NN];
    let mut left_sums = vec![0i64; NN * NN];

    for i in 0..NN {
        for j in 0..(NN - i) {
            let row = i + j;
            let idx = row * (row + 1) / 2 + i + 1;

            if i > 0 {
                right_sums[j * NN + i] = right_sums[(j + 1) * NN + (i - 1)] + s[idx];
            } else {
                right_sums[j * NN + i] = s[idx];
            }

            if i > 0 {
                left_sums[i * NN + j] = left_sums[(i - 1) * NN + (j + 1)] + s[row * (row + 1) / 2 + i];
            } else {
                left_sums[i * NN + j] = 0;
            }
        }
    }

    let mut ans = i64::MAX;

    for i in 0..NN {
        for j in 0..(NN - i) {
            let mut total_sum: i64 = 0;
            for k in 0..(NN - i - j) {
                unsafe {
                    total_sum += *right_sums.get_unchecked(i * NN + j + k)
                               - *left_sums.get_unchecked(j * NN + i + k);
                }
                if total_sum < ans {
                    ans = total_sum;
                }
            }
        }
    }

    println!("{}", ans);
}
