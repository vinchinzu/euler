// Project Euler 732 - Standing on the Shoulders of Trolls
//
// Knapsack DP with left/right splitting.

const NTROLLS: usize = 1000;
const MOD_VAL: i64 = 1_000_000_007;
const INF: i32 = 1_000_000_000;

struct Troll {
    h: i32,
    l: i32,
    q: i32,
}

fn generate_trolls() -> Vec<Troll> {
    let mut trolls = Vec::with_capacity(NTROLLS);
    let mut r: i64 = 1;
    for _ in 0..NTROLLS {
        let h = ((r % 101) + 101) % 101 + 50;
        r = r * 5 % MOD_VAL;
        let l = ((r % 101) + 101) % 101 + 50;
        r = r * 5 % MOD_VAL;
        let q = ((r % 101) + 101) % 101 + 50;
        r = r * 5 % MOD_VAL;
        trolls.push(Troll {
            h: h as i32,
            l: l as i32,
            q: q as i32,
        });
    }
    trolls
}

fn main() {
    let trolls = generate_trolls();

    let mut total_h: i32 = 0;
    let mut total_iq: i32 = 0;
    for t in &trolls {
        total_h += t.h;
        total_iq += t.q;
    }
    let d = (total_h as f64 / std::f64::consts::SQRT_2).ceil() as usize;

    // Right DP: right_all[k][j] = min IQ to reach distance >= j using trolls[NTROLLS-k..NTROLLS-1]
    let mut right_all = vec![vec![INF; d + 1]; NTROLLS];
    right_all[0][0] = 0;

    for k in 1..NTROLLS {
        right_all[k] = right_all[k - 1].clone();
        let t = &trolls[NTROLLS - k];
        let h = t.h as usize;
        let q = t.q;

        for j in (h..=d).rev() {
            let val = right_all[k][j - h];
            if val < INF && val + q < right_all[k][j] {
                right_all[k][j] = val + q;
            }
        }
        // Suffix minimum
        for j in (0..d).rev() {
            if right_all[k][j + 1] < right_all[k][j] {
                right_all[k][j] = right_all[k][j + 1];
            }
        }
    }

    // Left DP
    let mut ldp = vec![INF; d + 1];
    ldp[0] = 0;

    let mut ans: i32 = 0;

    for i in 0..NTROLLS {
        let dist_raw = d as i32 - trolls[i].h - trolls[i].l;
        if dist_raw >= 0 {
            let dist = dist_raw as usize;
            let rrow = &right_all[NTROLLS - 1 - i];
            for j in 0..=dist {
                let lv = ldp[j];
                let rv = rrow[dist - j];
                if lv < INF && rv < INF {
                    let iq_used = lv + rv;
                    let remaining = total_iq - iq_used;
                    if remaining > ans {
                        ans = remaining;
                    }
                }
            }
        }

        let h = trolls[i].h as usize;
        let q = trolls[i].q;
        for j in (h..=d).rev() {
            let val = ldp[j - h];
            if val < INF && val + q < ldp[j] {
                ldp[j] = val + q;
            }
        }
        for j in (0..d).rev() {
            if ldp[j + 1] < ldp[j] {
                ldp[j] = ldp[j + 1];
            }
        }
    }

    println!("{}", ans);
}
