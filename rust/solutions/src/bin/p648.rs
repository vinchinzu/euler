// Project Euler 648 - Skipping Squares
// Power series expansion with DP

const NN: usize = 1000;
const MOD: i64 = 1_000_000_000;

fn main() {
    let max_s = (NN / 2) * (NN / 2);

    let mut is_sq = vec![false; max_s + 1];
    let mut sqrt_of = vec![0i32; max_s + 1];
    let mut r = 0i32;
    while (r * r) as usize <= max_s {
        let sq = (r * r) as usize;
        is_sq[sq] = true;
        sqrt_of[sq] = r;
        r += 1;
    }

    let mut jump1 = vec![0i64; NN + 1];
    let mut jump2 = vec![0i64; NN + 1];
    let mut f = vec![0i64; NN + 1];

    for s in (0..=max_s).rev() {
        if s > 0 && is_sq[s] {
            f.fill(0);
            f[0] = (sqrt_of[s] - 1) as i64;
        } else {
            // SAFETY: all indices provably in bounds; arrays have length NN+1
            unsafe {
                let f_ptr = f.as_mut_ptr();
                let j1_ptr = jump1.as_ptr();
                let j2_ptr = jump2.as_ptr();
                
                *f_ptr = *j2_ptr;
                
                let mut k = 1;
                while k + 3 <= NN {
                    let v0 = *j2_ptr.add(k) + *j1_ptr.add(k - 1) - *j2_ptr.add(k - 1);
                    let v1 = *j2_ptr.add(k + 1) + *j1_ptr.add(k) - *j2_ptr.add(k);
                    let v2 = *j2_ptr.add(k + 2) + *j1_ptr.add(k + 1) - *j2_ptr.add(k + 1);
                    let v3 = *j2_ptr.add(k + 3) + *j1_ptr.add(k + 2) - *j2_ptr.add(k + 2);
                    
                    *f_ptr.add(k) = if v0 >= MOD { v0 - MOD } else if v0 < 0 { v0 + MOD } else { v0 };
                    *f_ptr.add(k + 1) = if v1 >= MOD { v1 - MOD } else if v1 < 0 { v1 + MOD } else { v1 };
                    *f_ptr.add(k + 2) = if v2 >= MOD { v2 - MOD } else if v2 < 0 { v2 + MOD } else { v2 };
                    *f_ptr.add(k + 3) = if v3 >= MOD { v3 - MOD } else if v3 < 0 { v3 + MOD } else { v3 };
                    k += 4;
                }
                
                while k <= NN {
                    let v = *j2_ptr.add(k) + *j1_ptr.add(k - 1) - *j2_ptr.add(k - 1);
                    *f_ptr.add(k) = if v >= MOD { v - MOD } else if v < 0 { v + MOD } else { v };
                    k += 1;
                }
            }
        }

        std::mem::swap(&mut jump2, &mut jump1);
        std::mem::swap(&mut jump1, &mut f);
    }

    let mut ans = 0i64;
    for &v in &jump1 {
        ans += v;
        if ans >= MOD {
            ans -= MOD;
        }
    }
    println!("{}", ans);
}
