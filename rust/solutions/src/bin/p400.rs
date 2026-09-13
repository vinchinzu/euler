// Project Euler 400: Fibonacci Tree Game

const N: usize = 10_000;
const L: usize = 8192;
const MOD: i64 = 1_000_000_000_000_000_000; // 10^18

fn main() {
    let mut f = vec![0u32; N + 1];
    f[0] = 0;
    f[1] = 1;
    for k in 2..=N {
        f[k] = (f[k - 1] ^ f[k - 2]) + 1;
    }

    let mut g_prev2 = vec![0i64; L + 1];
    let mut g_prev = vec![0i64; L + 1];
    let mut g_cur = vec![0i64; L + 1];
    g_prev[0] = 1;

    for k in 2..=N {
        g_cur.iter_mut().for_each(|x| *x = 0);

        let fk2 = f[k - 2] as usize;
        for n in 0..L {
            if g_prev[n] == 0 { continue; }
            let target = (n ^ fk2) + 1;
            if target <= L {
                g_cur[target] = (g_cur[target] + g_prev[n]) % MOD;
            }
        }

        let fk1 = f[k - 1] as usize;
        for n in 0..L {
            if g_prev2[n] == 0 { continue; }
            let target = (fk1 ^ n) + 1;
            if target <= L {
                g_cur[target] = (g_cur[target] + g_prev2[n]) % MOD;
            }
        }

        g_cur[0] = (g_cur[0] + 1) % MOD;

        std::mem::swap(&mut g_prev2, &mut g_prev);
        std::mem::swap(&mut g_prev, &mut g_cur);
    }

    // After the loop, g_prev holds what was g_cur at the last iteration
    println!("{}", g_prev[1]);
}
