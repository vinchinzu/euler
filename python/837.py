"""Project Euler Problem 837: Amidakuji."""

import subprocess
import tempfile
import os

def solve():
    c_code = r'''
#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>

typedef __int128 i128;
typedef int64_t i64;

#define A 123456789
#define B 987654321
#define M 1234567891LL

i64 mod_pow(i64 base, i64 exp, i64 m) {
    i128 result = 1;
    i128 b = base % m;
    while (exp > 0) {
        if (exp & 1) result = (result * b) % m;
        b = (b * b) % m;
        exp >>= 1;
    }
    return (i64)result;
}

i64 mod_inv(i64 a, i64 m) {
    return mod_pow(a, m - 2, m);
}

int main() {
    // Precompute modular inverses 1..A
    i64 *mod_invs = malloc((A + 1) * sizeof(i64));
    mod_invs[1] = 1;
    for (int i = 2; i <= A; i++) {
        mod_invs[i] = (M - (M / i) * mod_invs[M % i] % M) % M;
    }

    // Compute factorial((A-1)/2) and factorial((B-1)/2)
    i64 fact_a = 1;
    for (i64 i = 2; i <= (A - 1) / 2; i++) {
        fact_a = (i128)fact_a * i % M;
    }
    i64 fact_b = 1;
    for (i64 i = 2; i <= (B - 1) / 2; i++) {
        fact_b = (i128)fact_b * i % M;
    }

    i64 term1 = mod_inv((i128)fact_a * fact_b % M, M);
    i64 term2 = 0;
    i64 ans = 0;

    for (int t = 3; t <= A; t += 2) {
        term1 = (i128)term1 * mod_invs[t - 1] % M;
        term1 = (i128)term1 * mod_invs[t] % M;
        term1 = (i128)term1 * ((A - t + 2) / 2) % M;
        term1 = (i128)term1 * ((B - t + 2) / 2) % M;

        term2 = (4 * term2 + 2) % M;

        ans = (ans + (i128)term1 * term2) % M;
    }

    // Multiply by ((A+B)/2)!
    i64 fact_total = 1;
    for (i64 i = 2; i <= (A + B) / 2; i++) {
        fact_total = (i128)fact_total * i % M;
    }

    ans = (i128)ans * fact_total % M;
    printf("%lld\n", (long long)ans);

    free(mod_invs);
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
