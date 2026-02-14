// Project Euler 572 - Idempotent Matrices
//
// Count 3x3 idempotent matrices (A^2=A) with integer elements |x| <= N=200.
// Uses rank decomposition: rank 0 (1), rank 1 + rank 2 (paired), rank 3 (1).

const N: i32 = 200;
const MAXPROD: i32 = N * N;
const PROD_OFFSET: usize = MAXPROD as usize;
const PROD_SIZE: usize = (2 * MAXPROD + 1) as usize;

fn my_gcd(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

fn main() {
    // Precompute GCD table
    let mut gcd_table = vec![vec![0i32; (N + 1) as usize]; (N + 1) as usize];
    for i in 0..=(N as usize) {
        for j in 0..=(N as usize) {
            gcd_table[i][j] = my_gcd(i as i32, j as i32);
        }
    }

    // Build product-to-pairs mapping
    let mut prod_count = vec![0u32; PROD_SIZE];
    for a in -N..=N {
        for b in -N..=N {
            prod_count[(a * b + MAXPROD) as usize] += 1;
        }
    }

    let mut prod_start = vec![0u32; PROD_SIZE];
    for i in 1..PROD_SIZE {
        prod_start[i] = prod_start[i - 1] + prod_count[i - 1];
    }

    let total = prod_start[PROD_SIZE - 1] + prod_count[PROD_SIZE - 1];
    let mut pair_r = vec![0i16; total as usize];
    let mut pair_c = vec![0i16; total as usize];
    let mut wpos = prod_start.clone();

    for a in -N..=N {
        for b in -N..=N {
            let idx = (a * b + MAXPROD) as usize;
            let w = wpos[idx] as usize;
            pair_r[w] = a as i16;
            pair_c[w] = b as i16;
            wpos[idx] += 1;
        }
    }

    let mut ans: i64 = 0;

    for a_val in -N..=(N + 1) {
        for e_val in -N..=(N + 1) {
            let i_val = 1 - a_val - e_val;
            if i_val < -N || i_val > N + 1 {
                continue;
            }

            let can_rank1 = a_val <= N && e_val <= N && i_val <= N;
            let can_rank2 = a_val > -N && e_val > -N && i_val > -N;
            if !can_rank1 && !can_rank2 {
                continue;
            }

            let inc: i64 = can_rank1 as i64 + can_rank2 as i64;

            let pa = (a_val + MAXPROD) as usize;
            let pe = (e_val + MAXPROD) as usize;
            let pi_idx = (i_val + MAXPROD) as usize;

            let na = prod_count[pa];
            let ne = prod_count[pe];
            let ni = prod_count[pi_idx];

            if na == 0 || ne == 0 || ni == 0 {
                continue;
            }

            let sa = prod_start[pa] as usize;
            let se = prod_start[pe] as usize;
            let si = prod_start[pi_idx] as usize;

            for j1 in sa..(sa + na as usize) {
                let r = pair_r[j1] as i32;
                let x = pair_c[j1] as i32;

                for j2 in se..(se + ne as usize) {
                    let s = pair_r[j2] as i32;
                    let y = pair_c[j2] as i32;

                    if (r * y).abs() > N || (s * x).abs() > N {
                        continue;
                    }

                    for j3 in si..(si + ni as usize) {
                        let t = pair_r[j3] as i32;
                        let z = pair_c[j3] as i32;

                        if (r * z).abs() > N
                            || (s * z).abs() > N
                            || (t * x).abs() > N
                            || (t * y).abs() > N
                        {
                            continue;
                        }

                        if gcd_table[gcd_table[r.unsigned_abs() as usize][s.unsigned_abs() as usize] as usize]
                            [t.unsigned_abs() as usize]
                            != 1
                        {
                            continue;
                        }

                        ans += inc;
                    }
                }
            }
        }
    }

    ans = ans / 2 + 2;
    println!("{}", ans);
}
