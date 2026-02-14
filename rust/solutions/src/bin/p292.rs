// Project Euler 292: Pythagorean Polygons
use std::collections::HashMap;
use euler_utils::gcd;

const PERIM: i32 = 120;

struct Dir {
    a: i32,
    b: i32,
    c: i32,
    angle: f64,
}

fn isqrt(n: i32) -> i32 {
    let mut r = (n as f64).sqrt() as i32;
    while r * r > n { r -= 1; }
    while (r + 1) * (r + 1) <= n { r += 1; }
    r
}

fn generate_directions() -> Vec<Dir> {
    let mut dirs = Vec::new();
    for a in -PERIM..=PERIM {
        for b in -PERIM..=PERIM {
            if a == 0 && b == 0 { continue; }
            let c2 = a * a + b * b;
            let c = isqrt(c2);
            if c * c != c2 || c > PERIM { continue; }
            if gcd(a.unsigned_abs() as u64, b.unsigned_abs() as u64) != 1 { continue; }
            dirs.push(Dir { a, b, c, angle: (b as f64).atan2(a as f64) });
        }
    }
    dirs.sort_by(|a, b| a.angle.partial_cmp(&b.angle).unwrap());
    dirs
}

fn encode(sx: i32, sy: i32, peri: i32, ne: i32) -> i64 {
    ((sx + PERIM) as i64 * 241 + (sy + PERIM) as i64) * (121 * 4) + peri as i64 * 4 + ne as i64
}

fn decode(key: i64) -> (i32, i32, i32, i32) {
    let ne = (key % 4) as i32;
    let k2 = key / 4;
    let peri = (k2 % 121) as i32;
    let k3 = k2 / 121;
    let sy = (k3 % 241) as i32 - PERIM;
    let sx = (k3 / 241) as i32 - PERIM;
    (sx, sy, peri, ne)
}

fn main() {
    let dirs = generate_directions();
    let num_dirs = dirs.len();

    // Build options for each direction
    let mut dir_opts: Vec<Vec<(i32, i32, i32)>> = Vec::new();
    for d in &dirs {
        let mut opts = Vec::new();
        let mut k = 1;
        while k * d.c <= PERIM {
            opts.push((k * d.a, k * d.b, k * d.c));
            k += 1;
        }
        dir_opts.push(opts);
    }

    let mid = num_dirs / 2;

    // First half DP
    let mut cur: HashMap<i64, i64> = HashMap::new();
    cur.insert(encode(0, 0, 0, 0), 1);

    for didx in 0..mid {
        let opts = &dir_opts[didx];
        if opts.is_empty() { continue; }

        let snapshot: Vec<(i64, i64)> = cur.iter().map(|(&k, &v)| (k, v)).collect();
        for &(key, cnt) in &snapshot {
            let (sx, sy, peri, ne) = decode(key);
            for &(dx, dy, dl) in opts {
                let new_peri = peri + dl;
                if new_peri <= PERIM {
                    let new_ne = if ne < 3 { ne + 1 } else { 3 };
                    let nk = encode(sx + dx, sy + dy, new_peri, new_ne);
                    *cur.entry(nk).or_insert(0) += cnt;
                }
            }
        }
    }

    // Group by (sx, sy, ne) with prefix sums over perimeter
    let mut groups: HashMap<(i32, i32, i32), Vec<i64>> = HashMap::new();
    for (&key, &cnt) in &cur {
        let (sx, sy, peri, ne) = decode(key);
        let g = groups.entry((sx, sy, ne)).or_insert_with(|| vec![0i64; PERIM as usize + 1]);
        g[peri as usize] += cnt;
    }

    for g in groups.values_mut() {
        for p in 1..=PERIM as usize {
            g[p] += g[p - 1];
        }
    }

    // Second half DP
    let mut cur2: HashMap<i64, i64> = HashMap::new();
    cur2.insert(encode(0, 0, 0, 0), 1);

    for didx in mid..num_dirs {
        let opts = &dir_opts[didx];
        if opts.is_empty() { continue; }

        let snapshot: Vec<(i64, i64)> = cur2.iter().map(|(&k, &v)| (k, v)).collect();
        for &(key, cnt) in &snapshot {
            let (sx, sy, peri, ne) = decode(key);
            for &(dx, dy, dl) in opts {
                let new_peri = peri + dl;
                if new_peri <= PERIM {
                    let new_ne = if ne < 3 { ne + 1 } else { 3 };
                    let nk = encode(sx + dx, sy + dy, new_peri, new_ne);
                    *cur2.entry(nk).or_insert(0) += cnt;
                }
            }
        }
    }

    // Combine
    let mut total: i64 = 0;
    for (&key, &cnt1) in &cur2 {
        let (sx2, sy2, p2, ne2) = decode(key);
        let remaining = PERIM - p2;
        if remaining < 0 { continue; }

        // Need to match with first half states at position (-sx2, -sy2) with ne1 such that ne1+ne2 >= 3
        let min_ne1 = (3 - ne2).max(0);
        for ne1 in min_ne1..4 {
            if let Some(g) = groups.get(&(-sx2, -sy2, ne1)) {
                total += cnt1 * g[remaining as usize];
            }
        }
    }

    println!("{}", total);
}
