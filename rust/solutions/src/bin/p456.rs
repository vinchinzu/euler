// Project Euler 456: Triangles containing the origin
// Generate 2M points using PRNG. Count triangles containing origin.
// Group by angle ray, sliding window subtraction.

const NPOINTS: usize = 2_000_000;

fn quadrant(x: i32, y: i32) -> i32 {
    if y > 0 { return if x >= 0 { 0 } else { 1 }; }
    if y < 0 { return if x <= 0 { 2 } else { 3 }; }
    if x > 0 { 0 } else { 2 }
}

fn ncr2(n: i64) -> i64 { if n < 2 { 0 } else { n * (n - 1) / 2 } }
fn ncr3(n: i64) -> i64 { if n < 3 { 0 } else { n * (n - 1) * (n - 2) / 6 } }

fn main() {
    let mut px = vec![0i32; NPOINTS];
    let mut py = vec![0i32; NPOINTS];

    let mut n1: i32 = 1;
    let mut n2: i32 = 1;
    for i in 0..NPOINTS {
        n1 = (n1 as i64 * 1248 % 32323) as i32;
        n2 = (n2 as i64 * 8421 % 30103) as i32;
        px[i] = n1 - 16161;
        py[i] = n2 - 15051;
    }

    // Sort indices by angle
    let mut idx: Vec<usize> = (0..NPOINTS).collect();
    idx.sort_by(|&a, &b| {
        let qa = quadrant(px[a], py[a]);
        let qb = quadrant(px[b], py[b]);
        if qa != qb { return qa.cmp(&qb); }
        let cross = px[a] as i64 * py[b] as i64 - py[a] as i64 * px[b] as i64;
        if cross > 0 { std::cmp::Ordering::Less }
        else if cross < 0 { std::cmp::Ordering::Greater }
        else { std::cmp::Ordering::Equal }
    });

    // Group into rays
    let mut ray_start: Vec<usize> = Vec::new();
    let mut ray_len: Vec<usize> = Vec::new();
    let mut i = 0;
    while i < NPOINTS {
        let mut j = i + 1;
        while j < NPOINTS {
            let qi = quadrant(px[idx[i]], py[idx[i]]);
            let qj = quadrant(px[idx[j]], py[idx[j]]);
            if qi != qj { break; }
            let cross = px[idx[i]] as i64 * py[idx[j]] as i64 - py[idx[i]] as i64 * px[idx[j]] as i64;
            if cross != 0 { break; }
            j += 1;
        }
        ray_start.push(i);
        ray_len.push(j - i);
        i = j;
    }
    let num_rays = ray_start.len();

    // Build window: all points from rays[1..] first
    let mut window: Vec<usize> = Vec::with_capacity(2 * NPOINTS);
    for r in 1..num_rays {
        for k in ray_start[r]..ray_start[r] + ray_len[r] {
            window.push(idx[k]);
        }
    }

    let mut ans = ncr3(NPOINTS as i64);
    let mut start = 0usize;
    let mut end = window.len();

    for r in 0..num_rays {
        let rp = idx[ray_start[r]];
        let rpx = px[rp];
        let rpy = py[rp];
        let rlen = ray_len[r] as i64;

        // Advance start past points where cross(ray_rep, window[start]) > 0
        while start < end {
            let wi = window[start];
            let cross = rpx as i64 * py[wi] as i64 - rpy as i64 * px[wi] as i64;
            if cross <= 0 { break; }
            start += 1;
        }

        ans -= ncr2((end - start) as i64) * rlen;

        // Advance past collinear (cross == 0)
        while start < end {
            let wi = window[start];
            let cross = rpx as i64 * py[wi] as i64 - rpy as i64 * px[wi] as i64;
            if cross != 0 { break; }
            start += 1;
        }

        ans -= (end - start) as i64 * ncr2(rlen);
        ans -= ncr3(rlen);

        // Add this ray's points to window
        for k in ray_start[r]..ray_start[r] + ray_len[r] {
            window.push(idx[k]);
            end += 1;
        }
    }

    println!("{ans}");
}
