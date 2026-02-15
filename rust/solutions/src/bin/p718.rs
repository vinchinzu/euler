// Project Euler 718 - Unreachable Numbers
//
// Find sum of positive integers not expressible as A*a + B*b + C*c (a,b,c > 0)
// where A=17^6, B=19^6, C=23^6. Uses two-queue BFS over residues mod A.

const MOD: i64 = 1_000_000_007;

fn ncr2(n: i64) -> i64 {
    if n < 2 { 0 } else { n * (n - 1) / 2 }
}

fn main() {
    let mut a: i64 = 1;
    let mut b: i64 = 1;
    let mut c: i64 = 1;
    for _ in 0..6 {
        a *= 17;
        b *= 19;
        c *= 23;
    }

    let mut visited = vec![false; a as usize];

    let cap = a as usize + 1;
    let mut q1 = Vec::with_capacity(cap);
    let mut q2 = Vec::with_capacity(cap);
    let mut q1_head: usize = 0;
    let mut q2_head: usize = 0;

    let mut val = a + b + c;
    let mut ans: i64 = 0;

    loop {
        visited[val as usize % a as usize] = true;
        let q = val / a;
        let r = val % a;
        let term = ((ncr2(q) % MOD) as i128 * (a % MOD) as i128 % MOD as i128
            + (r % MOD) as i128 * (q % MOD) as i128 % MOD as i128)
            % MOD as i128;
        ans = (ans + term as i64) % MOD;

        let next_val1 = val + b;
        let next_val2 = val + c;

        if !visited[next_val1 as usize % a as usize] {
            q1.push(next_val1);
        }
        if !visited[next_val2 as usize % a as usize] {
            q2.push(next_val2);
        }

        if q1_head >= q1.len() && q2_head >= q2.len() {
            break;
        }

        let v1 = if q1_head < q1.len() { q1[q1_head] } else { -1 };
        let v2 = if q2_head < q2.len() { q2[q2_head] } else { -1 };

        if v1 < 0 {
            val = v2;
        } else if v2 < 0 {
            val = v1;
        } else {
            val = v1.min(v2);
        }

        if q1_head < q1.len() && val == q1[q1_head] {
            q1_head += 1;
        }
        if q2_head < q2.len() && val == q2[q2_head] {
            q2_head += 1;
        }
    }

    println!("{}", ans);
}
