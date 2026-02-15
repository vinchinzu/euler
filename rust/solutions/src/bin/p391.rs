const NN: usize = 1010;

fn main() {
    let mut buf = vec![vec![0i16; NN * NN]; 2];

    let mut ans: i64 = 0;
    for i in 1..=1000usize {
        let n = i + 1;
        let mut fi = 0usize;

        // Initialize
        {
            let f = &mut buf[fi];
            for s in 0..std::cmp::min(n + 2, NN) {
                for x in 0..std::cmp::min(n + 1, NN) {
                    if s + x >= n {
                        f[s * NN + x] = 0;
                    } else {
                        f[s * NN + x] = (s + x) as i16;
                    }
                }
            }
        }

        for h in 1..=n {
            let gi = 1 - fi;
            let (src, dst) = if fi < gi {
                let (a, b) = buf.split_at_mut(gi);
                (a[fi].as_ptr(), b[0].as_mut_ptr())
            } else {
                let (a, b) = buf.split_at_mut(fi);
                (b[0].as_ptr(), a[gi].as_mut_ptr())
            };
            let xlim = std::cmp::min(n + 1, NN);
            for s in 0..=(n - h) {
                unsafe {
                    let s1_base = src.add((s + 1) * NN);
                    let s_base_r = src.add(s * NN);
                    let s_base_w = dst.add(s * NN);
                    for x in 0..xlim {
                        let y = *s1_base.add(x) as usize;
                        *s_base_w.add(x) = *s_base_r.add(y);
                    }
                }
            }
            fi = gi;
        }

        let x = buf[fi][0] as i64;
        ans += x * x * x;
    }
    println!("{}", ans);
}
