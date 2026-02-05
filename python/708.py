"""Project Euler Problem 708: Twos Are All You Need."""

import subprocess
import tempfile
import os

def solve():
    c_code = r'''
#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <math.h>

typedef long long ll;
typedef unsigned long long ull;
typedef __int128 lll;

/*
 * Sum of τ(k) for k = 1 to n equals Σ_{d=1}^n floor(n/d)
 * This can be computed in O(sqrt(n)) time using floor quotient grouping.
 */
ll sum_floor_quotients(ll n) {
    ll result = 0;
    ll i = 1;
    while (i <= n) {
        ll q = n / i;
        ll r = n / q;
        result += q * (r - i + 1);
        i = r + 1;
    }
    return result;
}

ll N;
int L;
int *primes;
int num_primes;
ll *sum_fq_arr; // precomputed sum_floor_quotients for small values

void sieve(int limit) {
    char *is_prime = calloc(limit + 1, 1);
    primes = malloc((limit / 5 + 1) * sizeof(int));
    num_primes = 0;

    for (int i = 2; i <= limit; i++) {
        if (!is_prime[i]) {
            primes[num_primes++] = i;
            if ((ll)i * i <= limit) {
                for (int j = i * i; j <= limit; j += i)
                    is_prime[j] = 1;
            }
        }
    }
    free(is_prime);
}

ll ans = 0;

void helper(int min_index, ll d, ll mult) {
    ll q = N / d;
    ll sum_val;
    if (q <= L) {
        sum_val = sum_fq_arr[q];
    } else {
        sum_val = sum_floor_quotients(q);
    }
    ans += sum_val * mult;

    for (int index = min_index; index < num_primes; index++) {
        ll p = primes[index];
        if ((double)d * p * p > (double)N)
            break;
        ll new_d = d * p;
        int e = 2;
        while ((double)new_d * p <= (double)N) {
            new_d *= p;
            helper(index + 1, new_d, mult << (e - 2));
            e++;
        }
    }
}

int main() {
    N = 100000000000000LL;  // 10^14
    L = (int)sqrt((double)N) + 1;
    while ((ll)L * L > N) L--;

    sieve(L);

    // Precompute num_divisors for small values
    int *num_div = calloc(L + 1, sizeof(int));
    for (int i = 1; i <= L; i++) {
        for (int j = i; j <= L; j += i) {
            num_div[j]++;
        }
    }

    // Precompute cumulative sum
    sum_fq_arr = malloc((L + 1) * sizeof(ll));
    sum_fq_arr[0] = 0;
    for (int i = 1; i <= L; i++) {
        sum_fq_arr[i] = sum_fq_arr[i - 1] + num_div[i];
    }
    free(num_div);

    helper(0, 1, 1);

    printf("%lld\n", ans);

    free(primes);
    free(sum_fq_arr);
    return 0;
}
'''
    with tempfile.NamedTemporaryFile(suffix='.c', delete=False) as f:
        f.write(c_code.encode())
        c_file = f.name
    exe = c_file[:-2]
    subprocess.run(['gcc', '-O3', '-march=native', '-lm', '-o', exe, c_file], check=True)
    result = subprocess.check_output([exe]).decode().strip()
    os.unlink(c_file)
    os.unlink(exe)
    print(result)

if __name__ == "__main__":
    solve()
