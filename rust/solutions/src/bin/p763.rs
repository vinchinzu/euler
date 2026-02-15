// Project Euler 763 - Amoebas in a 3D Grid
// Faithful port of the C solution.
// DP recurrence computing D(N) = number of distinct arrangements after N divisions.

fn main() {
    const M_VAL: usize = 9999;
    const MOD: u64 = 1_000_000_000;

    // Determine max_n: (n+1)*(n+2)/2 <= M_VAL
    let mut n_tmp: usize = 0;
    while (n_tmp + 1) * (n_tmp + 2) / 2 <= M_VAL {
        n_tmp += 1;
    }
    let max_n = n_tmp - 1;
    let n_val = max_n + 3;

    let mut offset_arr = vec![0i32; n_val + 4];
    let mut lens_arr = vec![0i32; n_val + 4];
    for n in 0..n_val + 2 {
        offset_arr[n] = ((n + 1) * (n + 2) / 2) as i32;
        let ln = M_VAL as i32 - offset_arr[n] + 1;
        lens_arr[n] = if ln > 0 { ln } else { 0 };
    }

    // Allocate u and v arrays: u[n] has n * lens[n] elements
    let mut u_arr: Vec<Vec<u32>> = vec![Vec::new(); n_val + 2];
    let mut v_arr: Vec<Vec<u32>> = vec![Vec::new(); n_val + 2];
    for n in 1..n_val + 2 {
        let ln = lens_arr[n];
        if ln > 0 {
            let sz = n as usize * ln as usize;
            u_arr[n] = vec![0u32; sz];
            v_arr[n] = vec![0u32; sz];
        }
    }

    let mut f0 = vec![0u32; M_VAL + 1];
    let mut a2 = vec![0u32; M_VAL + 1];
    a2[0] = 1;

    let mut n_active: i32 = 0;

    for m in 0..=M_VAL as i32 {
        while n_active + 1 < n_val as i32 + 1 && offset_arr[(n_active + 1) as usize] <= m {
            n_active += 1;
        }

        for n in 1..=n_active {
            let nu = n as usize;
            let off = offset_arr[nu];
            let ln = lens_arr[nu];
            let idx_cur = m - off;
            // idx_cur should always be in [0, ln) thanks to n_active tracking

            let mp1 = m - n - 2;
            let idx1 = mp1 - off;

            let mp2 = m - n - 3;
            let idx2 = mp2 - offset_arr[nu + 1];
            let lnp = lens_arr[nu + 1];

            let mp3 = m - n - 1;
            let idx3 = mp3 - offset_arr[if n > 0 { nu - 1 } else { 0 }];
            let lnm = if n > 1 { lens_arr[nu - 1] } else { 0 };

            if n == 1 {
                let mut val_u: u64 = 0;
                if idx1 >= 0 {
                    val_u += 2 * u_arr[1][idx1 as usize] as u64 + v_arr[1][idx1 as usize] as u64;
                }
                if idx2 >= 0 && lnp > 0 {
                    val_u += v_arr[2][idx2 as usize] as u64 + u_arr[2][(lnp + idx2) as usize] as u64;
                }
                if mp3 >= 0 {
                    val_u += f0[mp3 as usize] as u64;
                }
                u_arr[1][idx_cur as usize] = (val_u % MOD) as u32;

                let mut val_v: u64 = 0;
                if idx1 >= 0 {
                    val_v += 2 * v_arr[1][idx1 as usize] as u64 + 2 * u_arr[1][idx1 as usize] as u64;
                }
                if idx2 >= 0 && lnp > 0 {
                    val_v += v_arr[2][(lnp + idx2) as usize] as u64 + 2 * u_arr[2][idx2 as usize] as u64;
                }
                if mp3 >= 0 {
                    val_v += f0[mp3 as usize] as u64;
                }
                v_arr[1][idx_cur as usize] = (val_v % MOD) as u32;
                continue;
            }

            let u_n1 = if idx1 >= 0 { u_arr[nu][idx1 as usize] } else { 0 };
            let v_n1 = if idx1 >= 0 { v_arr[nu][idx1 as usize] } else { 0 };
            let u_p1 = if idx2 >= 0 && lnp > 0 { u_arr[nu + 1][idx2 as usize] } else { 0 };
            let v_p1 = if idx2 >= 0 && lnp > 0 { v_arr[nu + 1][idx2 as usize] } else { 0 };

            let mut base: i32 = 0;
            let mut base_next: i32 = ln;
            let mut base_p: i32 = lnp;
            let mut base_m: i32 = 0;

            if idx1 < 0 {
                // Only n-1 term survives
                for _k in 1..n {
                    u_arr[nu][(base + idx_cur) as usize] = u_arr[nu - 1][(base_m + idx3) as usize];
                    v_arr[nu][(base + idx_cur) as usize] = v_arr[nu - 1][(base_m + idx3) as usize];
                    base = base_next;
                    base_next += ln;
                    base_m += lnm;
                }
                // k = n
                u_arr[nu][((n - 1) * ln + idx_cur) as usize] = u_arr[nu - 1][((n - 2) * lnm + idx3) as usize];
                v_arr[nu][((n - 1) * ln + idx_cur) as usize] = v_arr[nu - 1][((n - 2) * lnm + idx3) as usize];
                continue;
            }

            if idx2 >= 0 && lnp > 0 {
                // Full recurrence
                for _k in 1..n {
                    let uval: u64 = u_arr[nu][(base + idx1) as usize] as u64
                        + v_p1 as u64
                        + u_arr[nu + 1][(base_p + idx2) as usize] as u64
                        + u_arr[nu - 1][(base_m + idx3) as usize] as u64
                        + v_n1 as u64
                        + u_arr[nu][(base_next + idx1) as usize] as u64;
                    u_arr[nu][(base + idx_cur) as usize] = (uval % MOD) as u32;

                    let vval: u64 = v_arr[nu][(base + idx1) as usize] as u64
                        + v_arr[nu + 1][(base_p + idx2) as usize] as u64
                        + u_p1 as u64
                        + v_arr[nu - 1][(base_m + idx3) as usize] as u64
                        + v_arr[nu][(base_next + idx1) as usize] as u64
                        + u_n1 as u64;
                    v_arr[nu][(base + idx_cur) as usize] = (vval % MOD) as u32;

                    base = base_next;
                    base_next += ln;
                    base_p += lnp;
                    base_m += lnm;
                }
                let base_last = (n - 1) * ln;
                let uval: u64 = 2 * u_arr[nu][(base_last + idx1) as usize] as u64
                    + v_n1 as u64
                    + v_p1 as u64
                    + u_arr[nu + 1][(base_p + idx2) as usize] as u64
                    + u_arr[nu - 1][((n - 2) * lnm + idx3) as usize] as u64;
                u_arr[nu][(base_last + idx_cur) as usize] = (uval % MOD) as u32;

                let vval: u64 = 2 * v_arr[nu][(base_last + idx1) as usize] as u64
                    + 2 * u_n1 as u64
                    + v_arr[nu + 1][(base_p + idx2) as usize] as u64
                    + 2 * u_p1 as u64
                    + v_arr[nu - 1][((n - 2) * lnm + idx3) as usize] as u64;
                v_arr[nu][(base_last + idx_cur) as usize] = (vval % MOD) as u32;
            } else {
                // No n+1 contribution
                for _k in 1..n {
                    let uval: u64 = u_arr[nu][(base + idx1) as usize] as u64
                        + u_arr[nu - 1][(base_m + idx3) as usize] as u64
                        + v_n1 as u64
                        + u_arr[nu][(base_next + idx1) as usize] as u64;
                    u_arr[nu][(base + idx_cur) as usize] = (uval % MOD) as u32;

                    let vval: u64 = v_arr[nu][(base + idx1) as usize] as u64
                        + v_arr[nu - 1][(base_m + idx3) as usize] as u64
                        + v_arr[nu][(base_next + idx1) as usize] as u64
                        + u_n1 as u64;
                    v_arr[nu][(base + idx_cur) as usize] = (vval % MOD) as u32;

                    base = base_next;
                    base_next += ln;
                    base_m += lnm;
                }
                let base_last = (n - 1) * ln;
                let uval: u64 = 2 * u_arr[nu][(base_last + idx1) as usize] as u64 + v_n1 as u64
                    + u_arr[nu - 1][((n - 2) * lnm + idx3) as usize] as u64;
                u_arr[nu][(base_last + idx_cur) as usize] = (uval % MOD) as u32;

                let vval: u64 = 2 * v_arr[nu][(base_last + idx1) as usize] as u64 + 2 * u_n1 as u64
                    + v_arr[nu - 1][((n - 2) * lnm + idx3) as usize] as u64;
                v_arr[nu][(base_last + idx_cur) as usize] = (vval % MOD) as u32;
            }
        }

        // f0 and a2
        let mut val_f: u64 = 0;
        if m >= 1 { val_f += a2[(m - 1) as usize] as u64; }
        if m >= 2 { val_f += 4 * f0[(m - 2) as usize] as u64; }
        let mp = m - 3;
        if mp >= offset_arr[1] && lens_arr[1] > 0 {
            let id1 = (mp - offset_arr[1]) as usize;
            val_f += 2 * u_arr[1][id1] as u64 + v_arr[1][id1] as u64;
        }
        f0[m as usize] = (val_f % MOD) as u32;

        if m >= 1 {
            let mut val_a: u64 = 3 * a2[(m - 1) as usize] as u64;
            if m >= 2 { val_a += 3 * f0[(m - 2) as usize] as u64; }
            a2[m as usize] = (val_a % MOD) as u32;
        }
    }

    println!("{}", a2[9999]);
}
