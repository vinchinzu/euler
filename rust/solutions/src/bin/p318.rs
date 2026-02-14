// Project Euler 318: Leading Digits of Power Sums

fn main() {
    let limit = 2011;
    let exponent = 2011;
    let mut total: i64 = 0;

    for p in 1..limit {
        let sp = (p as f64).sqrt();
        let q_max_beta = ((sp + 1.0) * (sp + 1.0)) as i32;
        let q_max_sum = limit - p;
        let q_end = q_max_beta.min(q_max_sum);

        if q_end <= p { continue; }

        for q in (p + 1)..=q_end {
            let sq = (q as f64).sqrt();
            let beta = sq - sp;
            if beta >= 1.0 { continue; }

            let log_beta = beta.log10();
            let denom = -2.0 * log_beta;
            if denom <= 0.0 { continue; }

            let n_val = exponent as f64 / denom;
            let n = n_val.ceil() as i64;
            let n = n.max(1);
            total += n;
        }
    }

    println!("{}", total);
}
