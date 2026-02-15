// Project Euler 763 - Amoebas in a 3D Grid
// DP recurrence computing D(N) = number of distinct arrangements after N divisions.

fn main() {
    const M_VAL: usize = 9999;
    const MOD: u64 = 1_000_000_000;

    // Determine max_n: (n+1)*(n+2)/2 <= M_VAL
    let mut n_tmp = 0;
    while (n_tmp + 1) * (n_tmp + 2) / 2 <= M_VAL {
        n_tmp += 1;
    }
    let max_n = n_tmp - 1;
    let n_val = max_n + 3;

    let mut offset_arr = vec![0usize; n_val + 4];
    let mut lens_arr = vec![0usize; n_val + 4];
    for n in 0..n_val + 2 {
        offset_arr[n] = (n + 1) * (n + 2) / 2;
        let ln = if M_VAL >= offset_arr[n] { M_VAL - offset_arr[n] + 1 } else { 0 };
        lens_arr[n] = ln;
    }

    // Allocate u and v arrays
    let mut u_arr: Vec<Vec<u32>> = vec![Vec::new(); n_val + 2];
    let mut v_arr: Vec<Vec<u32>> = vec![Vec::new(); n_val + 2];
    for n in 1..n_val + 2 {
        let ln = lens_arr[n];
        if ln > 0 {
            let sz = n * ln;
            u_arr[n] = vec![0u32; sz];
            v_arr[n] = vec![0u32; sz];
        }
    }

    let mut f0 = vec![0u32; M_VAL + 1];
    let mut a2 = vec![0u32; M_VAL + 1];
    a2[0] = 1;

    let mut n_active: usize = 0;

    for m in 0..=M_VAL {
        while n_active + 1 < n_val + 1 && offset_arr[n_active + 1] <= m {
            n_active += 1;
        }

        for n in 1..=n_active {
            let off = offset_arr[n];
            let ln = lens_arr[n];
            let idx_cur = m as isize - off as isize;
            if idx_cur < 0 || idx_cur as usize >= ln { continue; }
            let idx_cur = idx_cur as usize;

            let mp1 = m as isize - n as isize - 2;
            let idx1 = mp1 - off as isize;

            let mp2 = m as isize - n as isize - 3;
            let idx2 = mp2 - offset_arr[n + 1] as isize;
            let lnp = lens_arr[n + 1];

            let mp3 = m as isize - n as isize - 1;
            let idx3 = mp3 - offset_arr[if n > 0 { n - 1 } else { 0 }] as isize;
            let lnm = if n > 1 { lens_arr[n - 1] } else { 0 };

            if n == 1 {
                let mut val_u: u64 = 0;
                if idx1 >= 0 && (idx1 as usize) < ln {
                    val_u += 2 * u_arr[1][idx1 as usize] as u64 + v_arr[1][idx1 as usize] as u64;
                }
                if idx2 >= 0 && lnp > 0 && (idx2 as usize) < lnp {
                    val_u += v_arr[2][idx2 as usize] as u64 + u_arr[2][lnp + idx2 as usize] as u64;
                }
                if mp3 >= 0 {
                    val_u += f0[mp3 as usize] as u64;
                }
                u_arr[1][idx_cur] = (val_u % MOD) as u32;

                let mut val_v: u64 = 0;
                if idx1 >= 0 && (idx1 as usize) < ln {
                    val_v += 2 * v_arr[1][idx1 as usize] as u64 + 2 * u_arr[1][idx1 as usize] as u64;
                }
                if idx2 >= 0 && lnp > 0 && (idx2 as usize) < lnp {
                    val_v += v_arr[2][lnp + idx2 as usize] as u64 + 2 * u_arr[2][idx2 as usize] as u64;
                }
                if mp3 >= 0 {
                    val_v += f0[mp3 as usize] as u64;
                }
                v_arr[1][idx_cur] = (val_v % MOD) as u32;
                continue;
            }

            let u_n1 = if idx1 >= 0 && (idx1 as usize) < ln { u_arr[n][idx1 as usize] } else { 0 };
            let v_n1 = if idx1 >= 0 && (idx1 as usize) < ln { v_arr[n][idx1 as usize] } else { 0 };
            let u_p1 = if idx2 >= 0 && lnp > 0 && (idx2 as usize) < lnp { u_arr[n + 1][idx2 as usize] } else { 0 };
            let v_p1 = if idx2 >= 0 && lnp > 0 && (idx2 as usize) < lnp { v_arr[n + 1][idx2 as usize] } else { 0 };

            if idx1 < 0 {
                // Only n-1 term
                for k in 1..n {
                    let base = (k - 1) * ln;
                    let base_m = (k - 1) * lnm;
                    if idx3 >= 0 && (idx3 as usize) < lnm {
                        u_arr[n][base + idx_cur] = u_arr[n - 1][base_m + idx3 as usize];
                        v_arr[n][base + idx_cur] = v_arr[n - 1][base_m + idx3 as usize];
                    }
                }
                let base_last = (n - 1) * ln;
                let base_m_last = (n - 2) * lnm;
                if idx3 >= 0 && (idx3 as usize) < lnm {
                    u_arr[n][base_last + idx_cur] = u_arr[n - 1][base_m_last + idx3 as usize];
                    v_arr[n][base_last + idx_cur] = v_arr[n - 1][base_m_last + idx3 as usize];
                }
                continue;
            }

            let idx1u = idx1 as usize;

            if idx2 >= 0 && lnp > 0 && (idx2 as usize) < lnp {
                let idx2u = idx2 as usize;
                // Full recurrence
                for k in 1..n {
                    let base = (k - 1) * ln;
                    let base_next = k * ln;
                    let base_p = (k - 1) * lnp;
                    let base_m = (k - 1) * lnm;

                    let mut uval: u64 = u_arr[n][base + idx1u] as u64
                        + v_p1 as u64
                        + u_arr[n + 1][base_p + idx2u] as u64
                        + if idx3 >= 0 && (idx3 as usize) < lnm { u_arr[n - 1][base_m + idx3 as usize] as u64 } else { 0 }
                        + v_n1 as u64
                        + u_arr[n][base_next + idx1u] as u64;
                    u_arr[n][base + idx_cur] = (uval % MOD) as u32;

                    let mut vval: u64 = v_arr[n][base + idx1u] as u64
                        + v_arr[n + 1][base_p + idx2u] as u64
                        + u_p1 as u64
                        + if idx3 >= 0 && (idx3 as usize) < lnm { v_arr[n - 1][base_m + idx3 as usize] as u64 } else { 0 }
                        + v_arr[n][base_next + idx1u] as u64
                        + u_n1 as u64;
                    v_arr[n][base + idx_cur] = (vval % MOD) as u32;
                }
                let base_last = (n - 1) * ln;
                let base_p_last = (n - 1) * lnp;
                let base_m_last = (n - 2) * lnm;
                let uval = 2u64 * u_arr[n][base_last + idx1u] as u64
                    + v_n1 as u64
                    + v_p1 as u64
                    + u_arr[n + 1][base_p_last + idx2u] as u64
                    + if idx3 >= 0 && (idx3 as usize) < lnm { u_arr[n - 1][base_m_last + idx3 as usize] as u64 } else { 0 };
                u_arr[n][base_last + idx_cur] = (uval % MOD) as u32;

                let vval = 2u64 * v_arr[n][base_last + idx1u] as u64
                    + 2 * u_n1 as u64
                    + v_arr[n + 1][base_p_last + idx2u] as u64
                    + 2 * u_p1 as u64
                    + if idx3 >= 0 && (idx3 as usize) < lnm { v_arr[n - 1][base_m_last + idx3 as usize] as u64 } else { 0 };
                v_arr[n][base_last + idx_cur] = (vval % MOD) as u32;
            } else {
                // No n+1 contribution
                for k in 1..n {
                    let base = (k - 1) * ln;
                    let base_next = k * ln;
                    let base_m = (k - 1) * lnm;

                    let uval = u_arr[n][base + idx1u] as u64
                        + if idx3 >= 0 && (idx3 as usize) < lnm { u_arr[n - 1][base_m + idx3 as usize] as u64 } else { 0 }
                        + v_n1 as u64
                        + u_arr[n][base_next + idx1u] as u64;
                    u_arr[n][base + idx_cur] = (uval % MOD) as u32;

                    let vval = v_arr[n][base + idx1u] as u64
                        + if idx3 >= 0 && (idx3 as usize) < lnm { v_arr[n - 1][base_m + idx3 as usize] as u64 } else { 0 }
                        + v_arr[n][base_next + idx1u] as u64
                        + u_n1 as u64;
                    v_arr[n][base + idx_cur] = (vval % MOD) as u32;
                }
                let base_last = (n - 1) * ln;
                let base_m_last = (n - 2) * lnm;
                let uval = 2u64 * u_arr[n][base_last + idx1u] as u64 + v_n1 as u64
                    + if idx3 >= 0 && (idx3 as usize) < lnm { u_arr[n - 1][base_m_last + idx3 as usize] as u64 } else { 0 };
                u_arr[n][base_last + idx_cur] = (uval % MOD) as u32;

                let vval = 2u64 * v_arr[n][base_last + idx1u] as u64 + 2 * u_n1 as u64
                    + if idx3 >= 0 && (idx3 as usize) < lnm { v_arr[n - 1][base_m_last + idx3 as usize] as u64 } else { 0 };
                v_arr[n][base_last + idx_cur] = (vval % MOD) as u32;
            }
        }

        // f0 and a2
        let mut val_f: u64 = 0;
        if m >= 1 { val_f += a2[m - 1] as u64; }
        if m >= 2 { val_f += 4 * f0[m - 2] as u64; }
        let mp = m as isize - 3;
        if mp >= offset_arr[1] as isize && lens_arr[1] > 0 {
            let id1 = (mp - offset_arr[1] as isize) as usize;
            if id1 < lens_arr[1] {
                val_f += 2 * u_arr[1][id1] as u64 + v_arr[1][id1] as u64;
            }
        }
        f0[m] = (val_f % MOD) as u32;

        if m >= 1 {
            let mut val_a: u64 = 3 * a2[m - 1] as u64;
            if m >= 2 { val_a += 3 * f0[m - 2] as u64; }
            a2[m] = (val_a % MOD) as u32;
        }
    }

    println!("{}", a2[9999]);
}
