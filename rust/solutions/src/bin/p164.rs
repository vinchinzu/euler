fn main() {
    let n = 20;
    let mut prev = [[0i64; 10]; 10];

    for d1 in 1..=9 {
        for d2 in 0..=9 {
            prev[d1][d2] = 1;
        }
    }

    for _pos in 3..=n {
        let mut cur = [[0i64; 10]; 10];
        for d1 in 0..=9 {
            for d2 in 0..=9 {
                if prev[d1][d2] == 0 { continue; }
                let max_d3 = 9i32 - d1 as i32 - d2 as i32;
                if max_d3 < 0 { continue; }
                for d3 in 0..=(max_d3 as usize) {
                    cur[d2][d3] += prev[d1][d2];
                }
            }
        }
        prev = cur;
    }

    let mut total: i64 = 0;
    for i in 0..10 {
        for j in 0..10 {
            total += prev[i][j];
        }
    }
    println!("{}", total);
}
