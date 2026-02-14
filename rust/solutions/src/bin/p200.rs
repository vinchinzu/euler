// Project Euler 200 - Find the 200th prime-proof sqube containing "200".
use euler_utils::miller_rabin;

fn contains_200(n: u64) -> bool {
    let s = n.to_string();
    s.contains("200")
}

fn prime_proof(n: u64) -> bool {
    let s: Vec<u8> = n.to_string().into_bytes();
    let len = s.len();
    for i in 0..len {
        let orig = s[i];
        for ch in b'0'..=b'9' {
            if ch == orig { continue; }
            if i == 0 && ch == b'0' { continue; }
            if i == len - 1 && ch == b'0' { continue; }
            if i == len - 1 && ch == b'5' && len > 1 { continue; }
            let mut modified = s.clone();
            modified[i] = ch;
            let m: u64 = String::from_utf8(modified).unwrap().parse().unwrap();
            if m >= 2 && miller_rabin(m) {
                return false;
            }
        }
    }
    true
}

fn sieve_primes(limit: usize) -> Vec<usize> {
    let mut is_p = vec![true; limit + 1];
    is_p[0] = false;
    if limit >= 1 { is_p[1] = false; }
    let mut i = 2;
    while i * i <= limit {
        if is_p[i] {
            let mut j = i * i;
            while j <= limit { is_p[j] = false; j += i; }
        }
        i += 1;
    }
    (2..=limit).filter(|&i| is_p[i]).collect()
}

fn main() {
    let target_index = 200;
    let limit: u64 = 300_000_000_000;
    let bound = 600_000;
    let primes = sieve_primes(bound);

    let mut squbes: Vec<u64> = Vec::new();

    for i in 0..primes.len() {
        let p = primes[i] as u64;
        let p2 = p * p;
        if p2 > limit { break; }
        let q_max_cube = limit / p2;
        let mut q_max = (q_max_cube as f64).cbrt() as u64;
        while (q_max + 1) * (q_max + 1) * (q_max + 1) <= q_max_cube { q_max += 1; }
        while q_max > 0 && q_max * q_max * q_max > q_max_cube { q_max -= 1; }
        if q_max < 2 { continue; }

        for j in 0..primes.len() {
            let q = primes[j] as u64;
            if q > q_max { break; }
            if q == p { continue; }
            let v = p2 * q * q * q;
            if v > limit { break; }
            if contains_200(v) {
                squbes.push(v);
            }
        }
    }

    squbes.sort();
    squbes.dedup();

    let mut count = 0;
    for &v in &squbes {
        if prime_proof(v) {
            count += 1;
            if count == target_index {
                println!("{}", v);
                return;
            }
        }
    }
}
