// Project Euler 816 - Shortest distance among points
// Generate 2M points via Blum Blum Shub, find closest pair
// Optimized sweep-line with integer arithmetic

fn main() {
    const NUM_POINTS: usize = 2_000_000;

    let mut points = Vec::with_capacity(NUM_POINTS);
    let mut s: u64 = 290797;
    let m: u64 = 50515093;

    for _ in 0..NUM_POINTS {
        let x = s;
        s = s.wrapping_mul(s) % m;
        let y = s;
        s = s.wrapping_mul(s) % m;
        points.push((x as u32, y as u32));
    }

    points.sort_unstable_by_key(|&(x, _)| x);

    let mut ans_sq: u64 = u64::MAX;
    
    for i in 0..NUM_POINTS {
        let (xi, yi) = points[i];
        
        for j in (i + 1)..NUM_POINTS {
            let (xj, yj) = points[j];
            
            let dx = (xj as u64) - (xi as u64);
            let dx_sq = dx * dx;
            if dx_sq >= ans_sq {
                break;
            }
            
            let dy = if yj >= yi {
                (yj as u64) - (yi as u64)
            } else {
                (yi as u64) - (yj as u64)
            };
            let dy_sq = dy * dy;
            
            if dy_sq >= ans_sq {
                continue;
            }
            
            let d_sq = dx_sq + dy_sq;
            if d_sq < ans_sq {
                ans_sq = d_sq;
            }
        }
    }

    println!("{:.9}", (ans_sq as f64).sqrt());
}
