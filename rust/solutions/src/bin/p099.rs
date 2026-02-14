// Project Euler 99: Largest exponential
// Find the line number with the largest value using logarithms.

fn main() {
    let data = include_str!("../../../../data/0099_base_exp.txt");
    let mut max_value = 0.0f64;
    let mut max_line = 0usize;

    for (i, line) in data.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() { continue; }
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() != 2 { continue; }
        let base: f64 = parts[0].parse().unwrap();
        let exp: f64 = parts[1].parse().unwrap();
        let value = exp * base.ln();
        if value > max_value {
            max_value = value;
            max_line = i + 1;
        }
    }

    println!("{max_line}");
}
