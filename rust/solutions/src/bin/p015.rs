// Project Euler 15: Lattice paths in a 20x20 grid
// Answer is C(40, 20) = 40! / (20! * 20!)

fn main() {
    let mut result: u64 = 1;
    for i in 1..=20u64 {
        result = result * (20 + i) / i;
    }
    println!("{result}");
}
