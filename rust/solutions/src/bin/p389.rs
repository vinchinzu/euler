// Project Euler 389: Platonic Dice
// Hierarchical dice variance computation.

fn main() {
    let mu_t = 2.5_f64;
    let var_t = 1.25_f64;

    let mu_6 = 3.5_f64;
    let var_6 = 35.0 / 12.0;
    let e_c = mu_6 * mu_t;
    let var_c = var_6 * mu_t + mu_6 * mu_6 * var_t;

    let mu_8 = 4.5_f64;
    let var_8 = 63.0 / 12.0;
    let e_o = mu_8 * e_c;
    let var_o = var_8 * e_c + mu_8 * mu_8 * var_c;

    let mu_12 = 6.5_f64;
    let var_12 = 143.0 / 12.0;
    let e_d = mu_12 * e_o;
    let var_d = var_12 * e_o + mu_12 * mu_12 * var_o;

    let mu_20 = 10.5_f64;
    let var_20 = 399.0 / 12.0;
    let var_i = var_20 * e_d + mu_20 * mu_20 * var_d;

    println!("{:.4}", var_i);
}
