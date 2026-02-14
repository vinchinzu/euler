// Project Euler 248: Numbers for which Euler's totient equals 13!
use euler_utils::is_prime;

const KF: i64 = 6_227_020_800; // 13!
const TARGET_N: usize = 150_000;

fn get_divisors(n: i64) -> Vec<i64> {
    let small_primes = [2i64, 3, 5, 7, 11, 13];
    let mut divs = vec![1i64];
    let mut temp = n;
    for &p in &small_primes {
        if temp % p == 0 {
            let sz = divs.len();
            let mut power = 1i64;
            while temp % p == 0 {
                temp /= p;
                power *= p;
                for i in 0..sz {
                    divs.push(divs[i] * power);
                }
            }
        }
    }
    if temp > 1 {
        let sz = divs.len();
        for i in 0..sz {
            divs.push(divs[i] * temp);
        }
    }
    divs
}

fn main() {
    let divisors = get_divisors(KF);

    let mut nums: Vec<(i64, i64)> = vec![(1, 1)]; // (prod, phi)

    for &d in &divisors {
        let p = KF / d + 1;
        if p < 2 { continue; }
        if !is_prime(p as u64) { continue; }

        let old_len = nums.len();
        let mut new_entries = Vec::new();
        for i in 0..old_len {
            let (prod, phi) = nums[i];
            let mut pe = 1i128;
            loop {
                let new_phi = phi as i128 * pe * (p as i128 - 1);
                if new_phi > KF as i128 { break; }
                if KF as i128 % new_phi != 0 { break; }
                new_entries.push((prod * pe as i64 * p, new_phi as i64));
                pe *= p as i128;
            }
        }
        nums.extend(new_entries);
    }

    let mut valid: Vec<i64> = nums.iter()
        .filter(|&&(_, phi)| phi == KF)
        .map(|&(prod, _)| prod)
        .collect();

    valid.sort();
    println!("{}", valid[TARGET_N - 1]);
}
