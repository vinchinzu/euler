// Project Euler 692 - Siegbert and Jo (Fibonacci Nim)
// G(N) = sum H(k) for k=1..N, recurrence over Fibonacci numbers.

fn main() {
    let big_n: i64 = 23_416_728_348_467_685;

    // Build Fibonacci numbers
    let mut fibs = vec![0i64; 100];
    fibs[1] = 1; fibs[2] = 1;
    let mut cnt = 2;
    for i in 3..100 {
        fibs[i] = fibs[i - 1] + fibs[i - 2];
        cnt = i;
        if fibs[i] >= big_n { break; }
    }

    // a, b = G values at consecutive Fibonacci numbers
    let mut a = 1i64;
    let mut b = 1i64;
    let mut i = 2;
    loop {
        // Compute fib(i) inline
        let mut f_a = 1i64;
        let mut f_b = 1i64;
        for _ in 2..i {
            let f_c = f_a + f_b;
            f_a = f_b;
            f_b = f_c;
        }
        if f_b >= big_n { break; }
        let c = a + b + f_b;
        a = b;
        b = c;
        i += 1;
    }

    println!("{}", b);
}
