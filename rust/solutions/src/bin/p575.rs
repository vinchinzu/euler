// Project Euler 575 - Wandering Robot
//
// Robot in N x N room. Steady state probability of being in square-numbered room.

fn is_square(n: i32) -> bool {
    let r = (n as f64).sqrt() as i32;
    r * r == n || (r + 1) * (r + 1) == n
}

fn main() {
    let n: i32 = 1000;
    let mut case1: i64 = 0;
    let mut case2: i64 = 0;

    for i in 0..n {
        for j in 0..n {
            let room = i * n + j + 1;
            if is_square(room) {
                let is_corner = (i == 0 || i == n - 1) && (j == 0 || j == n - 1);
                let is_side = i == 0 || i == n - 1 || j == 0 || j == n - 1;
                if is_corner {
                    case1 += 3;
                    case2 += 2;
                } else if is_side {
                    case1 += 4;
                    case2 += 3;
                } else {
                    case1 += 5;
                    case2 += 4;
                }
            }
        }
    }

    let total1 = n as f64 * (5.0 * n as f64 - 4.0);
    let total2 = 4.0 * n as f64 * (n as f64 - 1.0);
    let ans = (case1 as f64 / total1 + case2 as f64 / total2) / 2.0;

    println!("{:.12}", ans);
}
