"""Project Euler Problem 636: Restricted Factorisations.

F(n) = number of ways to write n = a * b1^2 * b2^2 * c1^3 * c2^3 * c3^3 * d1^4 * d2^4 * d3^4 * d4^4
with all 10 bases distinct. Find F(10^6!) mod 10^9+7.

Approach: inclusion-exclusion over partitions of the 10 positions into groups
where same-group positions share the same base. Each partition type gives a
"jump profile" (multiset of coin values for unbounded knapsack), and we compute
the product over all primes of the coin-change count for that prime's exponent.
"""

import subprocess, tempfile, os
from collections import Counter
from math import factorial
from sympy import primerange


def solve():
    N = 10**6
    MOD = 10**9 + 7

    # Enumerate group compositions: (n1, n2, n3, n4) = how many of type 1,2,3,4
    group_comps = []
    for n1 in range(2):
        for n2 in range(3):
            for n3 in range(4):
                for n4 in range(5):
                    if n1 + n2 + n3 + n4 > 0:
                        group_comps.append((n1, n2, n3, n4))

    def enum_parts(remaining, min_idx, current):
        n1_r, n2_r, n3_r, n4_r = remaining
        if n1_r == 0 and n2_r == 0 and n3_r == 0 and n4_r == 0:
            yield tuple(sorted(current))
            return
        for idx in range(min_idx, len(group_comps)):
            gc = group_comps[idx]
            if gc[0] <= n1_r and gc[1] <= n2_r and gc[2] <= n3_r and gc[3] <= n4_r:
                new_rem = (n1_r - gc[0], n2_r - gc[1], n3_r - gc[2], n4_r - gc[3])
                yield from enum_parts(new_rem, idx, current + [gc])

    parts = set()
    for p in enum_parts((1, 2, 3, 4), 0, []):
        parts.add(p)

    def multinomial(n, groups):
        r = factorial(n)
        for g in groups:
            r //= factorial(g)
        return r

    jump_profiles = {}
    for part in parts:
        jumps = tuple(sorted([g[0] + 2*g[1] + 3*g[2] + 4*g[3] for g in part]))
        m = len(part)
        sign = (-1) ** (10 - m)
        bf = 1
        for g in part:
            bf *= factorial(sum(g) - 1)
        ways = 1
        for t, tot in enumerate([1, 2, 3, 4]):
            ways *= multinomial(tot, [g[t] for g in part])
        bc = Counter(part)
        for v in bc.values():
            ways //= factorial(v)
        coeff = sign * bf * ways
        jump_profiles[jumps] = jump_profiles.get(jumps, 0) + coeff

    jump_profiles = {k: v for k, v in jump_profiles.items() if v != 0}

    # Compute prime exponents in N!
    primes = list(primerange(2, N + 1))
    exp_counts = Counter()
    for p in primes:
        e = 0
        pk = p
        while pk <= N:
            e += N // pk
            if pk > N // p:
                break
            pk *= p
        exp_counts[e] += 1

    max_e = max(exp_counts.keys())
    distinct_exps = sorted(exp_counts.keys())
    exp_multiplicities = [exp_counts[e] for e in distinct_exps]

    # Prepare data for C
    profiles = sorted(jump_profiles.items())

    # Build input for C program
    lines = []
    lines.append(f"{len(profiles)} {len(distinct_exps)} {max_e} {MOD}")
    for jumps, coeff in profiles:
        lines.append(f"{coeff} {len(jumps)} " + " ".join(str(j) for j in jumps))
    for e in distinct_exps:
        lines.append(str(e))
    for m in exp_multiplicities:
        lines.append(str(m))

    input_data = "\n".join(lines) + "\n"

    c_code = r"""
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef long long ll;
typedef unsigned int u32;

ll power(ll base, ll exp, ll mod) {
    ll r = 1;
    base %= mod;
    if (base < 0) base += mod;
    while (exp > 0) {
        if (exp & 1) r = r * base % mod;
        base = base * base % mod;
        exp >>= 1;
    }
    return r;
}

int main() {
    int num_profiles, num_exps, max_e;
    ll MOD;
    scanf("%d %d %d %lld", &num_profiles, &num_exps, &max_e, &MOD);
    u32 M = (u32)MOD;

    /* Read profiles */
    ll *coeffs = (ll *)malloc(num_profiles * sizeof(ll));
    int *num_coins = (int *)malloc(num_profiles * sizeof(int));
    int **coins = (int **)malloc(num_profiles * sizeof(int *));

    for (int i = 0; i < num_profiles; i++) {
        scanf("%lld %d", &coeffs[i], &num_coins[i]);
        coins[i] = (int *)malloc(num_coins[i] * sizeof(int));
        for (int j = 0; j < num_coins[i]; j++) {
            scanf("%d", &coins[i][j]);
        }
    }

    /* Read distinct exponents and multiplicities */
    int *exps = (int *)malloc(num_exps * sizeof(int));
    int *mults = (int *)malloc(num_exps * sizeof(int));
    for (int i = 0; i < num_exps; i++) scanf("%d", &exps[i]);
    for (int i = 0; i < num_exps; i++) scanf("%d", &mults[i]);

    /* For each profile, compute coin-change DP and evaluate product */
    /* Use u32 for dp to reduce memory bandwidth (halves cache pressure) */
    u32 *dp = (u32 *)malloc((max_e + 1) * sizeof(u32));
    ll answer = 0;

    for (int pi = 0; pi < num_profiles; pi++) {
        /* Initialize DP */
        memset(dp, 0, (max_e + 1) * sizeof(u32));
        dp[0] = 1;

        for (int ci = 0; ci < num_coins[pi]; ci++) {
            int c = coins[pi][ci];
            for (int i = c; i <= max_e; i++) {
                u32 v = dp[i] + dp[i - c];
                dp[i] = v >= M ? v - M : v;
            }
        }

        /* Compute product over distinct exponents */
        ll prod = 1;
        for (int ei = 0; ei < num_exps; ei++) {
            ll val = dp[exps[ei]];
            prod = prod % MOD * power(val, mults[ei], MOD) % MOD;
        }

        /* Add coeff * prod to answer */
        ll c = coeffs[pi] % MOD;
        if (c < 0) c += MOD;
        answer = (answer + c * prod) % MOD;
    }

    /* Divide by 1!*2!*3!*4! = 288 */
    ll div_val = 1;
    for (int i = 1; i <= 4; i++) {
        ll f = 1;
        for (int j = 2; j <= i; j++) f *= j;
        div_val *= f;
    }
    answer = answer % MOD * power(div_val, MOD - 2, MOD) % MOD;

    printf("%lld\n", answer);

    free(dp);
    for (int i = 0; i < num_profiles; i++) free(coins[i]);
    free(coins); free(num_coins); free(coeffs);
    free(exps); free(mults);
    return 0;
}
"""

    with tempfile.TemporaryDirectory() as tmpdir:
        src = os.path.join(tmpdir, "sol.c")
        exe = os.path.join(tmpdir, "sol")
        with open(src, "w") as f:
            f.write(c_code)
        r = subprocess.run(["gcc", "-O2", "-o", exe, src], capture_output=True, text=True)
        if r.returncode != 0:
            import sys; sys.stderr.write("Compile: " + r.stderr + "\n"); return -1
        result = subprocess.run([exe], input=input_data, capture_output=True, text=True, timeout=28)
        if result.returncode != 0:
            import sys; sys.stderr.write("Runtime: " + result.stderr + "\n"); return -1
        return int(result.stdout.strip())


if __name__ == "__main__":
    print(solve())
