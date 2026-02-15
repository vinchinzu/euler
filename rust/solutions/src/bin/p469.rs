fn main() {
    let limit = 1000;
    let mut es = vec![0.0f64; limit];
    let mut ans = 0.0;
    let mut prev_ans = -1.0f64;

    for i in 0..limit {
        let mut e = 0.0;
        if i > 1 {
            for j in 1..=(i - 2) {
                e += (1.0 + es[j] + es[i - j - 1]) / (i - 2) as f64;
            }
        }
        es[i] = e;
        let cand = if i > 0 { (i as f64 - e) / (i as f64 + 1.0) } else { 0.0 };
        if cand == prev_ans { break; }
        prev_ans = cand;
        ans = cand;
    }

    println!("{:.14}", ans);
}
