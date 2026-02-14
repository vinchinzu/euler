// Project Euler 269: Polynomials with at least one integer root
use std::collections::HashMap;

const B: usize = 10;
const NSTEP: usize = 16;
const NONE: i32 = i32::MIN;

fn main() {
    let init = [0i32; B];

    let mut cur: HashMap<[i32; B], i64> = HashMap::new();
    cur.insert(init, 1);

    for _step in 1..=NSTEP {
        let mut new_map: HashMap<[i32; B], i64> = HashMap::new();

        for (key, &count) in &cur {
            for d in 0..B as i32 {
                let mut nk = [0i32; B];
                for j in 0..B {
                    let v = key[j];
                    if v == NONE {
                        nk[j] = NONE;
                    } else {
                        let new_v = v * (-(j as i32)) + d;
                        if new_v > 0 {
                            let t = (new_v * (-(j as i32)) + B as i32 - 1) * (-(j as i32));
                            nk[j] = if t >= new_v { NONE } else { new_v };
                        } else if new_v < 0 {
                            let t = new_v * (j as i32) * (j as i32) + B as i32 - 1;
                            nk[j] = if t <= new_v { NONE } else { new_v };
                        } else {
                            nk[j] = 0;
                        }
                    }
                }
                *new_map.entry(nk).or_insert(0) += count;
            }
        }

        cur = new_map;
    }

    let mut ans: i64 = 0;
    for (key, &count) in &cur {
        if key.iter().any(|&v| v == 0) {
            ans += count;
        }
    }

    println!("{}", ans);
}
