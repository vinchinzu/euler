// Project Euler 287: Quadtree Encoding
const N: u32 = 24;
const L: i64 = 1 << (N - 1);

fn black(x: i64, y: i64) -> bool {
    (x - L) * (x - L) + (y - L) * (y - L) <= L * L
}

fn len_enc(x: i32, y: i32, side: i32) -> i64 {
    let mut stack: Vec<(i32, i32, i32)> = Vec::with_capacity(100000);
    stack.push((x, y, side));
    let mut bits: i64 = 0;

    while let Some((cx, cy, cs)) = stack.pop() {
        let b00 = black(cx as i64, cy as i64);
        let b11 = black((cx + cs - 1) as i64, (cy + cs - 1) as i64);
        let b10 = black((cx + cs - 1) as i64, cy as i64);
        let b01 = black(cx as i64, (cy + cs - 1) as i64);
        if b00 == b11 && b10 == b01 {
            bits += 2;
        } else {
            let half = cs >> 1;
            bits += 1;
            stack.push((cx, cy, half));
            stack.push((cx + half, cy, half));
            stack.push((cx, cy + half, half));
            stack.push((cx + half, cy + half, half));
        }
    }
    bits
}

fn main() {
    let l = L as i32;
    let ans = 1 + len_enc(0, 0, l) + 2 * len_enc(l, 0, l) + len_enc(l, l, l);
    println!("{ans}");
}
