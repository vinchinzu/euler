// Project Euler 756 - Approximating a Sum
// E(Delta | phi(k), n, m) = sum_{j=1}^{n-m} phi(j) * C(n-j, m) / C(n, m)
//
// where C(n-j,m)/C(n,m) = prod_{t=0}^{j-1} (n-t-m)/(n-t)
//
// Compute directly to avoid catastrophic cancellation from S - (large sum).
// Use Kahan compensated summation for precision.

fn main() {
    let n: usize = 12_345_678;
    let m: usize = 12_345;

    // Sieve for Euler's totient
    let mut phi = vec![0u32; n + 1];
    for i in 0..=n {
        // SAFETY: i is in range [0..=n], phi has length n + 1
        unsafe {
            *phi.get_unchecked_mut(i) = i as u32;
        }
    }
    
    for i in 2..=n {
        if phi[i] == i as u32 {
            let p = i as u32;
            let factor = p - 1;
            
            let mut j = i;
            let stride = i;
            let stride4 = stride * 4;
            
            while j + stride4 <= n {
                // SAFETY: j, j+stride, j+2*stride, j+3*stride all <= n,
                // phi has length n + 1, so all accesses are in bounds.
                unsafe {
                    let v0 = *phi.get_unchecked(j);
                    *phi.get_unchecked_mut(j) = (v0 / p) * factor;
                    
                    let j1 = j + stride;
                    let v1 = *phi.get_unchecked(j1);
                    *phi.get_unchecked_mut(j1) = (v1 / p) * factor;
                    
                    let j2 = j1 + stride;
                    let v2 = *phi.get_unchecked(j2);
                    *phi.get_unchecked_mut(j2) = (v2 / p) * factor;
                    
                    let j3 = j2 + stride;
                    let v3 = *phi.get_unchecked(j3);
                    *phi.get_unchecked_mut(j3) = (v3 / p) * factor;
                }
                j += stride4;
            }
            
            while j <= n {
                // SAFETY: j <= n, phi has length n + 1
                unsafe {
                    let val = *phi.get_unchecked(j);
                    *phi.get_unchecked_mut(j) = (val / p) * factor;
                }
                j += stride;
            }
        }
    }

    // Compute E[Delta] = sum_{j=1}^{n-m} phi(j) * w_j
    // where w_j = prod_{t=0}^{j-1} (n-t-m)/(n-t)
    // w_0 = 1, w_j = w_{j-1} * (n - j + 1 - m) / (n - j + 1)
    let mut ans: f64 = 0.0;
    let mut comp: f64 = 0.0;
    let mut w: f64 = 1.0;

    let max_j = n - m;
    let m_f64 = m as f64;

    let mut j = 1;
    while j <= max_j {
        let idx = (n - j + 1) as f64;
        w *= (idx - m_f64) / idx;

        if w < 1e-18 {
            break;
        }

        // SAFETY: j is in range [1..=max_j] where max_j = n - m < n,
        // and phi has length n + 1, so phi[j] is always in bounds.
        let phi_j = unsafe { *phi.get_unchecked(j) };
        let term = phi_j as f64 * w;

        let y = term - comp;
        let t = ans + y;
        comp = (t - ans) - y;
        ans = t;
        
        j += 1;
    }

    println!("{:.6}", ans);
}
