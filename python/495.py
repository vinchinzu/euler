"""Project Euler Problem 495: Writing n! as product of k distinct integers.

Find the number of ways that N! can be written as the product of K distinct
positive integers.

Uses inclusion-exclusion over partitions of K, with embedded C for performance.
Algorithm:
  1. Sieve primes up to N=10000, compute exponents of each prime in 10000!
  2. Enumerate all partitions of K=30 (5604 partitions) via DFS
  3. For each partition, build DP array incrementally (coin change style)
     f(c)[e] = number of ways to write e as c[0]*a_0 + c[1]*a_1 + ...
  4. Product of f(c)[exponent_of_p] over all primes p, with sign/symmetry factors
  5. Sum over all partitions
"""
import subprocess, os, tempfile


def solve():
    c_code = r"""
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define N 10000
#define K 30
#define MOD 1000000007LL

static long long dp[K + 1][N + 1];  /* dp[depth][e] */
static int primes[1300];
static int exponents[1300];
static int nprimes;
static long long inv_val[K + 1];    /* modular inverse of i */
static long long inv_fact[K + 1];   /* inverse factorial of i */
static long long ans;

/* Sieve primes up to N */
static void sieve(void) {
    char is_prime[N + 1];
    memset(is_prime, 1, sizeof(is_prime));
    is_prime[0] = is_prime[1] = 0;
    int i, j;
    for (i = 2; (long long)i * i <= N; i++) {
        if (is_prime[i]) {
            for (j = i * i; j <= N; j += i)
                is_prime[j] = 0;
        }
    }
    nprimes = 0;
    for (i = 2; i <= N; i++) {
        if (is_prime[i])
            primes[nprimes++] = i;
    }
}

/* Count factors of p in N! */
static int vp_factorial(int n, int p) {
    int count = 0;
    long long pw = p;
    while (pw <= n) {
        count += n / (int)pw;
        pw *= p;
    }
    return count;
}

/* Precompute modular inverses and inverse factorials */
static void precompute(void) {
    inv_val[0] = 0;
    inv_val[1] = 1;
    int i;
    for (i = 2; i <= K; i++) {
        inv_val[i] = (MOD - MOD / i) * inv_val[MOD % i] % MOD;
    }
    long long fact = 1;
    for (i = 1; i <= K; i++) fact = fact * i % MOD;
    inv_fact[K] = 1;
    /* inv_fact[K] = modular inverse of K! */
    {
        long long base = fact, exp = MOD - 2, result = 1;
        while (exp > 0) {
            if (exp & 1) result = result * base % MOD;
            base = base * base % MOD;
            exp >>= 1;
        }
        inv_fact[K] = result;
    }
    for (i = K - 1; i >= 0; i--) {
        inv_fact[i] = inv_fact[i + 1] * (i + 1) % MOD;
    }
}

/*
 * DFS over partitions of K.
 * coins[0..depth-1] is the current partition prefix.
 * dp[depth] holds the DP array for this prefix.
 * When remaining==0, evaluate the partition.
 */
static int coins[K + 1];
static int freq[K + 1];  /* frequency of each coin value in current partition */

static void helper(int min_val, int remaining, int depth) {
    if (remaining == 0) {
        /* Evaluate this partition */
        long long res = 1;
        int i;

        /* Product of dp[depth][exponent_of_p] over all primes */
        for (i = 0; i < nprimes; i++) {
            res = res * dp[depth][exponents[i]] % MOD;
        }

        /* Multiply by (-1)^(c_i+1) * inv(c_i) for each coin c_i */
        for (i = 0; i < depth; i++) {
            int c = coins[i];
            /* (-1)^(c+1): positive when c+1 even (c odd), negative when c+1 odd (c even) */
            if (c % 2 == 0) {
                /* sign is -1, i.e., MOD-1 */
                res = res * (MOD - 1) % MOD;
            }
            res = res * inv_val[c] % MOD;
        }

        /* Multiply by inv_fact of frequency of each distinct coin value */
        /* Count frequencies from coins array */
        /* Since coins are non-decreasing, we can count runs */
        i = 0;
        while (i < depth) {
            int j = i;
            while (j < depth && coins[j] == coins[i]) j++;
            int cnt = j - i;
            res = res * inv_fact[cnt] % MOD;
            i = j;
        }

        ans = (ans + res) % MOD;
        return;
    }

    int coeff;
    for (coeff = min_val; coeff <= remaining; coeff++) {
        /* Build dp[depth+1] from dp[depth] by adding coin of value coeff */
        /* Copy dp[depth] to dp[depth+1], then do prefix sum with step coeff */
        /* But we're going to use dp[depth] as the source for all coeff values
           at this level, so dp[depth] must not be modified. */

        /* We store result in dp[depth+1] since depth+1 is the new depth */
        int max_exp = N;  /* dp arrays go up to index N */
        int e;

        /* Copy parent DP */
        memcpy(dp[depth + 1], dp[depth], (max_exp + 1) * sizeof(long long));

        /* Coin change: for e from coeff to max_exp, dp[depth+1][e] += dp[depth+1][e-coeff] */
        for (e = coeff; e <= max_exp; e++) {
            dp[depth + 1][e] = (dp[depth + 1][e] + dp[depth + 1][e - coeff]) % MOD;
        }

        coins[depth] = coeff;
        helper(coeff, remaining - coeff, depth + 1);
    }
}

int main(void) {
    sieve();
    precompute();

    int i;
    for (i = 0; i < nprimes; i++) {
        exponents[i] = vp_factorial(N, primes[i]);
    }

    /* Base case: dp[0] = [1, 0, 0, ...] */
    memset(dp, 0, sizeof(dp));
    dp[0][0] = 1;

    ans = 0;
    helper(1, K, 0);

    printf("%lld\n", ans);
    return 0;
}
"""
    tmpdir = tempfile.mkdtemp()
    src = os.path.join(tmpdir, "sol495.c")
    exe = os.path.join(tmpdir, "sol495")
    with open(src, 'w') as f:
        f.write(c_code)
    subprocess.run(["gcc", "-O2", "-o", exe, src, "-lm"], check=True, capture_output=True)
    result = subprocess.run([exe], capture_output=True, text=True, check=True, timeout=280)
    print(result.stdout.strip())


if __name__ == "__main__":
    solve()
