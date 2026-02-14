// Project Euler 277: A Modified Collatz sequence
// Find smallest integer > 10^15 whose sequence starts with given string.

fn main() {
    let s = b"UDDDUdddDDUDDddDdDddDDUDDdUUDd";
    let n_limit: i128 = 1_000_000_000_000_000; // 10^15

    let mut modulus: i128 = 1;
    let mut ans: i128 = 0;

    for i in (0..s.len()).rev() {
        modulus *= 3;
        match s[i] {
            b'D' => {
                ans *= 3;
            }
            b'U' => {
                ans = 3 * ans - 2;
                while ans % 4 != 0 {
                    ans += modulus;
                }
                ans /= 4;
            }
            b'd' => {
                ans = 3 * ans + 1;
                while ans % 2 != 0 {
                    ans += modulus;
                }
                ans /= 2;
            }
            _ => {}
        }
    }

    while ans <= n_limit {
        ans += modulus;
    }

    println!("{}", ans);
}
