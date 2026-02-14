// Project Euler 039: Integer Right Triangles
// For which perimeter p <= 1000 is the number of right-triangle solutions maximised?

fn main() {
    const P: usize = 1000;
    let mut counts = [0u32; P + 1];

    for a in 1..P / 2 {
        for b in a..P - a {
            let c2 = a * a + b * b;
            let c = (c2 as f64).sqrt() as usize;
            // Check exact square
            if c * c == c2 {
                let perim = a + b + c;
                if perim <= P {
                    counts[perim] += 1;
                }
            }
        }
    }

    let best = (1..=P).max_by_key(|&i| counts[i]).unwrap();
    println!("{best}");
}
