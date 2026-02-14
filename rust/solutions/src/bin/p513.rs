// Project Euler 513 - Triangles with Integer Median
// Mobius function summation over coprime lattice point counts.

const NN: usize = 100_000;

fn compute_mobius(limit: usize) -> Vec<i8> {
    let mut mobius = vec![0i8; limit + 1];
    let mut spf = vec![0u32; limit + 1];
    for i in 0..=limit { spf[i] = i as u32; }
    mobius[1] = 1;

    for i in 2..=limit {
        if spf[i] == i as u32 {
            mobius[i] = -1;
            let mut j = (i as u64) * (i as u64);
            while j <= limit as u64 {
                if spf[j as usize] == j as u32 { spf[j as usize] = i as u32; }
                j += i as u64;
            }
        } else {
            let p = spf[i] as usize;
            let q = i / p;
            if q % p == 0 {
                mobius[i] = 0;
            } else {
                mobius[i] = -mobius[q];
            }
        }
    }
    mobius
}

fn count_pq_case1(k: i64, l: i64, n: i64) -> i64 {
    let mut count = 0i64;
    for p in 1.. {
        let mut q_min = l;
        if p >= q_min { q_min = p + 1; }

        let q_bc = ((3 * k + l) as i128 * p as i128 + (k + l) as i128 - 1) / (k + l) as i128;
        let q_bc = q_bc as i64;
        if q_bc > q_min { q_min = q_bc; }

        let q_max_orig = (l as i128 * p as i128 / k as i128) as i64;
        let q_cn = ((n as i128 + k as i128 * p as i128) / l as i128) as i64;
        let q_max = q_max_orig.min(q_cn);

        if q_min > q_max {
            if q_cn < q_bc { break; }
            continue;
        }
        count += q_max - q_min + 1;
    }
    count
}

fn count_pq_case2(k: i64, l: i64, n: i64) -> i64 {
    let l_half = (l + 1) / 2;
    let n_half = n / 2;
    let mut count = 0i64;

    for p in 1.. {
        let mut q_min = l_half;
        if p >= q_min { q_min = p + 1; }

        let q_bc = ((3 * k + l) as i128 * p as i128 + (k + l) as i128 - 1) / (k + l) as i128;
        let q_bc = q_bc as i64;
        if q_bc > q_min { q_min = q_bc; }

        let q_max_orig = (l as i128 * p as i128 / k as i128) as i64;
        let q_cn = ((n_half as i128 + k as i128 * p as i128) / l as i128) as i64;
        let q_max = q_max_orig.min(q_cn);

        if q_min > q_max {
            if q_cn < q_bc { break; }
            continue;
        }
        count += q_max - q_min + 1;
    }
    count
}

fn count_pq_case3(k: i64, l: i64, n: i64) -> i64 {
    let mut count = 0i64;
    for v in 1.. {
        let mut u_min = v + 1;
        if l - v > u_min { u_min = l - v; }

        let u_from_y = ((k + l) as i128 * v as i128 + (l - k) as i128 - 1) / (l - k) as i128;
        let u_from_y = u_from_y as i64;
        if u_from_y > u_min { u_min = u_from_y; }

        let u_max_orig = ((2 * k + l) as i128 * v as i128 / k as i128) as i64;
        let num = n as i128 - (k + l) as i128 * v as i128;
        if num < 0 { break; }
        let u_cn = (num / (l - k) as i128) as i64;
        let u_max = u_max_orig.min(u_cn);

        if u_min <= u_max {
            count += u_max - u_min + 1;
        }
    }
    count
}

fn count_kl_case1(p: i64, q: i64, n: i64, big_l: i64) -> i64 {
    let l_base = q.max(big_l);
    let mut count = 0i64;

    for k in 1.. {
        let mut l_min = l_base + 1;
        if k + 1 > l_min { l_min = k + 1; }

        let l_from_y = ((q as i128 * k as i128 + p as i128 - 1) / p as i128) as i64;
        if l_from_y > l_min { l_min = l_from_y; }

        if 3 * p > q {
            let l_from_bc = (((3 * p - q) as i128 * k as i128 + (q - p) as i128 - 1) / (q - p) as i128) as i64;
            if l_from_bc > l_min { l_min = l_from_bc; }
        }

        let l_max = ((n as i128 + p as i128 * k as i128) / q as i128) as i64;
        if l_min > l_max { break; }
        count += l_max - l_min + 1;
    }
    count
}

