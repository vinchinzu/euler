const NSUITS: usize = 4;
const NRANKS: usize = 13;
const NPATTERNS: usize = 16;

fn check_hall(g: &[i32; NPATTERNS]) -> bool {
    for s in 1..16u32 {
        let ssize = s.count_ones() as i32;
        let mut ns = 0;
        for v in 1..16u32 {
            if v & s != 0 && g[v as usize] > 0 {
                ns += g[v as usize];
            }
        }
        if ns < ssize { return false; }
    }
    true
}

fn main() {
    let mut fact = [1i64; NRANKS + 1];
    for i in 1..=NRANKS {
        fact[i] = fact[i - 1] * i as i64;
    }

    let pattern_size: Vec<u32> = (0..NPATTERNS).map(|v| (v as u32).count_ones()).collect();

    let mut f_count = [0i64; 53];
    let mut g_vec = [0i32; NPATTERNS];

    fn enumerate(
        idx: usize, rem: i32, g_vec: &mut [i32; NPATTERNS],
        f_count: &mut [i64; 53], fact: &[i64; NRANKS + 1],
        pattern_size: &[u32],
    ) {
        if idx == NPATTERNS - 1 {
            g_vec[idx] = rem;

            let n: usize = (0..NPATTERNS).map(|v| g_vec[v] as usize * pattern_size[v] as usize).sum();

            if n >= 4 && n <= 13 && check_hall(g_vec) {
                let mut denom = 1i64;
                for v in 0..NPATTERNS {
                    denom *= fact[g_vec[v] as usize];
                }
                let coeff = fact[NRANKS] / denom;
                f_count[n] += coeff;
            }

            g_vec[idx] = 0;
            return;
        }

        for k in 0..=rem {
            g_vec[idx] = k;
            enumerate(idx + 1, rem - k, g_vec, f_count, fact, pattern_size);
        }
        g_vec[idx] = 0;
    }

    enumerate(0, NRANKS as i32, &mut g_vec, &mut f_count, &fact, &pattern_size);

    let total: i64 = f_count[4..=13].iter().sum();
    println!("{}", total);
}
