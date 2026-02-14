// Project Euler 194: Coloured Configurations
// C(100,25) * C*(C-1) * qa^25 * qb^75 mod 10^8
// where qa, qb = Q(1984) for the two unit graphs.

use euler_utils::mod_pow;

const MOD: u64 = 100_000_000;
const NV: usize = 7;

struct Edge {
    u: usize,
    v: usize,
}

fn chromatic_poly_at(edges: &[Edge], c: usize) -> u64 {
    if c == 0 {
        return 0;
    }
    if c == 1 {
        return if edges.is_empty() { 1 } else { 0 };
    }

    let mut adj_mask = [0u32; NV];
    for e in edges {
        adj_mask[e.u] |= 1 << e.v;
        adj_mask[e.v] |= 1 << e.u;
    }

    let mut coloring = [usize::MAX; NV];
    let mut count = 0u64;
    backtrack(0, c, &adj_mask, &mut coloring, &mut count);
    count
}

fn backtrack(v: usize, c: usize, adj_mask: &[u32; NV], coloring: &mut [usize; NV], count: &mut u64) {
    if v == NV {
        *count += 1;
        return;
    }
    'col: for col in 0..c {
        let mut mask = adj_mask[v];
        while mask != 0 {
            let u = mask.trailing_zeros() as usize;
            mask &= mask - 1;
            if coloring[u] == col {
                continue 'col;
            }
        }
        coloring[v] = col;
        backtrack(v + 1, c, adj_mask, coloring, count);
        coloring[v] = usize::MAX;
    }
}

// Lagrange interpolation using i128 for exact rational arithmetic
fn lagrange_mod(x_vals: &[i128], q_vals: &[i128], c_val: i128) -> u64 {
    let n = x_vals.len();
    let mut result_num: i128 = 0;
    let mut result_den: i128 = 1;

    for i in 0..n {
        let mut num = q_vals[i];
        let mut den: i128 = 1;
        for j in 0..n {
            if i == j { continue; }
            num *= c_val - x_vals[j];
            den *= x_vals[i] - x_vals[j];
        }
        result_num = result_num * den + num * result_den;
        result_den *= den;

        // Simplify
        let g = gcd128(result_num.unsigned_abs(), result_den.unsigned_abs());
        if g > 1 {
            result_num /= g as i128;
            result_den /= g as i128;
        }
    }

    if result_den < 0 {
        result_num = -result_num;
        result_den = -result_den;
    }

    let val = result_num / result_den;
    ((val % MOD as i128 + MOD as i128) % MOD as i128) as u64
}

fn gcd128(a: u128, b: u128) -> u128 {
    let (mut a, mut b) = (a, b);
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

fn main() {
    let a_count = 25u64;
    let b_count = 75u64;
    let c: u64 = 1984;

    let a_edges = [
        Edge{u:0,v:1}, Edge{u:0,v:2}, Edge{u:0,v:5}, Edge{u:1,v:2}, Edge{u:1,v:6},
        Edge{u:2,v:3}, Edge{u:3,v:4}, Edge{u:4,v:5}, Edge{u:4,v:6}, Edge{u:5,v:6},
    ];
    let b_edges = [
        Edge{u:0,v:1}, Edge{u:0,v:2}, Edge{u:0,v:5}, Edge{u:1,v:2}, Edge{u:1,v:6},
        Edge{u:2,v:3}, Edge{u:3,v:4}, Edge{u:4,v:5}, Edge{u:4,v:6},
    ];

    let mut x_vals = [0i128; 6];
    let mut qa_vals = [0i128; 6];
    let mut qb_vals = [0i128; 6];
    for i in 0..6 {
        let cv = i + 2;
        x_vals[i] = cv as i128;
        let pa = chromatic_poly_at(&a_edges, cv);
        let pb = chromatic_poly_at(&b_edges, cv);
        qa_vals[i] = (pa / (cv as u64 * (cv as u64 - 1))) as i128;
        qb_vals[i] = (pb / (cv as u64 * (cv as u64 - 1))) as i128;
    }

    let qa = lagrange_mod(&x_vals, &qa_vals, c as i128);
    let qb = lagrange_mod(&x_vals, &qb_vals, c as i128);

    // Compute C(100, 25) mod MOD via prime factorization
    let mut fact = [0i32; 101];
    for i in 2..=100usize {
        let mut x = i;
        let mut p = 2;
        while p <= x {
            while x % p == 0 {
                fact[p] += 1;
                x /= p;
            }
            p += 1;
        }
    }
    for i in 2..=25usize {
        let mut x = i;
        let mut p = 2;
        while p <= x {
            while x % p == 0 {
                fact[p] -= 1;
                x /= p;
            }
            p += 1;
        }
    }
    for i in 2..=75usize {
        let mut x = i;
        let mut p = 2;
        while p <= x {
            while x % p == 0 {
                fact[p] -= 1;
                x /= p;
            }
            p += 1;
        }
    }

    let mut comb: u64 = 1;
    for p in 2..=100 {
        if fact[p] > 0 {
            comb = comb * mod_pow(p as u64, fact[p] as u64, MOD) % MOD;
        }
    }

    let mut ans = c % MOD * ((c - 1) % MOD) % MOD;
    ans = ans * comb % MOD;
    ans = ans * mod_pow(qa, a_count, MOD) % MOD;
    ans = ans * mod_pow(qb, b_count, MOD) % MOD;

    println!("{}", ans);
}
