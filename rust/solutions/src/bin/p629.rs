// Project Euler 629 - Scatterstone Nim
// Nimber computation + partition DP with XOR tracking

const NN: usize = 200;
const MOD: i64 = 1_000_000_007;
const XS: usize = 256;

fn compute_nimbers2() -> Vec<i32> {
    let mut nim = vec![0i32; NN + 1];
    for n in 2..=NN {
        let mut used = [false; XS];
        for i in 1..n {
            let v = (nim[i] ^ nim[n - i]) as usize;
            if v < XS { used[v] = true; }
        }
        let mut mex = 0;
        while mex < XS && used[mex] { mex += 1; }
        nim[n] = mex as i32;
    }
    nim
}

fn compute_nimbers3() -> Vec<i32> {
    let mut nim = vec![0i32; NN + 1];
    for n in 2..=NN {
        let mut used = [false; XS];
        for i in 1..n {
            let v = (nim[i] ^ nim[n - i]) as usize;
            if v < XS { used[v] = true; }
        }
        for i in 1..n {
            for j in i..=(n - i) {
                let k = n - i - j;
                if k >= j {
                    let v = (nim[i] ^ nim[j] ^ nim[k]) as usize;
                    if v < XS { used[v] = true; }
                }
            }
        }
        let mut mex = 0;
        while mex < XS && used[mex] { mex += 1; }
        nim[n] = mex as i32;
    }
    nim
}

fn compute_nimbers4() -> Vec<i32> {
    let mut nim = vec![0i32; NN + 1];
    for n in 2..=NN { nim[n] = n as i32 - 1; }
    nim
}

fn count_winning(nimbers: &[i32]) -> i64 {
    // dp[a][x] = number of partitions of a with XOR of nimbers = x
    let sz = (NN + 1) * XS;
    let mut dp = vec![0i64; sz];
    dp[0 * XS + 0] = 1;

    let mut tmp = vec![0i64; sz];

    for d in 1..=NN {
        let g = nimbers[d] as usize;
        for v in tmp.iter_mut() { *v = 0; }
        for a in d..=NN {
            for x in 0..XS {
                let px = x ^ g;
                // SAFETY: indices a-d and px are in bounds
                tmp[a * XS + x] = (dp[(a - d) * XS + px] + tmp[(a - d) * XS + px]) % MOD;
            }
        }
        for a in 0..=NN {
            for x in 0..XS {
                dp[a * XS + x] = (dp[a * XS + x] + tmp[a * XS + x]) % MOD;
            }
        }
    }

    let mut result = 0i64;
    for x in 1..XS {
        result = (result + dp[NN * XS + x]) % MOD;
    }
    result
}

fn main() {
    let n2 = compute_nimbers2();
    let n3 = compute_nimbers3();
    let n4 = compute_nimbers4();

    let f2 = count_winning(&n2);
    let f3 = count_winning(&n3);
    let f4 = count_winning(&n4);

    let ans = (f2 + f3 + (NN as i64 - 3) % MOD * (f4 % MOD)) % MOD;
    println!("{}", ans);
}
