// Project Euler 653 - Frictionless Tube
// Elastic collisions: marbles pass through each other, sort final distances.

fn main() {
    let m: i64 = 32745673;
    let mut x: i64 = 6563116;
    let n_marbles: usize = 1_000_001;
    let j_val: usize = 500_001;
    let l_val: i64 = 1_000_000_000;
    let l_adj = l_val - n_marbles as i64 * 20 / 2;

    let mut distances = vec![0i64; n_marbles];
    let mut pos: i64 = 0;

    for i in 0..n_marbles {
        let gap_before = (x % 1000) + 1;
        let is_west = x > 10_000_000;
        pos += gap_before;
        distances[i] = if is_west { l_adj + pos } else { l_adj - pos };
        x = (x * x) % m;
    }

    distances.sort();
    println!("{}", distances[n_marbles - j_val]);
}
