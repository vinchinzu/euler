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

fn main() {
    let mut f = [0i64; 38];
    f[0] = 0; f[1] = 1;
    for i in 2..=36 { f[i] = f[i - 1] + f[i - 2]; }

    let pi: f64 = std::f64::consts::PI;
    let mut total: f64 = 0.0;
    let mut total_comp: f64 = 0.0;

    for n in 3..=34 {
        let p = f[n + 1];
        let q = f[n - 1];

        let pf = p as f64;
        let pi_over_p = pi / pf;
        let sin_a = pi_over_p.sin();

        // Initial sign: +1 if q odd, -1 if q even
        let mut sign: f64 = if q % 2 == 1 { 1.0 } else { -1.0 };

        // Kahan summation for the D_j sum
        let mut s: f64 = 0.0;
        let mut s_comp: f64 = 0.0;

        for j in 0..q {
            let c1 = (j as f64 * pi_over_p).cos();
            let c2 = ((j + 1) as f64 * pi_over_p).cos();
            let dj = sign * sin_a / (c1 * c2);

            // Kahan add
            let y = dj - s_comp;
            let t = s + y;
            s_comp = (t - s) - y;
            s = t;

            sign = -sign;
        }

        let area = pf * s;

        // Kahan add to total
        let y = area - total_comp;
        let t = total + y;
        total_comp = (t - total) - y;
        total = t;
    }

    println!("{:.10}", total);
}
