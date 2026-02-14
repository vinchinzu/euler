// Project Euler 230: Fibonacci Words
const A: &[u8] = b"1415926535897932384626433832795028841971693993751058209749445923078164062862089986280348253421170679";
const B: &[u8] = b"8214808651328230664709384460955058223172535940812848111745028410270193852110555964462294895493038196";

fn digit(mut n: i64) -> i64 {
    let mut lens = vec![0i64; 200];
    lens[0] = 100;
    lens[1] = 100;
    let mut k = 1;
    while lens[k] < n {
        k += 1;
        lens[k] = lens[k - 2] + lens[k - 1];
        if lens[k] > 2_000_000_000_000_000_000 { break; }
    }
    while k >= 2 {
        if n <= lens[k - 2] {
            k -= 2;
        } else {
            n -= lens[k - 2];
            k -= 1;
        }
    }
    if k == 0 {
        (A[(n - 1) as usize] - b'0') as i64
    } else {
        (B[(n - 1) as usize] - b'0') as i64
    }
}

fn main() {
    let mut ans: i64 = 0;
    let mut power10: i64 = 1;
    let mut power7: i64 = 1;

    for n in 0..18 {
        let pos = (127 + 19 * n) * power7;
        ans += power10 * digit(pos);
        power10 *= 10;
        power7 *= 7;
    }

    println!("{ans}");
}
