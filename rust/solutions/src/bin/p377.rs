const MODULUS: i64 = 1_000_000_000;
const ORDER: usize = 18;
const MAX_INIT: usize = 40;

type Matrix = [[i64; ORDER]; ORDER];

fn mat_mult(a: &Matrix, b: &Matrix) -> Matrix {
    let mut c = [[0i64; ORDER]; ORDER];
    for i in 0..ORDER {
        for p in 0..ORDER {
            if a[i][p] == 0 { continue; }
            for j in 0..ORDER {
                c[i][j] = (c[i][j] + (a[i][p] as i128 * b[p][j] as i128 % MODULUS as i128) as i64) % MODULUS;
            }
        }
    }
    c
}

fn mat_pow(m: &Matrix, mut exp: i64) -> Matrix {
    let mut result = [[0i64; ORDER]; ORDER];
    for i in 0..ORDER { result[i][i] = 1; }
    let mut base = *m;
    while exp > 0 {
        if exp & 1 != 0 { result = mat_mult(&result, &base); }
        base = mat_mult(&base, &base);
        exp >>= 1;
    }
    result
}

fn main() {
    // Compute initial values via DP
    let dp_size = (MAX_INIT + 1) * (MAX_INIT + 1);
    let mut dp_cnt = vec![0i64; dp_size];
    let mut dp_sum = vec![0i64; dp_size];
    dp_cnt[0] = 1;

    for length in 1..=MAX_INIT {
        for digit in 1..=9 {
            for s in 0..=(MAX_INIT - digit) {
                let prev = (length - 1) * (MAX_INIT + 1) + s;
                let cur = length * (MAX_INIT + 1) + s + digit;
                let cnt = dp_cnt[prev];
                if cnt == 0 { continue; }
                let sum_val = (dp_sum[prev] * 10 + cnt * digit as i64) % MODULUS;
                dp_cnt[cur] = (dp_cnt[cur] + cnt) % MODULUS;
                dp_sum[cur] = (dp_sum[cur] + sum_val) % MODULUS;
            }
        }
    }

    let mut f_vals = vec![0i64; MAX_INIT + 1];
    let mut count_vals = vec![0i64; MAX_INIT + 1];

    for length in 1..=MAX_INIT {
        for s in 0..=MAX_INIT {
            let idx = length * (MAX_INIT + 1) + s;
            f_vals[s] = (f_vals[s] + dp_sum[idx]) % MODULUS;
            count_vals[s] = (count_vals[s] + dp_cnt[idx]) % MODULUS;
        }
    }

    // Build companion matrix
    let mut companion = [[0i64; ORDER]; ORDER];
    for k in 0..9 { companion[0][k] = 10; }
    for k in 0..9 { companion[0][9 + k] = (k + 1) as i64; }
    for i in 1..9 { companion[i][i - 1] = 1; }
    for k in 0..9 { companion[9][9 + k] = 1; }
    for i in 10..18 { companion[i][i - 1] = 1; }

    let compute_f_at = |n: i64| -> i64 {
        if n <= MAX_INIT as i64 { return f_vals[n as usize]; }
        let mut state = [0i64; ORDER];
        for i in 0..9 { state[i] = f_vals[9 - i]; }
        for i in 0..9 { state[i + 9] = count_vals[9 - i]; }
        let power = n - 9;
        let m = mat_pow(&companion, power);
        let mut result = 0i64;
        for j in 0..ORDER {
            result = (result + (m[0][j] as i128 * state[j] as i128 % MODULUS as i128) as i64) % MODULUS;
        }
        result
    };

    let mut total = 0i64;
    let mut power = 13i64;
    for _ in 1..=17 {
        let fn_val = compute_f_at(power);
        total = (total + fn_val) % MODULUS;
        power *= 13;
    }

    println!("{:09}", total);
}
