// Project Euler 321: Swapping Counters
// Recurrence: a[i] = 6*a[i-2] - a[i-4] + 4

fn main() {
    let mut seq = [0i64; 40];
    seq[0] = 1;
    seq[1] = 3;
    seq[2] = 10;
    seq[3] = 22;

    for i in 4..40 {
        seq[i] = 6 * seq[i - 2] - seq[i - 4] + 4;
    }

    let total: i64 = seq.iter().sum();
    println!("{}", total);
}
