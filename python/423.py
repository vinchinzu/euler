"""Project Euler Problem 423: Consecutive die throws.

C(n) = number of outcomes of throwing a 6-sided die n times such that the
number of consecutive identical pairs does not exceed pi(n).

C(n) = K * sum_{c=0}^{pi(n)} C(n-1, c) * (K-1)^(n-1-c)

Using recurrence on f (where C(n) = K*f) and R = C(n-1, pi(n)) * (K-1)^(n-1-pi(n)):
  Non-prime n: f_new = K*f - R; R_new = R*(n-1)*(K-1)/(n-1-pi)
  Prime n:     R_new = R*(n-1)/(pi+1); f_new = K*f - R + R_new

Uses embedded C for performance since N=50,000,000.
"""
import subprocess, os, tempfile


def solve():
    c_code = r"""
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

#define N 50000000
#define K 6
#define MOD 1000000007LL

static char is_prime_arr[N + 1];
static long long inv_arr[N + 1];

int main() {
    int i;

    /* Sieve of Eratosthenes */
    memset(is_prime_arr, 1, sizeof(is_prime_arr));
    is_prime_arr[0] = is_prime_arr[1] = 0;
    for (i = 2; (long long)i * i <= N; i++) {
        if (is_prime_arr[i]) {
            for (int j = i * i; j <= N; j += i)
                is_prime_arr[j] = 0;
        }
    }

    /* Modular inverses */
    inv_arr[0] = 0;
    inv_arr[1] = 1;
    for (i = 2; i <= N; i++) {
        inv_arr[i] = (MOD - MOD / i * inv_arr[MOD % i] % MOD) % MOD;
    }

    /*
     * f = sum_{c=0}^{pi(n)} C(n-1,c) * (K-1)^(n-1-c), so C(n) = K * f
     * R = C(n-1, pi(n)) * (K-1)^(n-1-pi(n))
     *
     * Initial (n=1): pi(1)=0, f=1, R=1, C(1)=6.
     */
    long long f = 1;
    long long R = 1;
    int pi_n = 0;

    long long ans = (K * f) % MOD;  /* C(1) = 6 */

    for (int n = 2; n <= N; n++) {
        if (is_prime_arr[n]) {
            /* pi(n) = pi(n-1) + 1 */
            /* R_new = R * (n-1) / (pi_n + 1) */
            long long R_new = R * ((n - 1) % MOD) % MOD * inv_arr[pi_n + 1] % MOD;
            /* f_new = K*f - R + R_new */
            f = ((long long)K * f % MOD - R % MOD + R_new + MOD) % MOD;
            R = R_new;
            pi_n++;
        } else {
            /* pi(n) = pi(n-1) */
            /* f_new = K*f - R */
            long long f_new = ((long long)K * f % MOD - R % MOD + MOD) % MOD;
            /* R_new = R * (n-1) * (K-1) / (n-1-pi_n) */
            long long R_new;
            if (n - 1 > pi_n) {
                R_new = R * ((n - 1) % MOD) % MOD * (K - 1) % MOD * inv_arr[n - 1 - pi_n] % MOD;
            } else {
                /* n-1 == pi_n: R = C(n-1, n-1) * 5^0 = 1 */
                R_new = 1;
            }
            f = f_new;
            R = R_new;
        }

        long long C_n = (long long)K * f % MOD;
        ans = (ans + C_n) % MOD;
    }

    printf("%lld\n", ans);
    return 0;
}
"""
    tmpdir = tempfile.mkdtemp()
    src = os.path.join(tmpdir, "sol423.c")
    exe = os.path.join(tmpdir, "sol423")
    with open(src, 'w') as f:
        f.write(c_code)
    subprocess.run(["gcc", "-O2", "-o", exe, src, "-lm"], check=True, capture_output=True)
    result = subprocess.run([exe], capture_output=True, text=True, check=True, timeout=280)
    print(result.stdout.strip())


if __name__ == "__main__":
    solve()
