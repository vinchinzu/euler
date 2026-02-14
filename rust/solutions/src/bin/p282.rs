// Project Euler 282: The Ackermann Function
use euler_utils::{mod_pow, mod_mul, euler_phi};

fn tower2(height: u64, m: u64) -> u64 {
    if m == 1 { return 0; }
    if m == 2 { return 0; }
    if height == 0 { return 1 % m; }
    let phi_m = euler_phi(m);
    let exp = tower2(height - 1, phi_m);
    mod_pow(2, exp, m)
}

fn main() {
    let mut big_m: u64 = 1;
    for _ in 0..8 { big_m *= 14; }

    let mut ans: u64 = 0;
    // A(0,0) = 1, A(1,1) = 3, A(2,2) = 7, A(3,3) = 61
    ans = (ans + 1) % big_m;
    ans = (ans + 3) % big_m;
    ans = (ans + 7) % big_m;
    ans = (ans + 61) % big_m;

    // A(4,4) = 2^^7 - 3
    let a44 = (tower2(7, big_m) + big_m - 3) % big_m;
    ans = (ans + a44) % big_m;

    // A(5,5) and A(6,6) converge for large tower height
    let a55 = (tower2(1_000_000_000_000_000_000, big_m) + big_m - 3) % big_m;
    ans = (ans + a55) % big_m;

    let a66 = (tower2(1_000_000_000_000_000_000, big_m) + big_m - 3) % big_m;
    ans = (ans + a66) % big_m;

    println!("{ans}");
}
