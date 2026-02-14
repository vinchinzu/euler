// Project Euler Problem 126: Cuboid layers
// N_m(x,y,z) = 2(xy+yz+zx) + 4(x+y+z)(m-1) + 4(m-1)(m-2)
// Find smallest n with exactly 1000 cuboid layer representations.

fn main() {
    const N_TARGET: usize = 1000;
    const N_LIMIT: usize = 20000;

    let mut counts = vec![0u32; N_LIMIT + 1];

    for m in 1.. {
        let m1 = m - 1i64;
        let term_fixed = 4 * m1 * (m as i64 - 2);
        let term_factor = 4 * m1;

        let min_cubes = 6 + 12 * m1 + term_fixed;
        if min_cubes > N_LIMIT as i64 {
            break;
        }

        for z in 1.. {
            let z = z as i64;
            let cubes_zzz = 6 * z * z + 12 * z * m1 + term_fixed;
            if cubes_zzz > N_LIMIT as i64 {
                break;
            }

            for y in z.. {
                let cubes_yyz = 2 * (y * y + 2 * y * z) + term_factor * (2 * y + z) + term_fixed;
                if cubes_yyz > N_LIMIT as i64 {
                    break;
                }

                for x in y.. {
                    let n_m = 2 * (x * y + y * z + z * x) + term_factor * (x + y + z) + term_fixed;
                    if n_m > N_LIMIT as i64 {
                        break;
                    }
                    counts[n_m as usize] += 1;
                }
            }
        }
    }

    for n in 1..=N_LIMIT {
        if counts[n] == N_TARGET as u32 {
            println!("{}", n);
            return;
        }
    }
}
