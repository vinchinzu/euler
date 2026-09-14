// Project Euler 841 - Regular Star Polygons
// Sum of A(F_{n+1}, F_{n-1}) for n=3..34
//
// A(p,q) = p * tan(q*pi/p) + 2 * sum_{k=1}^{q-1} (-1)^{q-k} * p * tan(k*pi/p)
//
// Direct computation suffers from catastrophic cancellation between p*tan(q*pi/p)
// and the alternating sum. Instead we use Abel summation to rewrite as:
//
// A(p,q) = p * sum_{j=0}^{q-1} s_j * D_j
// where D_j = sin(pi/p) / (cos(j*pi/p) * cos((j+1)*pi/p))
// and s_j = (-1)^j if q is odd, (-1)^{j+1} if q is even.
//
// All D_j terms are positive and small (O(1/p)), eliminating cancellation.
// Kahan compensated summation is used for full f64 precision.

use rayon::prelude::*;

fn compute_area(p: i64, q: i64, pi: f64) -> f64 {
    let pf = p as f64;
    let pi_over_p = pi / pf;
    let sin_a = pi_over_p.sin();
    let cos_a = pi_over_p.cos();

    // Initial sign: +1 if q odd, -1 if q even
    let mut sign: f64 = if q % 2 == 1 { 1.0 } else { -1.0 };

    // Kahan summation for the D_j sum
    let mut s: f64 = 0.0;
    let mut s_comp: f64 = 0.0;

    // Use incremental cos computation via angle addition formula
    // cos((j+1)*a) = cos(j*a)*cos(a) - sin(j*a)*sin(a)
    // sin((j+1)*a) = sin(j*a)*cos(a) + cos(j*a)*sin(a)
    let mut cos_j = 1.0; // cos(0)
    let mut sin_j = 0.0; // sin(0)

    for _ in 0..q {
        let cos_j_plus_1 = cos_j * cos_a - sin_j * sin_a;
        let sin_j_plus_1 = sin_j * cos_a + cos_j * sin_a;
        
        let dj = sign * sin_a / (cos_j * cos_j_plus_1);

        // Kahan add
        let y = dj - s_comp;
        let t = s + y;
        s_comp = (t - s) - y;
        s = t;

        sign = -sign;
        cos_j = cos_j_plus_1;
        sin_j = sin_j_plus_1;
    }

    pf * s
}

fn main() {
    let mut f = [0i64; 38];
    f[0] = 0; f[1] = 1;
    for i in 2..=36 { f[i] = f[i - 1] + f[i - 2]; }

    let pi: f64 = std::f64::consts::PI;

    // Compute areas in parallel
    let areas: Vec<f64> = (3..=34)
        .into_par_iter()
        .map(|n| {
            let p = f[n + 1];
            let q = f[n - 1];
            compute_area(p, q, pi)
        })
        .collect();

    // Combine results with Kahan summation
    let mut total: f64 = 0.0;
    let mut total_comp: f64 = 0.0;

    for area in areas {
        let y = area - total_comp;
        let t = total + y;
        total_comp = (t - total) - y;
        total = t;
    }

    println!("{:.10}", total);
}
