const S: usize = 10_000;
const MAX_SPLIT: usize = 200;

fn solve_one(p: f64) -> f64 {
    let q = 1.0 - p;

    let mut qp = vec![0.0f64; S + 1];
    qp[0] = 1.0;
    for i in 1..=S {
        qp[i] = qp[i - 1] * q;
        if qp[i] < 1e-300 { qp[i] = 0.0; }
    }

    let mut t = vec![0.0f64; S + 1];
    let mut u = vec![0.0f64; S + 1];
    t[0] = 0.0;
    t[1] = 1.0;
    u[0] = 0.0;
    u[1] = 0.0;

    for n in 2..=S {
        let denom = 1.0 - qp[n];
        let best_u;

        if denom <= 1e-300 {
            best_u = n as f64;
        } else {
            let mut bu = n as f64;
            let lim = (n - 1).min(MAX_SPLIT);
            for g in 1..=lim {
                let qg = qp[g];
                let p_g_healthy = (qg - qp[n]) / denom;
                let p_g_infected = 1.0 - p_g_healthy;
                let cost = 1.0 + p_g_healthy * u[n - g] + p_g_infected * (u[g] + t[n - g]);
                if cost < bu { bu = cost; }
            }
            best_u = bu;
        }
        u[n] = best_u;

        let mut best_t = n as f64;
        let qn = qp[n];
        let pool_all = 1.0 + (1.0 - qn) * u[n];
        if pool_all < best_t { best_t = pool_all; }

        let lim = (n - 1).min(MAX_SPLIT);
        for g in 1..=lim {
            let qg = qp[g];
            let cost = 1.0 + qg * t[n - g] + (1.0 - qg) * (u[g] + t[n - g]);
            if cost < best_t { best_t = cost; }
        }

        t[n] = best_t;
    }

    t[S]
}

fn main() {
    let mut total = 0.0f64;
    for p_idx in 1..=50 {
        let p = p_idx as f64 * 0.01;
        total += solve_one(p);
    }
    println!("{:.6}", total);
}
