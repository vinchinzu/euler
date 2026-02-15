// Project Euler 371: Expected plates until a 1000-sum pair

fn main() {
    let mut e = [[0.0f64; 2]; 500];

    e[499][1] = 1.0 / (1.0 - 500.0 / 1000.0);
    e[499][0] = (1.0 + (1.0 / 1000.0) * e[499][1]) / (1.0 - 500.0 / 1000.0);

    for k in (0..=498).rev() {
        let p_stay = (1.0 + k as f64) / 1000.0;
        let p_new = 2.0 * (499 - k) as f64 / 1000.0;
        let p_500_0 = 1.0 / 1000.0;

        // h=1 first
        e[k][1] = (1.0 + p_new * e[k + 1][1]) / (1.0 - p_stay);
        // h=0
        e[k][0] = (1.0 + p_new * e[k + 1][0] + p_500_0 * e[k][1]) / (1.0 - p_stay);
    }

    println!("{:.8}", e[0][0]);
}
