// Project Euler 709 - Even Stevens
//
// dp[n] = sum_{k even} C(n-1, k) * dp[k] * dp[n-k-1]
// N = 24680, mod = 1020202009. Uses rolling Pascal's triangle row.

const N: usize = 24680;
const M: i64 = 1_020_202_009;

fn main() {
    let mut dp = vec![0i64; N + 1];
    dp[0] = 1;

    // C_row[j] = C(current_n, j), starts with C(0, j)
    let mut c_row = vec![0i64; N + 1];
    c_row[0] = 1;

    for i in 1..=N {
        // c_row currently holds C(i-1, j)
        let mut val: i64 = 0;
        let mut k = 0;
        while k < i {
            val = (val + c_row[k] % M * (dp[k] % M) % M * (dp[i - k - 1] % M)) % M;
            k += 2;
        }
        dp[i] = val;

        // Update c_row from C(i-1, j) to C(i, j)
        let mut prev: i64 = 0;
        for j in 0..=i {
            let tmp = c_row[j];
            c_row[j] = (prev + c_row[j]) % M;
            prev = tmp;
        }
    }

    println!("{}", dp[N]);
}
