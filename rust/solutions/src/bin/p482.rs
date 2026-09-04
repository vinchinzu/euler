use rayon::prelude::*;

const N: i64 = 10_000_000;

#[inline(always)]
fn gcd_u32(mut a: u32, mut b: u32) -> u32 {
    if a == 0 { return b; }
    if b == 0 { return a; }
    let shift = (a | b).trailing_zeros();
    a >>= a.trailing_zeros();
    loop {
        b >>= b.trailing_zeros();
        if a > b { std::mem::swap(&mut a, &mut b); }
        b -= a;
        if b == 0 { break; }
    }
    a << shift
}

#[inline(always)]
fn gcd_u64(mut a: u64, mut b: u64) -> u64 {
    if a == 0 { return b; }
    if b == 0 { return a; }
    let shift = (a | b).trailing_zeros();
    a >>= a.trailing_zeros();
    loop {
        b >>= b.trailing_zeros();
        if a > b { std::mem::swap(&mut a, &mut b); }
        b -= a;
        if b == 0 { break; }
    }
    a << shift
}

#[inline(always)]
fn tr(n: i64) -> i64 {
    n * (n + 1) / 2
}

fn solve() -> i64 {
    let l_val = (N as f64 / 108.0f64.sqrt()) as usize;
    let map_size = l_val + 10;

    // 1st pass: count entries per index for CSR representation
    let mut counts = vec![0u32; map_size];
    let mut m = 2u32;
    while 2 * m * m <= N as u32 {
        let mut n = 1 + (m & 1);
        while n < m && 2 * m * (m + n) <= N as u32 {
            if gcd_u32(m, n) == 1 {
                let a = (m * m - n * n) as usize;
                let b = (2 * m * n) as usize;
                if a < map_size { counts[a] += 1; }
                if b < map_size { counts[b] += 1; }
            }
            n += 2;
        }
        m += 1;
    }

    // Prefix sums for CSR offsets
    let mut offsets = vec![0u32; map_size + 1];
    for i in 0..map_size {
        offsets[i + 1] = offsets[i] + counts[i];
    }
    let total_entries = offsets[map_size] as usize;
    let mut data = vec![0u32; total_entries];
    let mut pos = offsets.clone();

    // 2nd pass: populate contiguous data buffer
    let mut m = 2u32;
    while 2 * m * m <= N as u32 {
        let mut n = 1 + (m & 1);
        while n < m && 2 * m * (m + n) <= N as u32 {
            if gcd_u32(m, n) == 1 {
                let a = (m * m - n * n) as usize;
                let b = (2 * m * n) as u32;
                if a < map_size {
                    let p = pos[a] as usize;
                    data[p] = b;
                    pos[a] += 1;
                }
                let b_usize = b as usize;
                if b_usize < map_size {
                    let p = pos[b_usize] as usize;
                    data[p] = (m * m - n * n) as u32;
                    pos[b_usize] += 1;
                }
            }
            n += 2;
        }
        m += 1;
    }

    // Collect non-empty keys
    let mut keys: Vec<u32> = Vec::with_capacity(350_000);
    for i in 1..map_size {
        if offsets[i] != offsets[i + 1] {
            keys.push(i as u32);
        }
    }

    // Parallel search: on-the-fly divisor computation, zero vector allocations per key
    let mut candidates: Vec<(u32, u32, u32)> = keys
        .par_iter()
        .with_min_len(1024)
        .fold(Vec::new, |mut local_candidates, &a1| {
            let a1_usize = a1 as usize;
            let start1 = offsets[a1_usize] as usize;
            let end1 = offsets[a1_usize + 1] as usize;

            // Stack-allocated divisor buffer (numbers < 10^6 have at most 240 divisors)
            let mut divs = [0u32; 256];
            let mut num_divs = 0;
            let step = 1 + (a1 & 1);
            let mut i = 1u32;
            while i * i <= a1 {
                if a1 % i == 0 {
                    divs[num_divs] = i;
                    num_divs += 1;
                    let d2 = a1 / i;
                    if d2 != i {
                        divs[num_divs] = d2;
                        num_divs += 1;
                    }
                }
                i += step;
            }

            let a1_i64 = a1 as i64;
            let max_mult_limit = ((map_size - 1) as u32) / a1;

            for &d in &divs[..num_divs] {
                let q = a1 / d;
                let max_mult = q.min(max_mult_limit);

                for mult in 1..=max_mult {
                    if mult > 1 && gcd_u32(q, mult) != 1 {
                        continue;
                    }

                    let a2 = (mult * d) as usize;
                    let start2 = offsets[a2] as usize;
                    let end2 = offsets[a2 + 1] as usize;
                    if start2 == end2 {
                        continue;
                    }

                    let r = a1_i64 * mult as i64;
                    let r2 = r * r;
                    let mult_i64 = mult as i64;
                    let q_i64 = q as i64;

                    for &b1 in &data[start1..end1] {
                        let x = b1 as i64 * mult_i64;
                        if 2 * x >= N {
                            continue;
                        }

                        for &b2 in &data[start2..end2] {
                            let y = b2 as i64 * q_i64;
                            let xy_sum = x + y;
                            if 2 * xy_sum >= N {
                                continue;
                            }

                            let den = x * y - r2;
                            if den <= 0 {
                                continue;
                            }

                            let num = (r2 as u64) * (xy_sum as u64);
                            let z_approx = (num / den as u64) as i64;
                            if 2 * (xy_sum + z_approx) > N {
                                continue;
                            }

                            let num_mod = num % (den as u64);
                            let g = gcd_u64(num_mod, den as u64) as i64;
                            let den_r = den / g;
                            if den_r > N / 2 {
                                continue;
                            }

                            let num_r = (num / g as u64) as i64;
                            let perim = 2 * (xy_sum * den_r + num_r);
                            if perim <= N {
                                let mut sx = (x * den_r) as u32;
                                let mut sy = (y * den_r) as u32;
                                let mut sz = num_r as u32;
                                if sx > sy { std::mem::swap(&mut sx, &mut sy); }
                                if sy > sz { std::mem::swap(&mut sy, &mut sz); }
                                if sx > sy { std::mem::swap(&mut sx, &mut sy); }
                                local_candidates.push((sx, sy, sz));
                            }
                        }
                    }
                }
            }
            local_candidates
        })
        .reduce(Vec::new, |mut a, b| {
            a.extend(b);
            a
        });

    // In-place sort and deduplicate candidates (~12k elements, < 1 ms)
    candidates.sort_unstable();
    candidates.dedup();

    // Accumulate total perimeter and incenter-to-vertex distances
    let mut ans: i64 = 0;
    for &(sx, sy, sz) in &candidates {
        let x = sx as i64;
        let y = sy as i64;
        let z = sz as i64;
        let sum_xyz = x + y + z;
        let r2 = (x * y * z) / sum_xyz;
        let perim = 2 * sum_xyz;
        let ia = (r2 + x * x).isqrt();
        let ib = (r2 + y * y).isqrt();
        let ic = (r2 + z * z).isqrt();
        ans += tr(N / perim) * (perim + ia + ib + ic);
    }
    ans
}

fn main() {
    println!("{}", solve());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_answer() {
        assert_eq!(solve(), 1400824879147);
    }
}
