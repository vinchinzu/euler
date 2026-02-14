// Project Euler 150: Searching a triangular array for sub-triangles with minimum sum
//
// Uses prefix sums along right-slanting and left-slanting directions.

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

    // right_sums[j][i] and left_sums[i][j]
    let mut right_sums = vec![vec![0i64; NN]; NN];
    let mut left_sums = vec![vec![0i64; NN]; NN];

    for i in 0..NN {
        for j in 0..(NN - i) {
            let row = i + j;
            let idx = row * (row + 1) / 2 + i + 1;

            if i > 0 {
                right_sums[j][i] = right_sums[j + 1][i - 1] + s[idx];
            } else {
                right_sums[j][i] = s[idx];
            }

            if i > 0 {
                left_sums[i][j] = left_sums[i - 1][j + 1] + s[row * (row + 1) / 2 + i];
            } else {
                left_sums[i][j] = 0;
            }
        }
    }

    let mut ans = i64::MAX;

    for i in 0..NN {
        for j in 0..(NN - i) {
            let mut total_sum: i64 = 0;
            for k in 0..(NN - i - j) {
                total_sum += right_sums[i][j + k] - left_sums[j][i + k];
                if total_sum < ans {
                    ans = total_sum;
                }
            }
        }
    }

    println!("{}", ans);
}
