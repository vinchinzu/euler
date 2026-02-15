// Project Euler 823 - Factor Shuffle

const N: usize = 10000;
const MOD: i64 = 1234567891;

#[derive(Clone, Copy)]
struct Factor {
    prime: i32,
    tiebreak: i32,
}

#[derive(Clone)]
struct Number {
    factors: Vec<Factor>,
}

fn main() {
    let k: i64 = 10_000_000_000_000_000;

    // SPF sieve
    let mut spf = vec![0i32; N + 1];
    for i in 0..=N { spf[i] = i as i32; }
    for i in 2..=N {
        if (i as i64) * (i as i64) > N as i64 { break; }
        if spf[i] == i as i32 {
            let mut j = i * i;
            while j <= N {
                if spf[j] == j as i32 { spf[j] = i as i32; }
                j += i;
            }
        }
    }

    let mut numbers: Vec<Number> = Vec::new();
    let mut total_factors = 0i64;

    for n in 2..=N {
        let mut temp = Vec::new();
        let mut x = n;
        while x > 1 {
            let p = spf[x];
            temp.push(p);
            x = (x as i32 / p) as usize;
        }
        let fc = temp.len();
        let mut factors: Vec<Factor> = Vec::with_capacity(fc);
        for (i, &p) in temp.iter().enumerate() {
            factors.push(Factor {
                prime: p,
                tiebreak: (i as i32) * (N as i32) + (n as i32),
            });
        }
        factors.sort_by(|a, b| {
            b.prime.cmp(&a.prime).then(b.tiebreak.cmp(&a.tiebreak))
        });
        numbers.push(Number { factors });
        total_factors += fc as i64;
    }

    let shuffle_rounds = 2 * total_factors;

    let do_shuffle = |numbers: &mut Vec<Number>| {
        let mut collected: Vec<Factor> = Vec::new();
        let mut new_numbers: Vec<Number> = Vec::new();

        for num in numbers.iter_mut() {
            if !num.factors.is_empty() {
                collected.push(*num.factors.last().unwrap());
                num.factors.pop();
                if !num.factors.is_empty() {
                    new_numbers.push(num.clone());
                }
            }
        }

        collected.sort_by(|a, b| {
            b.prime.cmp(&a.prime).then(b.tiebreak.cmp(&a.tiebreak))
        });

        let mut result = vec![Number { factors: collected }];
        result.append(&mut new_numbers);
        *numbers = result;
    };

    for _ in 0..shuffle_rounds {
        do_shuffle(&mut numbers);
    }

    // Collect factor info
    struct FactorInfo {
        k_mod: i64,
        tiebreak: i32,
        prime: i32,
    }

    let mut finfo: Vec<FactorInfo> = Vec::new();
    let mut max_k_mod: i64 = 0;

    for (pos, num) in numbers.iter().enumerate() {
        for (j, f) in num.factors.iter().enumerate() {
            let period = (pos + num.factors.len() - j) as i64;
            let remaining = k - shuffle_rounds;
            let mut km = remaining % period;
            if km < 0 { km += period; }

            if km > max_k_mod { max_k_mod = km; }
            finfo.push(FactorInfo {
                k_mod: km,
                tiebreak: f.tiebreak,
                prime: f.prime,
            });
        }
    }

    finfo.sort_by_key(|f| f.k_mod);

    let max_tb = 15 * N + N + 1;
    let mut tb_to_numidx = vec![-1i32; max_tb];
    for (i, num) in numbers.iter().enumerate() {
        for f in &num.factors {
            tb_to_numidx[f.tiebreak as usize] = i as i32;
        }
    }

    let mut products = vec![0i64; numbers.len().max(1)];
    let mut fi_ptr = 0usize;

    for k_val in 0..=max_k_mod {
        while fi_ptr < finfo.len() && finfo[fi_ptr].k_mod == k_val {
            let tb = finfo[fi_ptr].tiebreak as usize;
            let prime = finfo[fi_ptr].prime as i64;
            let numidx = tb_to_numidx[tb];

            if numidx >= 0 {
                let ni = numidx as usize;
                if ni < products.len() {
                    if products[ni] == 0 {
                        products[ni] = prime;
                    } else {
                        products[ni] = products[ni] as i128 * prime as i128 % MOD as i128
                            as i64;
                    }
                }
            }
            fi_ptr += 1;
        }

        if fi_ptr >= finfo.len() { break; }

        if k_val < max_k_mod {
            do_shuffle(&mut numbers);

            for v in tb_to_numidx.iter_mut() { *v = -1; }
            for (i, num) in numbers.iter().enumerate() {
                for f in &num.factors {
                    tb_to_numidx[f.tiebreak as usize] = i as i32;
                }
            }
        }
    }

    let mut answer: i64 = 0;
    for &p in &products {
        if p > 0 {
            answer = (answer + p) % MOD;
        }
    }

    println!("{}", answer);
}
