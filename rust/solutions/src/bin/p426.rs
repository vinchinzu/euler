// Project Euler 426: Box-Ball System
#[repr(C)]
struct Frame {
    start: usize,
    i: usize,
    diff: i64,
    res: i64,
    phase: u8,
}

#[inline(always)]
unsafe fn get_len_unchecked(lengths: &[i32], idx: usize) -> i64 {
    *lengths.get_unchecked(idx) as i64
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
        match f.phase {
            0 => {
                let start = f.start;

                if start + 1 == lengths.len() {
                    // SAFETY: start < len by condition above
                    let val = unsafe { get_len_unchecked(&lengths, start) };
                    return_val = val * val;
                    stack.pop();
                    continue;
                }

                let mut i = f.i;
                let mut diff = f.diff;
                
                loop {
                    let len = lengths.len();
                    
                    // SAFETY: i < len maintained by loop invariant
                    diff += unsafe { get_len_unchecked(&lengths, i) };

                    if i + 1 == len {
                        lengths.push(diff as i32);
                        f.start = start + 1;
                        f.i = start + 1;
                        f.diff = 0;
                        f.phase = 0;
                        f.res = 0;
                        break;
                    }
                    
                    // SAFETY: i + 1 < len verified by check above
                    let next_val = unsafe { get_len_unchecked(&lengths, i + 1) };
                    if diff <= next_val {
                        f.diff = diff;
                        f.i = i;
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
                    
                    diff -= next_val;
                    i += 2;
                }
            }
            1 => {
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
            }
            _ => {
                return_val = f.res + return_val;
                stack.pop();
            }
        }
    }

    println!("{}", return_val);
}
