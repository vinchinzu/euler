fn t(n: i64) -> i64 {
    let n2 = n * n;
    let nm1_2 = (n - 1) * (n - 1);
    let mut correction: i64 = 0;
    let mut num_odd_rows: i64 = 0;
    let mut x: i64 = 0;
    let mut y: i64 = n - 1;
    let mut left_border = false;

    loop {
        if x * x + y * y < nm1_2 {
            x += 1;
        }

        let prev_x = x;

        while (x + 1) * (x + 1) + y * y < n2 {
            x += 1;
        }

        y -= 1;

        let right_border = x * x + y * y >= nm1_2;

        let width = x - prev_x + 1;
        let odd_parity = width % 2;

        num_odd_rows += (x - prev_x - 1)
            + if !left_border { 1 } else { 0 }
            + if !right_border { 1 } else { 0 }
            + odd_parity;

        correction += ((x - prev_x - 1)
            + if left_border { -1 } else { 1 }
            + if right_border { -1 } else { 1 })
            * if odd_parity == 1 { 2 } else { -2 };

        if y <= x {
            if y == x {
                if x * x + y * y >= nm1_2 {
                    correction += 1;
                }
            } else {
                correction -= 1;
                if !left_border && !right_border {
                    num_odd_rows -= 1;
                }
            }
            break;
        }

        left_border = right_border;
    }

    2 * num_odd_rows * (n - num_odd_rows) + correction
}

fn main() {
    let mut ans: i64 = 0;

    for i in 3..=31 {
        let n = (1i64 << i) - i;
        if n == 5 {
            ans += 3;
        } else if n % 2 == 0 {
            ans += t(n);
        }
    }

    println!("{}", ans);
}
