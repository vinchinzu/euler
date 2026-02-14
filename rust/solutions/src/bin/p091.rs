// Project Euler 91: Right triangles with integer coordinates
// Count right triangles with one vertex at origin on a 50x50 grid.

use euler_utils::gcd;

fn main() {
    let n: i64 = 50;
    // Case 1: right angle at origin (OP . OQ = 0)
    let mut count = n * n;

    // Case 2: right angle at P
    for x1 in 0..=n {
        for y1 in 0..=n {
            if x1 == 0 && y1 == 0 {
                continue;
            }
            let g = gcd(x1 as u64, y1 as u64) as i64;
            let dx = -y1 / g;
            let dy = x1 / g;

            let mut k = 1;
            loop {
                let x2 = x1 + k * dx;
                let y2 = y1 + k * dy;
                if x2 < 0 || x2 > n || y2 < 0 || y2 > n {
                    break;
                }
                count += 1;
                k += 1;
            }
        }
    }

    // Case 3: right angle at Q (symmetric to case 2)
    count += count - n * n;

    println!("{count}");
}
