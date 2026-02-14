// Project Euler 260: Stone Game
fn main() {
    const N: usize = 1000;

    let mut lines = vec![vec![false; N + 1]; N + 1];
    let mut diags = vec![vec![false; N + 1]; N + 1];
    let mut space_arr = vec![vec![false; N + 1]; N + 1];

    let mut ans: i64 = 0;

    for x in 0..=N {
        for y in x..=N {
            for z in y..=N {
                if lines[x][y] || lines[x][z] || lines[y][z] { continue; }
                if diags[x][z - y] || diags[y][z - x] || diags[z][y - x] { continue; }
                if space_arr[y - x][z - y] { continue; }

                lines[x][y] = true;
                lines[x][z] = true;
                lines[y][z] = true;
                diags[x][z - y] = true;
                diags[y][z - x] = true;
                diags[z][y - x] = true;
                space_arr[y - x][z - y] = true;

                ans += (x + y + z) as i64;
            }
        }
    }

    println!("{}", ans);
}
