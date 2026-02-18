// Verify doubling formulas for binary partitions
// S_0(m) = p(m), S_k(m) = sum_{j=0}^m S_{k-1}(j)

fn main() {
    let limit = 200;
    let mut p = vec![0u64; limit + 1];
    p[0] = 1;
    for n in 1..=limit {
        if n % 2 == 1 {
            p[n] = p[n - 1];
        } else {
            p[n] = p[n - 1] + p[n / 2];
        }
    }

    // Compute S_k for k = 0..5
    let k_max = 6;
    let mut s = vec![vec![0u64; limit + 1]; k_max + 1];
    for n in 0..=limit {
        s[0][n] = p[n];
    }
    for k in 1..=k_max {
        let mut cumsum = 0u64;
        for n in 0..=limit {
            cumsum += s[k - 1][n];
            s[k][n] = cumsum;
        }
    }

    // Verify: S_0(2m) = S_1(m)
    for m in 0..=20 {
        assert_eq!(s[0][2 * m], s[1][m], "S_0(2m) != S_1(m) for m={}", m);
    }
    println!("S_0(2m) = S_1(m) verified");

    // Verify: S_1(2m) = 2*S_2(m) - S_1(m)
    for m in 0..=20 {
        assert_eq!(s[1][2 * m], 2 * s[2][m] - s[1][m], "S_1 formula failed for m={}", m);
    }
    println!("S_1(2m) = 2*S_2(m) - S_1(m) verified");

    // Verify: S_2(2m) = 4*S_3(m) - 3*S_2(m)
    for m in 0..=20 {
        let lhs = s[2][2 * m];
        let rhs = 4 * s[3][m] - 3 * s[2][m];
        assert_eq!(lhs, rhs, "S_2 formula failed for m={}: {} != {}", m, lhs, rhs);
    }
    println!("S_2(2m) = 4*S_3(m) - 3*S_2(m) verified");

    // Try to find pattern for S_k(2m) in terms of S_{k+1}(m) and S_k(m)
    // S_0(2m) = 1*S_1(m) - 0*S_0(m) -> coeff (1, 0)
    // S_1(2m) = 2*S_2(m) - 1*S_1(m) -> coeff (2, -1)
    // S_2(2m) = 4*S_3(m) - 3*S_2(m) -> coeff (4, -3)
    // Guess: S_k(2m) = 2^{k+1} * S_{k+1}(m) - (2^{k+1}-1) * S_k(m)?
    // k=0: 2*S_1(m) - 1*S_0(m)?  = 2*S_1(m) - S_0(m)
    // But we showed S_0(2m) = S_1(m). So 2*S_1(m) - S_0(m) should equal S_1(m)
    // -> S_1(m) = S_0(m)? No, that's wrong.

    // Let me check: S_0(2m) = S_1(m). For m=2: S_0(4) = p(4) = 4, S_1(2) = p(0)+p(1)+p(2) = 1+1+2=4. OK.
    // 2*S_1(2) - S_0(2) = 2*4 - 2 = 6 != 4. So the pattern S_k(2m) = 2^{k+1}*S_{k+1}(m) - (2^{k+1}-1)*S_k(m) doesn't hold for k=0.

    // Actual pattern:
    // k=0: S_0(2m) = 1*S_1(m) + 0*S_0(m)
    // k=1: S_1(2m) = 2*S_2(m) - 1*S_1(m)
    // k=2: S_2(2m) = 4*S_3(m) - 3*S_2(m)
    // Hypothesis: coefficients are (2^k, -(2^k - 1)) for k >= 0?
    // k=0: (1, 0) -> yes, 2^0=1, -(2^0-1)=0
    // k=1: (2, -1) -> yes, 2^1=2, -(2^1-1)=-1
    // k=2: (4, -3) -> yes, 2^2=4, -(2^2-1)=-3
    // Let's check k=3: should be S_3(2m) = 8*S_4(m) - 7*S_3(m)

    for m in 0..=20 {
        let lhs = s[3][2 * m];
        let rhs = 8 * s[4][m] - 7 * s[3][m];
        assert_eq!(lhs, rhs, "S_3 formula failed for m={}", m);
    }
    println!("S_3(2m) = 8*S_4(m) - 7*S_3(m) verified");

    for m in 0..=20 {
        let lhs = s[4][2 * m];
        let rhs = 16 * s[5][m] - 15 * s[4][m];
        assert_eq!(lhs, rhs, "S_4 formula failed for m={}", m);
    }
    println!("S_4(2m) = 16*S_5(m) - 15*S_4(m) verified");

    // Verify add-1 formulas:
    // S_0(2m+1) = S_0(2m) [since p is constant on odd]
    // S_k(2m+1) = S_k(2m) + S_{k-1}(2m+1)
    for m in 0..=20 {
        assert_eq!(s[0][2 * m + 1], s[0][2 * m]);
        for k in 1..=k_max {
            if 2 * m + 1 <= limit {
                assert_eq!(s[k][2 * m + 1], s[k][2 * m] + s[k - 1][2 * m + 1]);
            }
        }
    }
    println!("Add-1 formulas verified");

    println!("\nGreat! The general formula is: S_k(2m) = 2^k * S_{{k+1}}(m) - (2^k - 1) * S_k(m)");
}
