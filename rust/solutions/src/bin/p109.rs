// Project Euler 109: Darts
fn main() {
    let mut pre_scores = Vec::new();
    pre_scores.push(0); // miss
    for i in 1..=20 { pre_scores.push(i); }       // singles
    for i in 1..=20 { pre_scores.push(2 * i); }   // doubles
    for i in 1..=20 { pre_scores.push(3 * i); }   // trebles
    pre_scores.push(25);  // outer bull
    pre_scores.push(50);  // inner bull (D25)

    let mut finish_scores = Vec::new();
    for i in 1..=20 { finish_scores.push(2 * i); }
    finish_scores.push(50); // D25

    let limit = 99;
    let mut total = 0;

    for &f in &finish_scores {
        for i in 0..pre_scores.len() {
            for j in i..pre_scores.len() {
                let score = pre_scores[i] + pre_scores[j] + f;
                if score >= 1 && score <= limit {
                    total += 1;
                }
            }
        }
    }

    println!("{}", total);
}
