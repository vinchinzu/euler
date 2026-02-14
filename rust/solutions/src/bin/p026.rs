// Project Euler 026: Reciprocal Cycles
// Find d < 1000 for which 1/d has the longest recurring cycle in its decimal expansion.

fn cycle_length(d: u32) -> u32 {
    if d % 2 == 0 || d % 5 == 0 {
        return 0;
    }
    let mut seen = vec![-1i32; d as usize + 1];
    let mut rem = 1u32;
    let mut pos = 0u32;

    loop {
        if rem == 0 {
            return 0;
        }
        if seen[rem as usize] >= 0 {
            return pos - seen[rem as usize] as u32;
        }
        seen[rem as usize] = pos as i32;
        rem = (rem * 10) % d;
        pos += 1;
    }
}

fn main() {
    let (best_d, _) = (1..1000u32)
        .filter(|d| d % 2 != 0 && d % 5 != 0)
        .map(|d| (d, cycle_length(d)))
        .max_by_key(|&(_, c)| c)
        .unwrap();

    println!("{best_d}");
}
