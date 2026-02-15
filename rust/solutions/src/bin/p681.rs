// Project Euler 681 - Maximal Area
// Enumerate (w,x,y,z) with wxyz = K^2, w<=x<=y<=z, z < w+x+y, even perimeter.

fn main() {
    let n: i64 = 1_000_000;
    let n2 = n * n;

    // SPF sieve
    let maxn = (n as usize) + 1;
    let mut spf = vec![0u32; maxn];
    for i in 0..maxn { spf[i] = i as u32; }
    let mut i = 2;
    while i * i < maxn {
        if spf[i] == i as u32 {
            let mut j = i * i;
            while j < maxn {
                if spf[j] == j as u32 { spf[j] = i as u32; }
                j += i;
            }
        }
        i += 1;
    }

    // Case 1: z == y, wx must be perfect square
    let mut ans: i64 = 0;
    let mut w = 1i64;
    while w * w * w * w <= n2 {
        let mut x = w;
        while w * x * x * x <= n2 {
            let wx = w * x;
            let swx = (wx as f64).sqrt() as i64;
            let sq = if swx * swx == wx { swx }
                     else if (swx + 1) * (swx + 1) == wx { swx + 1 }
                     else { x += 1; continue };
            if (w + x) & 1 != 0 { x += 1; continue; }
            let y_max = n / sq;
            if y_max < x { x += 1; continue; }
            let count = y_max - x + 1;
            // sum of (w + x + y + y) for y in [x..y_max]
            // = count*(w+x) + 2*sum(y from x to y_max)
            // = count*(w+x) + 2*(count*(x+y_max)/2)
            // = count*(w+x+x+y_max)
            ans += count * (w + x + x + y_max);
            x += 1;
        }
        w += 1;
    }

    // Case 2: z > y
    let mut ans_dpos: i64 = 0;
    for k in 4..=n as usize {
        if spf[k] == k as u32 { continue; } // skip primes

        // Factor K
        let mut prms = Vec::new();
        let mut exps_v = Vec::new();
        let mut tmp = k;
        while tmp > 1 {
            let p = spf[tmp] as usize;
            let mut e = 0;
            while tmp % p == 0 { tmp /= p; e += 1; }
            prms.push(p as i64);
            exps_v.push(e * 2); // exponents of K^2
        }

        let k2 = (k as i64) * (k as i64);

        // Build divisors
        let mut divs: Vec<i64> = vec![1];
        for (idx, &p) in prms.iter().enumerate() {
            let old = divs.len();
            let mut pp = 1i64;
            for _ in 0..exps_v[idx] {
                pp *= p;
                for j in 0..old {
                    divs.push(divs[j] * pp);
                }
            }
        }
        divs.sort_unstable();

        let nd = divs.len();
        for i in 0..nd {
            let w = divs[i];
            if w * w * w * w > k2 { break; }
            let r1 = k2 / w;
            for j in i..nd {
                let x = divs[j];
                if x * x * x > r1 { break; }
                if r1 % x != 0 { continue; }
                let r2 = r1 / x;
                for l in j..nd {
                    let y = divs[l];
                    if y * y > r2 { break; }
                    if r2 % y != 0 { continue; }
                    let z = r2 / y;
                    if z <= y { continue; }
                    if z >= w + x + y { continue; }
                    let total = w + x + y + z;
                    if total & 1 != 0 { continue; }
                    ans_dpos += total;
                }
            }
        }
    }

    println!("{}", ans + ans_dpos);
}
