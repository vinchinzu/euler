// Project Euler 212: Combined Volume of Cuboids
use std::collections::HashMap;

const N_CUBOIDS: usize = 50000;
const L: i32 = 130;

struct Cuboid { x: i32, y: i32, z: i32, dx: i32, dy: i32, dz: i32 }

fn iround_down(n: i32, k: i32) -> i32 {
    let mut r = n % k;
    if r < 0 { r += k; }
    n - r
}

fn helper(cuboids: &[Cuboid], indices: &[usize], idx: usize,
          min_x: i32, min_y: i32, min_z: i32,
          max_x: i32, max_y: i32, max_z: i32,
          num_cuboids: i32) -> i64 {
    if min_x >= max_x || min_y >= max_y || min_z >= max_z { return 0; }
    if idx == indices.len() {
        if num_cuboids == 0 { return 0; }
        let vol = (max_x - min_x) as i64 * (max_y - min_y) as i64 * (max_z - min_z) as i64;
        return if num_cuboids % 2 == 0 { vol } else { -vol };
    }
    let c = &cuboids[indices[idx]];
    let result = helper(cuboids, indices, idx + 1, min_x, min_y, min_z, max_x, max_y, max_z, num_cuboids)
        + helper(cuboids, indices, idx + 1,
                 min_x.max(c.x), min_y.max(c.y), min_z.max(c.z),
                 max_x.min(c.x + c.dx), max_y.min(c.y + c.dy), max_z.min(c.z + c.dz),
                 num_cuboids + 1);
    result
}

fn main() {
    // Generate lagged Fibonacci sequence
    let mut s_seq = vec![0i32; 6 * N_CUBOIDS];
    for k in 1..=55usize {
        let kk = k as i64;
        s_seq[k - 1] = ((100003 - 200003 * kk + 300007 * kk * kk * kk) % 1000000) as i32;
    }
    for k in 55..(6 * N_CUBOIDS) {
        s_seq[k] = (s_seq[k - 24] + s_seq[k - 55]) % 1000000;
    }

    let mut cuboids = Vec::with_capacity(N_CUBOIDS);
    for i in 0..N_CUBOIDS {
        let idx = 6 * i;
        cuboids.push(Cuboid {
            x: s_seq[idx] % 10000,
            y: s_seq[idx + 1] % 10000,
            z: s_seq[idx + 2] % 10000,
            dx: s_seq[idx + 3] % 399 + 1,
            dy: s_seq[idx + 4] % 399 + 1,
            dz: s_seq[idx + 5] % 399 + 1,
        });
    }

    // Assign cuboids to sections
    let mut sections: HashMap<(i32, i32, i32), Vec<usize>> = HashMap::new();
    for i in 0..N_CUBOIDS {
        let c = &cuboids[i];
        let mut dx = 0;
        while dx < c.dx + L {
            let mut dy = 0;
            while dy < c.dy + L {
                let mut dz = 0;
                while dz < c.dz + L {
                    let sx = iround_down(c.x + dx, L);
                    let sy = iround_down(c.y + dy, L);
                    let sz = iround_down(c.z + dz, L);
                    sections.entry((sx, sy, sz)).or_default().push(i);
                    dz += L;
                }
                dy += L;
            }
            dx += L;
        }
    }

    let mut ans: i64 = 0;
    for (&(sx, sy, sz), indices) in &sections {
        ans -= helper(&cuboids, indices, 0, sx, sy, sz, sx + L, sy + L, sz + L, 0);
    }

    println!("{ans}");
}
