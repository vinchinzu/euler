/* Project Euler 725: Digit Sum Numbers.
 * S(N) = (B^N-1)/(B-1) * (N-1) * (2*C(B+N-2, B-2) - C(B,2)) mod 10^16
 * N=2020, B=10, M=10^16
 *
 * All arithmetic is done modulo M = 10^16 using __int128 to avoid overflow.
 * We need modular inverse of 9 mod M. Since gcd(9, 10^16) != 1, we can't
 * directly invert. Instead compute (10^2020 - 1)/9 mod 10^16 directly.
 *
 * Note: (10^n - 1)/9 = 1 + 10 + 100 + ... + 10^(n-1) = repunit(n).
 * C(B+N-2, B-2) = C(2026, 8) = 2026! / (8! * 2018!)
 * C(10, 2) = 45
 */
#include <stdio.h>
#include <stdint.h>

typedef unsigned long long ull;
typedef __int128 u128;

#define M 10000000000000000ULL  /* 10^16 */

ull mulmod(ull a, ull b, ull m) {
    return (u128)a * b % m;
}

ull powmod(ull base, ull exp, ull m) {
    ull result = 1;
    base %= m;
    while (exp > 0) {
        if (exp & 1) result = mulmod(result, base, m);
        base = mulmod(base, base, m);
        exp >>= 1;
    }
    return result;
}

int main() {
    int N = 2020;
    int B = 10;

    /* term1 = (B^N - 1) / (B - 1) mod M */
    /* = repunit(N) = sum_{i=0}^{N-1} 10^i mod M */
    /* Since N=2020 > 16, for mod 10^16 this is just sum of 10^i for i=0..15 + 0 for i>=16 */
    /* Wait, 10^16 mod 10^16 = 0, so repunit(2020) mod 10^16 = repunit(16) */
    /* repunit(16) = (10^16 - 1)/9 = 1111111111111111 */
    ull term1 = 0;
    ull pw = 1;
    for (int i = 0; i < N && i < 16; i++) {
        term1 = (term1 + pw) % M;
        pw = mulmod(pw, 10, M);
    }
    /* For i >= 16, 10^i mod 10^16 = 0, so no more contribution */

    /* term2 = N - 1 = 2019 */
    ull term2 = N - 1;

    /* term3 = 2 * C(B+N-2, B-2) - C(B, 2) */
    /* C(2026, 8) and C(10, 2) = 45 */
    /* C(2026, 8) = 2026 * 2025 * 2024 * 2023 * 2022 * 2021 * 2020 * 2019 / 8! */
    /* 8! = 40320 */
    /* We need this mod M. Since M = 10^16 and gcd(8!, M) != 1 (8! has factors of 2 and 5),
     * we compute C(2026,8) exactly (it fits in ~64 bits or so, let's check).
     * Actually 2026^8 ~ 2^88, way too big for 64 bits. Use __int128.
     * C(2026,8) = product / 40320. Let's compute step by step dividing as we go. */
    /* Actually C(n,k) for k=8, n=2026: each step multiply by (n-i) and divide by (i+1) */
    /* C(B+N-2, B-2) = C(2028, 8) */
    u128 comb = 1;
    for (int i = 0; i < 8; i++) {
        comb = comb * (2028 - i) / (i + 1);
    }
    /* comb = C(2028, 8) -- exact since binomial coefficients are integers */
    ull comb_mod = (ull)(comb % M);

    ull c10_2 = 45;  /* C(10, 2) */
    ull term3 = (mulmod(2, comb_mod, M) - c10_2 + M) % M;

    ull ans = mulmod(mulmod(term1, term2, M), term3, M);
    printf("%llu\n", ans);
    return 0;
}
