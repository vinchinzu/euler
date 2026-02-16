// Project Euler 385 - Ellipses Inside Triangles
//
// Find sum of areas of triangles with integer vertices |coord| <= N=10^9
// whose largest inscribed ellipse (Steiner inellipse) has foci at (±√13, 0).
//
// By Marden's theorem, the Steiner inellipse foci are the roots of p'(z)
// where p(z) = (z-z1)(z-z2)(z-z3). This requires:
//   - Centroid at origin: z1+z2+z3 = 0
//   - z1² + z1·z2 + z2² = 39 (real, imaginary part = 0)
//
// Parametrization via generalized Pell equations on the Eisenstein norm form:
//   D·(3m² - k²) = 39, where D = p²-pq+q² with gcd(p,q)=1
// Only D=1 (E=39) and D=13 (E=3) yield solutions.
//
// Triangle vertices: z1=(m(2p-q), -qk), z2=(m(2q-p), pk), z3=(-m(p+q), (q-p)k)
// Area = 3·|m·k|·D

use std::collections::HashSet;

fn pell_chain(m0: i64, k0: i64, m_limit: i64) -> Vec<(i64, i64)> {
    let mut results = Vec::new();
    let (mut m, mut k) = (m0, k0);
    while m <= m_limit {
        results.push((m, k));
        let (m_new, k_new) = (k + 2 * m, 2 * k + 3 * m);
        m = m_new;
        k = k_new;
    }
    results
}

fn main() {
    let n: i64 = 1_000_000_000;

    // (p, q) pairs with p²-pq+q²=D, gcd(|p|,|q|)=1
    let pq_d1: Vec<(i64, i64)> = vec![
        (0, 1), (1, 0), (1, 1), (0, -1), (-1, 0), (-1, -1),
    ];
    let pq_d13: Vec<(i64, i64)> = vec![
        (-4, -3), (-4, -1), (-3, -4), (-3, 1), (-1, -4), (-1, 3),
        (1, -3), (1, 4), (3, -1), (3, 4), (4, 1), (4, 3),
    ];

    // Pell chains for 3m²-k²=39 (two families)
    let chain_limit = n * 10; // generous upper bound for m values
    let mut chains_e39 = pell_chain(4, 3, chain_limit);
    chains_e39.extend(pell_chain(5, 6, chain_limit));

    // Pell chain for 3m²-k²=3 (one family, skip k=0)
    let chains_e3_raw = pell_chain(1, 0, chain_limit);
    let chains_e3: Vec<(i64, i64)> = chains_e3_raw
        .into_iter()
        .filter(|&(_, k)| k > 0)
        .collect();

    let mut seen: HashSet<[(i64, i64); 3]> = HashSet::new();
    let mut total: i128 = 0;

    let mut process = |p: i64, q: i64, m: i64, k: i64, _d: i64| {
        let signs: Vec<i64> = if k != 0 { vec![k, -k] } else { vec![0] };
        for ks in signs {
            let a = m * (2 * p - q);
            let b = -q * ks;
            let c = m * (2 * q - p);
            let dd = p * ks;
            let x3 = -m * (p + q);
            let y3 = (q - p) * ks;

            if a.abs() > n || b.abs() > n || c.abs() > n || dd.abs() > n
                || x3.abs() > n || y3.abs() > n
            {
                continue;
            }

            let det = a * dd - c * b;
            if det == 0 {
                continue;
            }

            // Sort the three points to create a canonical key
            let mut pts = [(a, b), (c, dd), (x3, y3)];
            pts.sort();
            if pts[0] == pts[1] || pts[1] == pts[2] {
                continue;
            }
            if seen.contains(&pts) {
                continue;
            }
            seen.insert(pts);

            let area = 3 * det.unsigned_abs() as i128 / 2;
            total += area;
        }
    };

    // D=1 cases
    for &(p, q) in &pq_d1 {
        for &(m, k) in &chains_e39 {
            process(p, q, m, k, 1);
        }
    }

    // D=13 cases
    for &(p, q) in &pq_d13 {
        for &(m, k) in &chains_e3 {
            process(p, q, m, k, 13);
        }
    }

    println!("{}", total);
}
