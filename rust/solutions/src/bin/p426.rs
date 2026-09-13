// Project Euler 426: Box-Ball System
struct Frame {
    start: usize,
    i: usize,
    diff: i64,
    res: i64,
    phase: u8,
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
                // SAFETY: start < lengths.len() by condition
                let val = unsafe { *lengths.get_unchecked(start) as i64 };
                return_val = val * val;
                stack.pop();
                continue;
            }

            loop {
                let i = f.i;
                let len = lengths.len();
                
                // SAFETY: i < len by loop invariant (i starts at valid start, only advances within bounds)
                f.diff += unsafe { *lengths.get_unchecked(i) as i64 };

                if i + 1 == len {
                    lengths.push(f.diff as i32);
                    f.start = start + 1;
                    f.i = start + 1;
                    f.diff = 0;
                    f.phase = 0;
                    f.res = 0;
                    break;
                }
                
                // SAFETY: i + 1 < len verified by check above
                let next_val = unsafe { *lengths.get_unchecked(i + 1) as i64 };
                if f.diff <= next_val {
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

                f.diff -= next_val;
                f.i = i + 2;
            }
        } else if f.phase == 1 {
            f.res = return_val;
            let i = f.i;
            lengths.truncate(i + 1);
            lengths.push(f.diff as i32);

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
            return_val = f.res + return_val;
            stack.pop();
        }
    }

    println!("{}", return_val);
}
