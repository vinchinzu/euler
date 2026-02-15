// Project Euler 877 - XOR-Product Equation
// Recurrence generating GF(2) solutions, XOR accumulator

fn main() {
    let limit: u64 = 1_000_000_000_000_000_000; // 10^18
    let mut total: u64 = 0;
    let mut a: u64 = 0;
    let mut b: u64 = 3;

    while b <= limit {
        total ^= b;
        let new_a = b;
        let new_b = (b << 1) ^ a;
        a = new_a;
        b = new_b;
    }

    println!("{}", total);
}
