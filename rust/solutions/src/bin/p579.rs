// Project Euler 579 - Lattice Points in Cubes
//
// Sum f(C) for all cubes with lattice point vertices, coordinates 0..N.
// Uses quaternion parameterization of primitive cubes.
// N = 5000, answer mod 10^9.

fn my_gcd(mut a: i32, mut b: i32) -> i32 {
    if a < 0 { a = -a; }
    if b < 0 { b = -b; }
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

fn isqrt_i(n: i32) -> i32 {
    let mut r = (n as f64).sqrt() as i32;
    while r * r > n {
        r -= 1;
    }
    while (r + 1) * (r + 1) <= n {
        r += 1;
    }
    r
}

fn main() {
    let n: i32 = 5000;
    let m: i64 = 1_000_000_000;

    let mut ans: i64 = 0;

    for a in 0..=isqrt_i(n) {
        for b in a..=isqrt_i(n - a * a) {
            for c in b..=isqrt_i(n - a * a - b * b) {
                for d in b..=isqrt_i(n - a * a - b * b - c * c) {
                    let l_sq = a * a + b * b + c * c + d * d;
                    if l_sq > n {
                        break;
                    }
                    if a + b + c + d == 0 {
                        continue;
                    }
                    if d < c && (a == 0 || a == b || b == d) {
                        continue;
                    }

                    // Compute axes
                    let axes: [[i32; 3]; 3] = [
                        [
                            a * a + b * b - c * c - d * d,
                            2 * (b * c + d * a),
                            2 * (b * d - c * a),
                        ],
                        [
                            2 * (b * c - d * a),
                            a * a - b * b + c * c - d * d,
                            2 * (c * d + b * a),
                        ],
                        [
                            2 * (b * d + c * a),
                            2 * (c * d - b * a),
                            a * a - b * b - c * c + d * d,
                        ],
                    ];

                    // Compute GCDs
                    let mut gcds = [0i32; 3];
                    for i in 0..3 {
                        gcds[i] = my_gcd(my_gcd(axes[i][0].abs(), axes[i][1].abs()), axes[i][2].abs());
                    }

                    if my_gcd(my_gcd(gcds[0], gcds[1]), gcds[2]) > 1 {
                        continue;
                    }

                    // Compute bounds
                    let mut mins = [3 * n; 3];
                    let mut maxs = [-3 * n; 3];
                    for subset in 0..8u32 {
                        let mut v = [0i32; 3];
                        for i in 0..3 {
                            if subset & (1 << i) != 0 {
                                for dim in 0..3 {
                                    v[dim] += axes[i][dim];
                                }
                            }
                        }
                        for dim in 0..3 {
                            if v[dim] < mins[dim] {
                                mins[dim] = v[dim];
                            }
                            if v[dim] > maxs[dim] {
                                maxs[dim] = v[dim];
                            }
                        }
                    }

                    // Compute symmetries
                    let mut num_symmetries: i64 = 24;
                    if a == 0 && (b == c || c == d) {
                        num_symmetries /= 2;
                    }
                    if b == c && (a == b || c == d) {
                        num_symmetries /= 3;
                    }
                    if b == 0 {
                        num_symmetries /= 4;
                    }

                    let d_sum = (gcds[0] + gcds[1] + gcds[2]) as i64;
                    let l = isqrt_i(l_sq) as i64;
                    for t in 1..=n as i64 {
                        let lt = l * t;
                        let num_points = lt * lt * lt + l * d_sum * t * t + d_sum * t + 1;
                        let mut num_cubes: i64 = 1;
                        for i in 0..3 {
                            let size = (maxs[i] - mins[i]) as i64 * t;
                            let nc = n as i64 - size + 1;
                            if nc <= 0 {
                                num_cubes = 0;
                                break;
                            }
                            num_cubes *= nc;
                        }
                        if num_cubes == 0 {
                            break;
                        }
                        ans = (ans + (num_points % m) * (num_cubes % m) % m * num_symmetries) % m;
                    }
                }
            }
        }
    }

    println!("{}", ans);
}
