// Project Euler 375: Minimum of subsequences
// BBS RNG, find period, stack algorithm, Lagrange interpolation.

fn main() {
    let n_val: i64 = 2_000_000_000;
    const BBS_MOD: i64 = 50515093;

    // Find period of BBS
    let mut s: i64 = 290797;
    s = s * s % BBS_MOD;
    let first = s;
    let mut period: i64 = 1;
    s = s * s % BBS_MOD;
    while s != first {
        s = s * s % BBS_MOD;
        period += 1;
    }

    let start = n_val % period;

    let mut points = [0i128; 3];

    for mult in 1..=3 {
        let n = start + mult as i64 * period;

        let mut stack_pos: Vec<i64> = vec![0];
        let mut stack_val: Vec<i64> = vec![-1];

        let mut m: i128 = 0;
        let mut sv: i64 = 290797;

        for pos in 1..=n {
            sv = sv * sv % BBS_MOD;

            while *stack_val.last().unwrap() > sv {
                let v = stack_val.pop().unwrap();
                let p = stack_pos.pop().unwrap();
                let prev_p = *stack_pos.last().unwrap();
                m += v as i128 * (p - prev_p) as i128 * (pos - p) as i128;
            }

            stack_pos.push(pos);
            stack_val.push(sv);
        }

        // Flush remaining
        let pos = n + 1;
        while stack_pos.len() > 1 {
            let v = stack_val.pop().unwrap();
            let p = stack_pos.pop().unwrap();
            let prev_p = *stack_pos.last().unwrap();
            m += v as i128 * (p - prev_p) as i128 * (pos - p) as i128;
        }

        points[mult - 1] = m;
    }

    // Lagrange interpolation through (1, y0), (2, y1), (3, y2)
    let k = n_val / period;

    let y0 = points[0];
    let y1 = points[1];
    let y2 = points[2];
    let d0 = y0;
    let d1 = y1 - y0;
    let d2 = y2 - 2 * y1 + y0;

    let result = d0 + d1 * (k as i128 - 1) + d2 * (k as i128 - 1) * (k as i128 - 2) / 2;

    println!("{}", result as i64);
}
