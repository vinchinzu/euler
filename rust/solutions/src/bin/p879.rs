// Project Euler 879 - Touch Screen Passwords on 4x4 Grid
// DFS with bitmask DP. 16 points, 2^16 = 65536 states.

const NUM_POINTS: usize = 16;
const WIDTH: usize = 4;

fn gcd(mut a: i32, mut b: i32) -> i32 {
    if a < 0 { a = -a; }
    if b < 0 { b = -b; }
    while b != 0 { let t = b; b = a % b; a = t; }
    a
}

fn main() {
    // Precompute intermediates
    let mut inters = [[0u32; NUM_POINTS]; NUM_POINTS];
    for p1 in 0..NUM_POINTS {
        for p2 in 0..NUM_POINTS {
            let r1 = (p1 / WIDTH) as i32;
            let c1 = (p1 % WIDTH) as i32;
            let r2 = (p2 / WIDTH) as i32;
            let c2 = (p2 % WIDTH) as i32;
            let dx = r2 - r1;
            let dy = c2 - c1;
            let g = gcd(dx, dy);
            if g > 1 {
                let sx = dx / g;
                let sy = dy / g;
                for k in 1..g {
                    let mr = r1 + k * sx;
                    let mc = c1 + k * sy;
                    let idx = mr as usize * WIDTH + mc as usize;
                    inters[p1][p2] |= 1 << idx;
                }
            }
        }
    }

    // DP memo: memo[current][used_mask]
    let mut memo = vec![vec![0i64; 1 << NUM_POINTS]; NUM_POINTS];
    let mut memo_set = vec![vec![false; 1 << NUM_POINTS]; NUM_POINTS];

    fn dfs(
        current: usize, used_mask: u32,
        inters: &[[u32; NUM_POINTS]; NUM_POINTS],
        memo: &mut Vec<Vec<i64>>,
        memo_set: &mut Vec<Vec<bool>>,
    ) -> i64 {
        if memo_set[current][used_mask as usize] {
            return memo[current][used_mask as usize];
        }

        let mut count = 1i64; // Current path is valid

        for next in 0..NUM_POINTS {
            if used_mask & (1 << next) != 0 { continue; }
            let req = inters[current][next];
            if (req & used_mask) == req {
                count += dfs(next, used_mask | (1 << next), inters, memo, memo_set);
            }
        }

        memo[current][used_mask as usize] = count;
        memo_set[current][used_mask as usize] = true;
        count
    }

    let mut total: i64 = 0;
    for start in 0..NUM_POINTS {
        total += dfs(start, 1 << start, &inters, &mut memo, &mut memo_set) - 1;
    }

    println!("{}", total);
}
