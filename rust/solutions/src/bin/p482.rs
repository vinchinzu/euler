use euler_utils::{gcd, lcm};

const N: i64 = 10_000_000;

fn gcd128(mut a: i128, mut b: i128) -> i128 {
    if a < 0 { a = -a; }
    if b < 0 { b = -b; }
    while b != 0 { let t = b; b = a % b; a = t; }
    a
}

fn isqrt_func(n: i64) -> i64 {
    if n <= 0 { return 0; }
    let mut x = (n as f64).sqrt() as i64;
    while x * x > n { x -= 1; }
    while (x + 1) * (x + 1) <= n { x += 1; }
    x
}

fn tr(n: i64) -> i64 {
    n * (n + 1) / 2
}

fn get_divisors(n: i64) -> Vec<i64> {
    let mut divs = Vec::new();
    let mut i = 1i64;
    while i * i <= n {
        if n % i == 0 {
            divs.push(i);
            if i != n / i { divs.push(n / i); }
        }
        i += 1;
    }
    divs
}

// Hash set using open addressing
const SOL_HASH_SIZE: usize = 2_000_003;

struct SolSet {
    keys: Vec<(i64, i64, i64)>,
    used: Vec<bool>,
    solutions: Vec<(i64, i64, i64)>,
}

impl SolSet {
    fn new() -> Self {
        SolSet {
            keys: vec![(0, 0, 0); SOL_HASH_SIZE],
            used: vec![false; SOL_HASH_SIZE],
            solutions: Vec::new(),
        }
    }

    fn add(&mut self, mut x: i64, mut y: i64, mut z: i64) -> bool {
        if x > y { std::mem::swap(&mut x, &mut y); }
        if y > z { std::mem::swap(&mut y, &mut z); }
        if x > y { std::mem::swap(&mut x, &mut y); }

        let h = ((x as u64).wrapping_mul(1_000_000_007).wrapping_add(y as u64)).wrapping_mul(1_000_000_007).wrapping_add(z as u64);
        let mut idx = (h % SOL_HASH_SIZE as u64) as usize;
        loop {
            if !self.used[idx] {
                self.keys[idx] = (x, y, z);
                self.used[idx] = true;
                self.solutions.push((x, y, z));
                return true;
            }
            if self.keys[idx] == (x, y, z) {
                return false;
            }
            idx = (idx + 1) % SOL_HASH_SIZE;
        }
    }
}

fn main() {
    let l_val = (N as f64 / 108.0f64.sqrt()) as i64;
    let map_size = (l_val + 10) as usize;

    let mut tri_map: Vec<Vec<i64>> = vec![Vec::new(); map_size];

    let mut m = 2i64;
    while 2 * m * m <= N {
        let mut n = 1i64;
        while n < m && 2 * m * (m + n) <= N {
            if (m + n) % 2 == 1 && gcd(m as u64, n as u64) == 1 {
                let a = m * m - n * n;
                let b = 2 * m * n;
                if (a as usize) < map_size { tri_map[a as usize].push(b); }
                if (b as usize) < map_size { tri_map[b as usize].push(a); }
            }
            n += 1;
        }
        m += 1;
    }

    let mut keys: Vec<usize> = Vec::new();
    for i in 1..map_size {
        if !tri_map[i].is_empty() { keys.push(i); }
    }

    let mut sol_set = SolSet::new();

    for &a1 in &keys {
        let divs = get_divisors(a1 as i64);

        for &d in &divs {
            let mut mult = 1i64;
            while mult * (a1 as i64) < map_size as i64 && mult * d <= a1 as i64 {
                let a2 = (mult * d) as usize;
                if a2 >= map_size || tri_map[a2].is_empty() { mult += 1; continue; }

                let r = lcm(a1 as u64, a2 as u64) as i64;
                if r > N { mult += 1; continue; }

                for bi in 0..tri_map[a1].len() {
                    let b1 = tri_map[a1][bi];
                    for bj in 0..tri_map[a2].len() {
                        let b2 = tri_map[a2][bj];

                        let x = b1 * r / a1 as i64;
                        let y = b2 * r / a2 as i64;
                        let r2: i128 = r as i128 * r as i128;
                        let num: i128 = r2 * (x as i128 + y as i128);
                        let den: i128 = x as i128 * y as i128 - r2;

                        if den > 0 && 2 * (x as i128 + y as i128 + num / den) <= N as i128 {
                            let g = gcd128(num, den);
                            let num_r = num / g;
                            let den_r = den / g;
                            if 2 * (x as i128 * den_r + y as i128 * den_r + num_r) <= N as i128 {
                                sol_set.add((x as i128 * den_r) as i64, (y as i128 * den_r) as i64, num_r as i64);
                            }
                        }
                    }
                }
                mult += 1;
            }
        }
    }

    let mut ans: i64 = 0;
    for &(x, y, z) in &sol_set.solutions {
        let r2 = (x * y * z) / (x + y + z);
        let perim = 2 * (x + y + z);
        let ia = isqrt_func(r2 + x * x);
        let ib = isqrt_func(r2 + y * y);
        let ic = isqrt_func(r2 + z * z);
        ans += tr(N / perim) * (perim + ia + ib + ic);
    }

    println!("{}", ans);
}
