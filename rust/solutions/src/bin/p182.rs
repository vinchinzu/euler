use euler_utils::gcd;

fn main() {
    let p: u64 = 1009;
    let q: u64 = 3643;
    let phi = (p - 1) * (q - 1);

    let mut min_unconcealed: i64 = -1;
    let mut sum_e: u64 = 0;

    for e in 2..phi {
        if gcd(e, phi) != 1 { continue; }

        let unconcealed = (1 + gcd(e - 1, p - 1)) * (1 + gcd(e - 1, q - 1));

        if min_unconcealed < 0 || (unconcealed as i64) < min_unconcealed {
            min_unconcealed = unconcealed as i64;
            sum_e = e;
        } else if unconcealed as i64 == min_unconcealed {
            sum_e += e;
        }
    }

    println!("{}", sum_e);
}
