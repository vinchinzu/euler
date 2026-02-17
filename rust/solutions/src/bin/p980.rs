// Problem 980: The Quaternion Group I
// Ported from python/980.py.

fn build_mul_table_q8() -> [[usize; 8]; 8] {
    let mut base = [[0usize; 4]; 4];
    let mut sgn = [[1i32; 4]; 4];

    for a in 0..4 {
        for b in 0..4 {
            if a == 0 {
                base[a][b] = b;
                sgn[a][b] = 1;
            } else if b == 0 {
                base[a][b] = a;
                sgn[a][b] = 1;
            } else if a == b {
                base[a][b] = 0;
                sgn[a][b] = -1;
            } else {
                match (a, b) {
                    (1, 2) => {
                        base[a][b] = 3;
                        sgn[a][b] = 1;
                    }
                    (2, 3) => {
                        base[a][b] = 1;
                        sgn[a][b] = 1;
                    }
                    (3, 1) => {
                        base[a][b] = 2;
                        sgn[a][b] = 1;
                    }
                    (2, 1) => {
                        base[a][b] = 3;
                        sgn[a][b] = -1;
                    }
                    (3, 2) => {
                        base[a][b] = 1;
                        sgn[a][b] = -1;
                    }
                    (1, 3) => {
                        base[a][b] = 2;
                        sgn[a][b] = -1;
                    }
                    _ => unreachable!("unexpected basis pair"),
                }
            }
        }
    }

    let mut mul = [[0usize; 8]; 8];
    for a_enc in 0..8 {
        let sa = if a_enc < 4 { 1 } else { -1 };
        let a = a_enc & 3;
        for b_enc in 0..8 {
            let sb = if b_enc < 4 { 1 } else { -1 };
            let b = b_enc & 3;
            let s = sa * sb * sgn[a][b];
            let c = base[a][b];
            mul[a_enc][b_enc] = if s == 1 { c } else { c ^ 4 };
        }
    }
    mul
}

fn f(n: usize) -> u128 {
    const MOD: u64 = 888_888_883;
    const MULT: u64 = 8_888;
    const GEN: [usize; 3] = [1, 2, 7]; // x->i, y->j, z->-k

    let mul = build_mul_table_q8();

    let mut r = [0usize; 8 * 3];
    for v in 0..8 {
        for b in 0..3 {
            r[v * 3 + b] = mul[v][GEN[b]];
        }
    }

    let mut inv = [0usize; 8];
    for e in 0..8 {
        for cand in 0..8 {
            if mul[e][cand] == 0 && mul[cand][e] == 0 {
                inv[e] = cand;
                break;
            }
        }
    }

    let mut a: u64 = 88_888_888;
    let mut counts = [0u64; 8];

    for _ in 0..n {
        let mut v = 0usize;
        for _ in 0..50 {
            let b = (a % 3) as usize;
            v = r[v * 3 + b];
            a = (a * MULT) % MOD;
        }
        counts[v] += 1;
    }

    let mut total: u128 = 0;
    for e in 0..8 {
        total += counts[e] as u128 * counts[inv[e]] as u128;
    }
    total
}

fn main() {
    println!("{}", f(1_000_000));
}
