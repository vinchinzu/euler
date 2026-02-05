"""Project Euler Problem 817: Digits in Squares."""

import subprocess
import tempfile
import os

def solve():
    c_code = r'''
#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <math.h>

typedef __int128 i128;
typedef int64_t i64;

#define N 100000
#define P 1000000007LL

// Check if n is a quadratic residue mod p using Euler's criterion
int is_square_mod(i64 n, i64 p) {
    n = n % p;
    if (n == 0) return 1;
    // n^((p-1)/2) == 1 mod p iff n is QR
    i64 exp = (p - 1) / 2;
    i128 result = 1;
    i128 base = n;
    while (exp > 0) {
        if (exp & 1) result = (result * base) % p;
        base = (base * base) % p;
        exp >>= 1;
    }
    return result == 1;
}

// Tonelli-Shanks algorithm to find sqrt of n mod p
i64 sqrt_mod(i64 n, i64 p) {
    n = n % p;
    if (n == 0) return 0;

    // p = 10^9 + 7 = 1 mod 4, so we need Tonelli-Shanks
    // Actually p-1 = 10^9 + 6 = 2 * 500000003
    // So Q = 500000003, S = 1
    i64 Q = (p - 1) / 2;
    int S = 1;

    // Find a quadratic non-residue z
    i64 z = 2;
    while (is_square_mod(z, p)) z++;

    i64 M = S;
    i128 c = 1;
    {
        i128 base = z;
        i64 exp = Q;
        while (exp > 0) {
            if (exp & 1) c = (c * base) % p;
            base = (base * base) % p;
            exp >>= 1;
        }
    }

    i128 t = 1;
    {
        i128 base = n;
        i64 exp = Q;
        while (exp > 0) {
            if (exp & 1) t = (t * base) % p;
            base = (base * base) % p;
            exp >>= 1;
        }
    }

    i128 R = 1;
    {
        i128 base = n;
        i64 exp = (Q + 1) / 2;
        while (exp > 0) {
            if (exp & 1) R = (R * base) % p;
            base = (base * base) % p;
            exp >>= 1;
        }
    }

    while (1) {
        if (t == 1) return (i64)R;

        // Find i such that t^(2^i) = 1
        int i = 0;
        i128 temp = t;
        while (temp != 1) {
            temp = (temp * temp) % p;
            i++;
        }

        // b = c^(2^(M-i-1))
        i128 b = c;
        for (int j = 0; j < M - i - 1; j++) {
            b = (b * b) % p;
        }

        M = i;
        c = (b * b) % p;
        t = (t * c) % p;
        R = (R * b) % p;
    }
}

// isqrt for i128
i64 isqrt128(i128 n) {
    if (n <= 1) return (i64)n;
    i64 x = (i64)sqrtl((long double)n);
    while ((i128)x * x > n) x--;
    while ((i128)(x+1) * (x+1) <= n) x++;
    return x;
}

// M(p, d) = smallest m such that m^2 in base p contains digit d
i64 M_func(i64 p, i64 d) {
    if (is_square_mod(d, p)) {
        i64 sq = sqrt_mod(d, p);
        return sq < p - sq ? sq : p - sq;
    }

    // Otherwise, check "tens" digit: find smallest m such that floor(m^2/p) mod p == d
    // m^2 in [h + d*p, h + (d+1)*p - 1] for some h = k*p^2
    for (i128 h = 0; ; h += (i128)p * p) {
        i128 low = h + (i128)d * p;
        i128 high = h + (i128)(d + 1) * p - 1;
        i64 sq_high = isqrt128(high);
        if ((i128)sq_high * sq_high >= low) {
            return sq_high;
        }
    }
}

int main() {
    i64 ans = 0;
    for (int d = 1; d <= N; d++) {
        ans += M_func(P, P - d);
    }
    printf("%lld\n", (long long)ans);
    return 0;
}
'''

    with tempfile.NamedTemporaryFile(suffix='.c', delete=False) as f:
        f.write(c_code.encode())
        c_file = f.name

    exe = c_file[:-2]
    subprocess.run(['gcc', '-O3', '-o', exe, c_file, '-lm'], check=True, capture_output=True)
    result = subprocess.check_output([exe]).decode().strip()
    os.unlink(c_file)
    os.unlink(exe)
    return int(result)

if __name__ == "__main__":
    print(solve())
