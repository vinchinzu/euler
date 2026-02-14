// Project Euler 395: Pythagorean Tree
const MAX_SEGS: usize = 100000;

fn main() {
    let a: f64 = 3.0;
    let b: f64 = 4.0;
    let c: f64 = 5.0;
    let l_bound = 2.0f64.sqrt() / (1.0 - b / c);

    let transforms = [
        (-a * b / (c * c), b * b / (c * c)),
        ((b - a) * b / (c * c), (b + a) * b / (c * c)),
        ((b + a) * b / (c * c), (b + a) * a / (c * c)),
        (1.0 + a * b / (c * c), a * a / (c * c)),
    ];

    let find_extreme = |dir: usize| -> f64 {
        let mut segs: Vec<((f64, f64), (f64, f64))> = vec![((0.0, 0.0), (1.0, 0.0))];
        let mut extremity = -1e30f64;

        for _ in 0..100 {
            let mut new_segs = Vec::with_capacity(segs.len() * 2);
            for &(start, end) in &segs {
                if new_segs.len() + 2 > MAX_SEGS { break; }
                let diff = (end.0 - start.0, end.1 - start.1);
                let mut pts = [(0.0, 0.0); 4];
                for j in 0..4 {
                    let (tx, ty) = transforms[j];
                    pts[j] = (
                        start.0 + diff.0 * tx - diff.1 * ty,
                        start.1 + diff.0 * ty + diff.1 * tx,
                    );
                }
                new_segs.push((pts[0], pts[1]));
                new_segs.push((pts[2], pts[3]));
            }

            let val = |p: (f64, f64)| match dir {
                0 => p.0, 1 => p.1, 2 => -p.0, 3 => -p.1, _ => 0.0,
            };

            let new_extremity = new_segs.iter()
                .flat_map(|&(s, e)| [val(s), val(e)])
                .fold(-1e30f64, f64::max);

            if (extremity - new_extremity).abs() < 1e-25 { break; }
            extremity = new_extremity;

            // Prune
            segs = new_segs.into_iter().filter(|&(s, e)| {
                let seg_len = ((s.0 - e.0).powi(2) + (s.1 - e.1).powi(2)).sqrt();
                let max_v = val(s).max(val(e));
                max_v > extremity - l_bound * seg_len
            }).collect();
            if segs.len() > MAX_SEGS { segs.truncate(MAX_SEGS); }
        }
        extremity
    };

    let max_x = find_extreme(0);
    let max_y = find_extreme(1);
    let min_x_neg = find_extreme(2);
    let min_y_neg = find_extreme(3);

    let width = max_x + min_x_neg;
    let height = max_y + min_y_neg;
    let area = width * height;

    println!("{:.10}", area);
}
