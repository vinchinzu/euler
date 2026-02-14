// Project Euler 341: Golomb's self-describing sequence
fn main() {
    let big_n: i64 = 1_000_000;
    let l: usize = 15_848_932;

    let mut g = vec![0i32; l + 10];
    let mut size_g: usize = 1;
    g[1] = 1;
    let mut k: i32 = 1;

    while size_g < l {
        g[size_g] = k;
        size_g += 1;
        for _ in 1..g[k as usize] {
            if size_g >= l { break; }
            g[size_g] = k;
            size_g += 1;
        }
        k += 1;
    }

    let mut sum_g: i64 = 0;
    let mut sum_kg: i64 = 0;
    let mut ans: i64 = 0;
    let mut n: i64 = 1;

    for ki in 1..size_g {
        sum_g += g[ki] as i64;
        sum_kg += ki as i64 * g[ki] as i64;
        while n < big_n && n * n * n <= sum_kg {
            ans += sum_g - (sum_kg - n * n * n) / ki as i64;
            n += 1;
        }
        if n >= big_n { break; }
    }

    println!("{ans}");
}
