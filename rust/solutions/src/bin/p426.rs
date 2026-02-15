// Project Euler 426: Box-Ball System
struct Frame {
    start: usize,
    i: usize,
    diff: i64,
    phase: i32,
    res: i64,
}

fn main() {
    let n = 10_000_000usize;

    let mut lengths: Vec<i32> = Vec::with_capacity(n + 2);
    let mut s: i64 = 290797;
    let modv: i64 = 50515093;
    for _ in 0..=n {
        lengths.push((s % 64) as i32 + 1);
        s = (s * s) % modv;
    }

    let mut stack: Vec<Frame> = Vec::with_capacity(6_000_000);
    stack.push(Frame { start: 0, i: 0, diff: 0, phase: 0, res: 0 });

    let mut return_val: i64 = 0;

    while let Some(f) = stack.last_mut() {
        if f.phase == 0 {
            let start = f.start;

            if start + 1 == lengths.len() {
                return_val = lengths[start] as i64 * lengths[start] as i64;
                stack.pop();
                continue;
            }

            loop {
                let i = f.i;
                f.diff += lengths[i] as i64;

                if i + 1 == lengths.len() {
                    let diff = f.diff as i32;
                    lengths.push(diff);
                    f.start = start + 1;
                    f.i = start + 1;
                    f.diff = 0;
                    f.phase = 0;
                    f.res = 0;
                    break;
                } else if f.diff <= lengths[i + 1] as i64 {
                    f.phase = 1;
                    stack.push(Frame {
                        start: i + 2,
                        i: i + 2,
                        diff: 0,
                        phase: 0,
                        res: 0,
                    });
                    break;
                }

                f.diff -= lengths[i + 1] as i64;
                f.i = i + 2;
            }
        } else if f.phase == 1 {
            f.res = return_val;
            let i = f.i;
            lengths.truncate(i + 1);
            let diff = f.diff as i32;
            lengths.push(diff);

            f.phase = 2;
            let start = f.start;
            stack.push(Frame {
                start: start + 1,
                i: start + 1,
                diff: 0,
                phase: 0,
                res: 0,
            });
        } else {
            // phase == 2
            return_val = f.res + return_val;
            stack.pop();
        }
    }

    println!("{}", return_val);
}
