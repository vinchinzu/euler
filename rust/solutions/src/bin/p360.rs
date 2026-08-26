// Project Euler 360: Scary Sphere
//
// S(r) = 6r + 24 * Σ_{x=1}^{r-1} x * χ((r-x)(r+x)), χ = r₂/4.
// r = 5^10, so gcd(r-x, r+x) | 2·5^10 and
//   χ(ab) = χ(a)χ(b) / ((v₅(a)+1)(v₅(b)+1)) * (v₅(a)+v₅(b)+1).
// v₅(r-x)+v₅(r+x) = 2 v₅(x) (0 if 5 ∤ x). χ(n) via lattice points x²+y² < 2r.

use rayon::prelude::*;

const R5: i64 = 9_765_625; // 5^10
const MAX_N: usize = 19_531_260; // 2*R5 + 10
const CHUNK: usize = 1 << 17;

fn main() {
    let mut chi = vec![0i16; MAX_N];
    fill_chi(&mut chi);

    let r = R5 as usize;
    let chi = &chi;
    let n_x = r - 1;
    let n_chunks = n_x.div_ceil(CHUNK);

    let partial: i64 = (0..n_chunks)
        .into_par_iter()
        .map(|ci| {
            let start = ci * CHUNK + 1;
            let end = (start + CHUNK).min(r);
            chunk_sum(start, end, r, chi)
        })
        .sum();

    let s_r5 = 6 * R5 + 24 * partial;
    println!("{}", 1024 * s_r5);
}

/// χ(n) = r₂(n)/4 for n < MAX_N, via first-octant lattice points.
fn fill_chi(chi: &mut [i16]) {
    let p = chi.as_mut_ptr();
    let max_n = MAX_N as u32;
    let mut x = 1u32;
    loop {
        let x2 = x * x;
        if x2 >= max_n {
            break;
        }
        // (±x, 0) and (0, ±x): r₂ += 4 → χ += 1
        unsafe {
            *p.add(x2 as usize) += 1;
        }
        // y = 1, 2, … with n = x²+y² < MAX_N. y < x: (x,y) and (y,x) → χ += 2;
        // y = x: (x,x) sign patterns → χ += 1.
        let mut n = x2 + 1;
        let mut inc = 3u32;
        let diag = x2 << 1;
        while n < max_n && n < diag {
            unsafe {
                *p.add(n as usize) += 2;
            }
            n += inc;
            inc += 2;
        }
        if n < max_n && n == diag {
            unsafe {
                *p.add(n as usize) += 1;
            }
        }
        x += 1;
    }
}

fn chunk_sum(start: usize, end: usize, r: usize, chi: &[i16]) -> i64 {
    let p = chi.as_ptr();
    let mut local = 0i64;
    for x in start..end {
        // r ≡ 1 (mod 4); x ≡ 2 (mod 4) ⇒ r±x ≡ 3 (mod 4) ⇒ χ = 0
        if x & 3 == 2 {
            continue;
        }
        let fa = unsafe { *p.add(r - x) };
        if fa == 0 {
            continue;
        }
        let fb = unsafe { *p.add(r + x) };
        if fb == 0 {
            continue;
        }
        let prod = fa as i64 * fb as i64;
        let w = if x % 5 != 0 {
            prod
        } else {
            let mut t = x;
            let mut v = 0i64;
            while t % 5 == 0 {
                t /= 5;
                v += 1;
            }
            // χ(a)=f(a)(v+1), χ(b)=f(b)(v+1), extra = 2v+1
            prod * (2 * v + 1) / ((v + 1) * (v + 1))
        };
        local += x as i64 * w;
    }
    local
}
