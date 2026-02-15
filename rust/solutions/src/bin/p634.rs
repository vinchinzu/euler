// Project Euler 634 - Numbers of the form a^2 * b^3
// Count squarefree b with b^3 <= N, sum sqrt(N/b^3)-1, subtract corrections

fn isqrt(n: i64) -> i64 {
    if n <= 0 { return 0; }
    let mut x = (n as f64).sqrt() as i64;
    while x * x > n { x -= 1; }
    while (x + 1) * (x + 1) <= n { x += 1; }
    x
}

fn icbrt(n: i64) -> i64 {
    if n <= 0 { return 0; }
    let mut x = (n as f64).cbrt() as i64;
    while x > 0 && x * x * x > n { x -= 1; }
    while (x + 1) * (x + 1) * (x + 1) <= n { x += 1; }
    x
}

fn main() {
    let n: i64 = 9_000_000_000_000_000_000;
    let l = icbrt(n) as usize;
    let lim = l + 1;

    let mut mobius = vec![1i32; lim + 1];
    let mut is_prime_arr = vec![true; lim + 1];
    is_prime_arr[0] = false;
    if lim >= 1 { is_prime_arr[1] = false; }

    for i in 2..=lim {
        if is_prime_arr[i] {
            for j in (i..=lim).step_by(i) {
                if j != i { is_prime_arr[j] = false; }
                mobius[j] *= -1;
            }
            let mut j = (i as u64) * (i as u64);
            while j <= lim as u64 { mobius[j as usize] = 0; j += (i as u64) * (i as u64); }
        }
    }

    let mut ans: i64 = 0;
    for b in 2..=l as i64 {
        if mobius[b as usize] != 0 {
            let q = n / (b * b * b);
            ans += isqrt(q) - 1;
        }
    }

    let sqrt_n = isqrt(n);
    let mut i_max = icbrt(sqrt_n);
    while (i_max + 1) * (i_max + 1) * (i_max + 1) <= sqrt_n { i_max += 1; }
    while i_max > 0 && i_max * i_max * i_max > sqrt_n { i_max -= 1; }

    for i in 2..=i_max {
        let i3 = i * i * i;
        ans -= mobius[i as usize] as i64 * (sqrt_n / i3);
        if is_prime_arr[i as usize] { ans -= 1; }
    }

    println!("{}", ans);
}
