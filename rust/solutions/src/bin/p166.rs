// Project Euler 166: Criss Cross - 4x4 grids with equal row/col/diagonal sums

fn main() {
    let mut total: i64 = 0;

    for s in 0..=36i32 {
        if s == 0 { total += 1; continue; }

        for a in 0..=9i32.min(s) {
            for b in 0..=9i32.min(s - a) {
                let ab = a + b;
                for c in 0..=9i32.min(s - ab) {
                    let d = s - ab - c;
                    if d < 0 || d > 9 { continue; }

                    for e in 0..=9i32.min(s - a) {
                        for f in 0..=9i32.min(s - b) {
                            let ef = e + f;
                            for g in 0..=9i32.min(s - c) {
                                let h = s - e - f - g;
                                if h < 0 || h > 9 { continue; }
                                if d + h > s { continue; }

                                let i_min = 0i32.max(s - 9 - (a + e));
                                let i_max = 9i32.min(s - (a + e));
                                if i_min > i_max { continue; }

                                for i in i_min..=i_max {
                                    let j = a + e + i - d - g;
                                    if j < 0 || j > 9 { continue; }

                                    let two_l = s - i - j - d - h + a + f;
                                    if two_l < 0 || two_l > 18 || two_l % 2 != 0 { continue; }
                                    let l = two_l / 2;
                                    if l < 0 || l > 9 { continue; }

                                    let k = s - i - j - l;
                                    if k < 0 || k > 9 { continue; }

                                    let m = s - a - e - i;
                                    if m < 0 || m > 9 { continue; }

                                    let n = s - b - f - j;
                                    if n < 0 || n > 9 { continue; }

                                    let o = s - c - g - k;
                                    if o < 0 || o > 9 { continue; }

                                    let p = s - d - h - l;
                                    if p < 0 || p > 9 { continue; }

                                    if m + n + o + p != s { continue; }
                                    if a + f + k + p != s { continue; }
                                    if d + g + j + m != s { continue; }

                                    total += 1;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    println!("{}", total);
}
