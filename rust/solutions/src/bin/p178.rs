// Project Euler 178: Step Numbers
fn main() {
    const MAX_LEN: usize = 40;
    const FULL_MASK: usize = 1023;

    let mut cur = [[0i64; 1024]; 10];
    for d in 1..=9usize {
        cur[d][1 << d] = 1;
    }

    let mut total: i64 = 0;

    for step in 0..MAX_LEN - 1 {
        let mut nxt = [[0i64; 1024]; 10];

        for digit in 0..=9usize {
            for mask in 0..=FULL_MASK {
                let count = cur[digit][mask];
                if count == 0 { continue; }

                if digit > 0 {
                    let nd = digit - 1;
                    let nm = mask | (1 << nd);
                    nxt[nd][nm] += count;
                }
                if digit < 9 {
                    let nd = digit + 1;
                    let nm = mask | (1 << nd);
                    nxt[nd][nm] += count;
                }
            }
        }

        cur = nxt;

        if step + 2 >= 10 {
            for digit in 0..=9 {
                total += cur[digit][FULL_MASK];
            }
        }
    }

    println!("{}", total);
}
