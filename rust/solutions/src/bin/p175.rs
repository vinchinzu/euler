use euler_utils::gcd;

fn main() {
    let mut p = 123456789i64;
    let mut q = 987654321i64;

    let g = gcd(p as u64, q as u64) as i64;
    p /= g;
    q /= g;

    let mut cf = Vec::new();

    let (mut num, mut den, desired_parity) = if p < q {
        (q, p, 1)
    } else {
        (p, q, 0)
    };

    while den != 0 {
        let a = num / den;
        cf.push(a);
        let tmp = den;
        den = num - a * den;
        num = tmp;
    }

    while cf.len() % 2 != desired_parity {
        let last = cf.len() - 1;
        if cf[last] > 1 {
            cf[last] -= 1;
            cf.push(1);
        } else if cf.len() == 1 {
            cf[0] += 1;
        } else {
            let len = cf.len();
            cf[len - 2] += 1;
            cf.pop();
        }
    }

    cf.reverse();

    let result: Vec<String> = cf.iter().map(|x| x.to_string()).collect();
    println!("{}", result.join(","));
}
