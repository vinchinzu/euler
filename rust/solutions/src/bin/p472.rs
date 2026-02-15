// Project Euler 472: Comfortable Distance
fn main() {
    let n_val: i64 = 1_000_000_000_000;
    let m: i64 = 100_000_000;

    let tr = |n: i64| -> i64 {
        (n % m) * ((n + 1) % m) / 2 % m
    };

    let f: [i64; 9] = [1, 2, 2, 4, 3, 6, 2, 6, 3];
    let mut sumf: i64 = 0;
    let mut index: i64 = 0;

    for i in 0..9 {
        sumf = (sumf + f[i]) % m;
        index += 1;
        if index >= n_val {
            println!("{}", sumf);
            return;
        }
    }

    loop {
        let length = index / 4;
        sumf = (sumf + 8) % m;
        index += 1;
        if index >= n_val {
            println!("{}", sumf);
            return;
        }

        let mut l: i64 = 1;
        while l <= length / 2 {
            // Rising range
            let mut count = n_val - index;
            if count > l { count = l; }
            sumf = (sumf + 2 * tr(count) % m) % m;
            index += l;
            if index >= n_val {
                println!("{}", sumf);
                return;
            }

            // Peak
            sumf = (sumf + 2 * ((2 * l + 1) % m)) % m;
            index += 1;
            if index >= n_val {
                println!("{}", sumf);
                return;
            }

            // Falling range
            let mut count2 = l - (n_val - index);
            if count2 < 1 { count2 = 1; }
            sumf = (sumf + 2 * ((tr(l) - tr(count2) + m) % m)) % m;
            index += l - 1;
            if index >= n_val {
                println!("{}", sumf);
                return;
            }

            l *= 2;
        }

        // Center rising range
        let mut count = n_val - index;
        if count > length { count = length; }
        sumf = (sumf + 2 * tr(count) % m) % m;
        index += length;
        if index >= n_val {
            println!("{}", sumf);
            return;
        }

        // Center peak
        sumf = (sumf + 3 * ((length + 1) % m)) % m;
        index += 1;
        if index >= n_val {
            println!("{}", sumf);
            return;
        }

        // Center falling range
        let mut count2 = length - (n_val - index) + 2;
        if count2 < 2 { count2 = 2; }
        sumf = (sumf + (tr(length + 2) - tr(count2) + m) % m) % m;
        index += length;
        if index >= n_val {
            println!("{}", sumf);
            return;
        }
    }
}
