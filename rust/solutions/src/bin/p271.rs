// Project Euler 271 - Modular Cubes, part 1
//
// Find the sum of all cube roots of 1 (mod N), other than 1 itself.
// N = 13082761331670030 = 2*3*5*7*11*13*17*19*23*29*31*37*41*43

fn ext_gcd(a: i128, b: i128) -> (i128, i128, i128) {
    if b == 0 {
        return (a, 1, 0);
    }
    let (g, x1, y1) = ext_gcd(b, a % b);
    (g, y1, x1 - (a / b) * y1)
}

fn crt(remainders: &[i128], moduli: &[i128]) -> i128 {
    let mut result = remainders[0];
    let mut m = moduli[0];

    for i in 1..remainders.len() {
        let (g, x, _) = ext_gcd(m, moduli[i]);
        let diff = remainders[i] - result;
        let lcm = m / g * moduli[i];
        result = result + m * x % lcm * (diff / g);
        m = lcm;
        result = ((result % m) + m) % m;
    }
    result
}

fn main() {
    let primes: Vec<i128> = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43];

    // Find cube roots of 1 mod each prime
    let mut cube_roots: Vec<Vec<i128>> = Vec::new();
    for &p in &primes {
        let mut roots = Vec::new();
        for x in 0..p {
            if (x * x % p) * x % p == 1 {
                roots.push(x);
            }
        }
        cube_roots.push(roots);
    }

    // Enumerate all combinations and CRT
    let n = primes.len();
    let mut indices = vec![0usize; n];
    let mut total: i128 = 0;

    loop {
        let rems: Vec<i128> = (0..n).map(|i| cube_roots[i][indices[i]]).collect();
        total += crt(&rems, &primes);

        // Increment indices
        let mut carry = true;
        for i in (0..n).rev() {
            if !carry {
                break;
            }
            indices[i] += 1;
            if indices[i] >= cube_roots[i].len() {
                indices[i] = 0;
            } else {
                carry = false;
            }
        }
        if carry {
            break;
        }
    }

    total -= 1; // exclude 1 itself
    println!("{}", total);
}
