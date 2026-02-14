// Project Euler 221: Alexandrian Integers

fn main() {
    let n_target = 150_000usize;
    let l = 80_000i64;
    let mut alexandrians: Vec<i64> = Vec::new();

    for a in 1..=l {
        let n = a * a + 1;
        let sq = (n as f64).sqrt() as i64;
        for d in 1..=sq {
            if n % d == 0 {
                let e = n / d;
                let p = a + d;
                let q = a + e;
                let a_val = a * p;
                if a_val <= 9_000_000_000_000_000_000i64 / q {
                    alexandrians.push(a_val * q);
                }
            }
        }
    }

    alexandrians.sort();
    alexandrians.dedup();

    println!("{}", alexandrians[n_target - 1]);
}