fn count_kl_case2(p: i64, q: i64, n: i64, big_l: i64) -> i64 {
    let q_half = q / 2;
    let l_half = big_l / 2;
    let l_base = q_half.max(l_half);
    let n_half = n / 2;
    let mut count = 0i64;

    for k in 1.. {
        let mut l_min = l_base + 1;
        if k + 1 > l_min { l_min = k + 1; }

        let l_from_y = ((q as i128 * k as i128 + p as i128 - 1) / p as i128) as i64;
        if l_from_y > l_min { l_min = l_from_y; }

        if 3 * p > q {
            let l_from_bc = (((3 * p - q) as i128 * k as i128 + (q - p) as i128 - 1) / (q - p) as i128) as i64;
            if l_from_bc > l_min { l_min = l_from_bc; }
        }

        let l_max = ((n_half as i128 + p as i128 * k as i128) / q as i128) as i64;
        if l_min > l_max { break; }
        count += l_max - l_min + 1;
    }
    count
}

fn count_kl_case3(p: i64, q: i64, n: i64, big_l: i64) -> i64 {
    let mut count = count_kl_case2(p, q, n, big_l);

    let b_min_q = (q + 1) / 2;
    let b_min_l = (big_l + 1) / 2;
    let b_base = b_min_q.max(b_min_l);

    let n_adj = n - q + p;
    if n_adj < 0 { return count; }

    for a in 0.. {
        let mut b_min = b_base;
        if a + 1 > b_min { b_min = a + 1; }

        let b_from_y_num = 2 * q as i128 * a as i128 + q as i128 - p as i128;
        if b_from_y_num > 0 {
            let b_from_y = ((b_from_y_num + 2 * p as i128 - 1) / (2 * p as i128)) as i64;
            if b_from_y > b_min { b_min = b_from_y; }
        }

        if 3 * p > q {
            let b_from_bc_num = (3 * p - q) as i128 * a as i128 + (2 * p - q) as i128;
            if b_from_bc_num > 0 {
                let b_from_bc = ((b_from_bc_num + (q - p) as i128 - 1) / (q - p) as i128) as i64;
                if b_from_bc > b_min { b_min = b_from_bc; }
            }
        }

        let b_max_num = 2 * p as i128 * a as i128 + n_adj as i128;
        if b_max_num < 0 { break; }
        let b_max = (b_max_num / (2 * q as i128)) as i64;

        if b_min <= b_max {
            count += b_max - b_min + 1;
        }
        if b_max < b_base { break; }
    }
    count
}

fn f(n: i64, check_parity: bool) -> i64 {
    let big_l = (1.5 * n as f64).sqrt() as i64;
    let mut result = 0i64;

    for l in 1..=big_l {
        for k in 1..l {
            if !check_parity || (k % 2 == 0 && l % 2 == 0) {
                result += count_pq_case1(k, l, n);
            } else if k % 2 == 0 || l % 2 == 0 {
                result += count_pq_case2(k, l, n);
            } else {
                result += count_pq_case3(k, l, n);
            }
        }
    }

    for q in 1..=big_l {
        for p in 1..q {
            if !check_parity || (p % 2 == 0 && q % 2 == 0) {
                result += count_kl_case1(p, q, n, big_l);
            } else if p % 2 == 0 || q % 2 == 0 {
                result += count_kl_case2(p, q, n, big_l);
            } else {
                result += count_kl_case3(p, q, n, big_l);
            }
        }
    }

    result
}

fn main() {
    let mobius = compute_mobius(NN);

    let mut ans: i64 = 0;
    for g in 1..=NN {
        if mobius[g] != 0 {
            ans += mobius[g] as i64 * f((NN / g) as i64, g % 2 == 1);
        }
    }

    println!("{}", ans);
}
