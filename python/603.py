"""Project Euler Problem 603: Concatenation of Consecutive Primes — embedded C."""

import subprocess, tempfile, os

def solve():
    c_code = r'''
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>

typedef long long i64;
typedef __int128 i128;

#define MOD 1000000007LL
#define N_PRIMES 1000000
#define K_COPIES 1000000000000LL
#define SIEVE_LIMIT 16000000

i64 mod_inv(i64 a, i64 m) {
    i64 t = 0, new_t = 1, r = m, new_r = a % m;
    while (new_r != 0) {
        i64 q = r / new_r;
        i64 tmp;
        tmp = new_t; new_t = t - q * new_t; t = tmp;
        tmp = new_r; new_r = r - q * new_r; r = tmp;
    }
    if (t < 0) t += m;
    return t;
}

i64 power_mod(i64 base, i64 exp, i64 mod) {
    i64 result = 1;
    base %= mod;
    while (exp > 0) {
        if (exp & 1) result = (i128)result * base % mod;
        base = (i128)base * base % mod;
        exp >>= 1;
    }
    return result;
}

int main() {
    /* Sieve of Eratosthenes */
    char *is_prime = (char*)calloc(SIEVE_LIMIT + 1, 1);
    memset(is_prime, 1, SIEVE_LIMIT + 1);
    is_prime[0] = is_prime[1] = 0;
    for (int i = 2; (long long)i * i <= SIEVE_LIMIT; i++) {
        if (is_prime[i])
            for (int j = i * i; j <= SIEVE_LIMIT; j += i)
                is_prime[j] = 0;
    }

    /* Collect first N_PRIMES primes */
    int *primes = (int*)malloc(N_PRIMES * sizeof(int));
    int count = 0;
    for (int i = 2; i <= SIEVE_LIMIT && count < N_PRIMES; i++)
        if (is_prime[i]) primes[count++] = i;
    free(is_prime);

    /* Build digit string – first pass: compute total length */
    int total_len = 0;
    for (int i = 0; i < N_PRIMES; i++) {
        char buf[16];
        total_len += sprintf(buf, "%d", primes[i]);
    }

    char *P = (char*)malloc(total_len + 1);
    int pos = 0;
    for (int i = 0; i < N_PRIMES; i++)
        pos += sprintf(P + pos, "%d", primes[i]);
    free(primes);

    i64 L = total_len;
    i64 M = MOD;
    i64 B = 10;

    i64 layered_num = 0, num = 0, layered_sum = 0, sum_val = 0;
    for (i64 n = 0; n < L; n++) {
        int d = P[n] - '0';
        layered_num = (layered_num + (i128)(n % M) * d) % M;
        layered_sum = (layered_sum + (i128)(n % M) * d) % M;
        num = (num + d) % M;
        sum_val = (sum_val + d) % M;
        layered_num = (i128)layered_num * B % M;
        num = (i128)num * B % M;
    }
    free(P);

    i64 piece = power_mod(B, L, M);
    i64 all_pow = (power_mod(piece, K_COPIES, M) - 1 + M) % M;
    i64 inv_den = mod_inv((piece - 1 + M) % M, M);

    i64 K_mod = K_COPIES % M;
    i64 Km1_mod = (K_COPIES - 1) % M;
    i64 tr_km1 = (i128)Km1_mod * K_mod % M * mod_inv(2, M) % M;
    i64 L_mod = L % M;

    i64 term1 = (i128)all_pow * inv_den % M;
    term1 = (i128)term1 * layered_num % M;

    i64 inner1 = (i128)L_mod * all_pow % M;
    inner1 = (i128)inner1 * inv_den % M;
    i64 inner2 = (i128)K_mod * L_mod % M;
    i64 term2_inner = (all_pow + inner1 - inner2 % M + 2 * M) % M;
    i64 term2 = (i128)term2_inner * inv_den % M;
    term2 = (i128)term2 * num % M;

    i64 term3 = (i128)K_mod * (layered_sum % M) % M;

    i64 term4 = (i128)L_mod * tr_km1 % M;
    term4 = (term4 + K_mod) % M;
    term4 = (i128)term4 * sum_val % M;

    i64 res = (term1 + term2 - term3 - term4 + 2 * M) % M;
    res = (i128)res * mod_inv(B - 1, M) % M;

    printf("%lld\n", res);
    return 0;
}
'''
    with tempfile.NamedTemporaryFile(suffix='.c', delete=False) as f:
        f.write(c_code.encode())
        c_file = f.name
    exe = c_file[:-2]
    subprocess.run(['gcc', '-O3', '-o', exe, c_file, '-lm'], check=True, capture_output=True)
    result = subprocess.check_output([exe], timeout=280).decode().strip()
    os.unlink(c_file)
    os.unlink(exe)
    return int(result)

if __name__ == "__main__":
    print(solve())
