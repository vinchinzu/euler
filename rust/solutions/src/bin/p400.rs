// Project Euler 400: Fibonacci Tree Game

const N: usize = 10_000;
const L: usize = 8192;
const MOD: i64 = 1_000_000_000_000_000_000;

fn main() {
    let mut f = vec![0usize; N + 1];
    f[1] = 1;
    for k in 2..=N {
        f[k] = (f[k - 1] ^ f[k - 2]) + 1;
    }

    let mut g_prev2 = vec![0i64; L + 1];
    let mut g_prev = vec![0i64; L + 1];
    let mut g_cur = vec![0i64; L + 1];
    g_prev[0] = 1;

    for k in 2..=N {
        g_cur.fill(0);

        let fk2 = f[k - 2];
        for n in 0..L {
            // SAFETY: n < L, arrays have size L+1
            let gp = unsafe { *g_prev.get_unchecked(n) };
            if gp == 0 {
                continue;
            }
            let target = (n ^ fk2) + 1;
            if target <= L {
                // SAFETY: target <= L checked, arrays have size L+1
                unsafe {
                    let cur = g_cur.get_unchecked_mut(target);
                    *cur = (*cur + gp) % MOD;
                }
            }
        }

        let fk1 = f[k - 1];
        for n in 0..L {
            // SAFETY: n < L, arrays have size L+1
            let gp2 = unsafe { *g_prev2.get_unchecked(n) };
            if gp2 == 0 {
                continue;
            }
            let target = (fk1 ^ n) + 1;
            if target <= L {
                // SAFETY: target <= L checked, arrays have size L+1
                unsafe {
                    let cur = g_cur.get_unchecked_mut(target);
                    *cur = (*cur + gp2) % MOD;
                }
            }
        }

        g_cur[0] = (g_cur[0] + 1) % MOD;

        std::mem::swap(&mut g_prev2, &mut g_prev);
        std::mem::swap(&mut g_prev, &mut g_cur);
    }

    println!("{}", g_prev[1]);
}
