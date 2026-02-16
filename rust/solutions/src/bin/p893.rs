// Problem 893: Matchsticks
// T(N) = sum of M(n) for n=1..N, where M(n) = minimum matchsticks to represent n.
// Representations use digits 0-9, addition (+), multiplication (*) with standard precedence.
// Costs: digits 0=6,1=2,2=5,3=5,4=4,5=5,6=6,7=3,8=7,9=6; operators +,* each cost 2.
// Any expression parses as: T1 + T2 + ... + Tk where Ti is a product of digit-literals.
// T(100) = 916 (verification), answer: T(10^6) = 26688208

fn main() {
    const N: usize = 1_000_000;

    // Matchstick cost per digit
    let dcost: [u8; 10] = [6, 2, 5, 5, 4, 5, 6, 3, 7, 6];

    // digit_cost[n] = cost of writing n as its decimal digits
    let mut dc = vec![0u8; N + 1];
    for n in 1..=N {
        let mut c: u8 = 0;
        let mut x = n;
        while x > 0 {
            c += dcost[x % 10];
            x /= 10;
        }
        dc[n] = c;
    }

    // pcost[n] = min cost to represent n as a product chain of digit-literals
    // Process n from small to large, try all factorizations n = d * (n/d), d >= 2, d*d <= n.
    let mut pcost = dc.clone();

    // Smallest prime factor sieve
    let mut spf = vec![0u32; N + 1];
    for i in 2..=N {
        if spf[i] == 0 {
            let mut j = i;
            while j <= N {
                if spf[j] == 0 {
                    spf[j] = i as u32;
                }
                j += i;
            }
        }
    }

    // For each composite n, try all divisor pairs
    for n in 4..=N {
        if spf[n] == n as u32 {
            continue; // prime
        }

        let sn = (n as f64).sqrt() as usize;

        // Factorize n using SPF
        let mut factors: Vec<(usize, u32)> = Vec::new();
        {
            let mut x = n;
            while x > 1 {
                let p = spf[x] as usize;
                let mut e = 0u32;
                while x % p == 0 {
                    x /= p;
                    e += 1;
                }
                factors.push((p, e));
            }
        }

        // Generate all divisors d with 2 <= d <= sqrt(n)
        fn gen_divs(factors: &[(usize, u32)], idx: usize, cur: usize, lim: usize, pc: &[u8], n: usize, best: &mut u8) {
            if idx == factors.len() {
                if cur >= 2 && cur <= lim {
                    let k = n / cur;
                    let cand = pc[cur] + 2 + pc[k];
                    if cand < *best {
                        *best = cand;
                    }
                }
                return;
            }
            let (p, e) = factors[idx];
            let mut pk = 1usize;
            for _ in 0..=e {
                if cur * pk > lim {
                    break;
                }
                gen_divs(factors, idx + 1, cur * pk, lim, pc, n, best);
                pk *= p;
            }
        }

        let mut best = pcost[n];
        gen_divs(&factors, 0, 1, sn, &pcost, n, &mut best);
        pcost[n] = best;
    }

    // M[n] = min(pcost[n], min_{v} pcost[v] + 2 + M[n-v])
    // Process left to right. Use addends with small pcost.
    // Group addends by their pcost value for efficiency.

    // Collect useful addends with pcost[v] + 2 <= threshold
    let add_threshold: u8 = 17;
    let mut useful_addends: Vec<usize> = (1..=N)
        .filter(|&v| pcost[v] + 2 <= add_threshold)
        .collect();
    useful_addends.sort_unstable();

    let mut m = pcost.clone();

    // Sweep: for each n, try all useful addends
    // Optimization: early exit when pcost[v]+2 alone exceeds current m[n]
    // Group by pcost for better branch prediction
    let mut addends_by_cost: Vec<Vec<usize>> = vec![Vec::new(); add_threshold as usize + 1];
    for &v in &useful_addends {
        addends_by_cost[pcost[v] as usize].push(v);
    }

    for n in 2..=N {
        let mut cur = m[n];
        // min possible improvement: pcost[v]+2+2 = pcost[v]+4
        // so we need pcost[v]+4 <= cur, i.e., pcost[v] <= cur-4
        let max_pcost = if cur >= 4 { cur - 4 } else { continue };

        for p in 2..=max_pcost.min(add_threshold - 2) as usize {
            for &v in &addends_by_cost[p] {
                if v >= n {
                    break;
                }
                let cand = p as u8 + 2 + m[n - v];
                if cand < cur {
                    cur = cand;
                }
            }
        }
        m[n] = cur;
    }

    let total: u64 = (1..=N).map(|i| m[i] as u64).sum();
    println!("{}", total);
}
