use euler_utils::gcd;

const N: usize = 1_000_000;

fn main() {
    let mut height_lists: Vec<Vec<i64>> = vec![Vec::new(); N + 1];

    // Generate Pythagorean triples
    let mut m = 2;
    while m * m + 1 < N {
        for n in 1..m {
            if gcd(m as u64, n as u64) != 1 { continue; }
            if m % 2 == n % 2 { continue; }

            let a = m * m - n * n;
            let b = 2 * m * n;
            let c = m * m + n * n;

            let mut k = 1;
            while k * c < N {
                let ka = k * a;
                let kb = k * b;
                if ka <= N { height_lists[ka].push(kb as i64); }
                if kb <= N { height_lists[kb].push(ka as i64); }
                k += 1;
            }
        }
        m += 1;
    }

    let mut count: i64 = 0;

    for w in 1..N {
        let hs = &height_lists[w];
        let nh = hs.len();
        if nh < 2 { continue; }

        for i in 0..nh {
            for j in (i + 1)..nh {
                let h1 = hs[i];
                let h2 = hs[j];
                if (h1 * h2) % (h1 + h2) == 0 {
                    count += 1;
                }
            }
        }
    }

    println!("{}", count);
}
