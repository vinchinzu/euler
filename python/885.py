"""Project Euler Problem 885.

Embedded C solution for speed. Recursive backtracking over sorted 18-digit
sequences in base 10, computing multinomial coefficients and number values mod M.
"""
import subprocess, tempfile, os, sys

C_CODE = r"""
#include <stdio.h>

typedef long long ll;
typedef unsigned long long ull;

#define N 18
#define B 10
#define M 1123455689LL

ll fact[N + 2];
ll inv_fact[N + 2];

ll power(ll base, ll exp, ll mod) {
    ll result = 1;
    base %= mod;
    if (base < 0) base += mod;
    while (exp > 0) {
        if (exp & 1) result = result * base % mod;
        base = base * base % mod;
        exp >>= 1;
    }
    return result;
}

void precompute(void) {
    fact[0] = 1;
    for (int i = 1; i <= N; i++)
        fact[i] = fact[i - 1] * i % M;
    inv_fact[N] = power(fact[N], M - 2, M);
    for (int i = N - 1; i >= 0; i--)
        inv_fact[i] = inv_fact[i + 1] * (i + 1) % M;
}

int counts[B];
ll ans;

ll gnCr(void) {
    int total = 0;
    for (int i = 0; i < B; i++) total += counts[i];
    ll result = fact[total];
    for (int i = 0; i < B; i++)
        result = result * inv_fact[counts[i]] % M;
    return result;
}

void helper(int index, int min_d, ll n) {
    if (index == N) {
        ans = (ans + (n % M) * (gnCr() % M)) % M;
        return;
    }
    for (int d = min_d; d < B; d++) {
        counts[d]++;
        helper(index + 1, d, n * B + d);
        counts[d]--;
    }
}

int main(void) {
    precompute();
    ans = 0;
    for (int i = 0; i < B; i++) counts[i] = 0;
    helper(0, 0, 0);
    ans %= M;
    printf("%lld\n", ans);
    return 0;
}
"""

def main():
    with tempfile.NamedTemporaryFile(suffix='.c', mode='w', delete=False) as f:
        f.write(C_CODE)
        c_path = f.name
    bin_path = c_path.replace('.c', '')
    try:
        subprocess.run(['gcc', '-O2', '-o', bin_path, c_path, '-lm'],
                       check=True, capture_output=True)
        result = subprocess.run([bin_path], capture_output=True, text=True,
                                timeout=280)
        print(result.stdout.strip())
    finally:
        for p in [c_path, bin_path]:
            if os.path.exists(p):
                os.unlink(p)

if __name__ == "__main__":
    main()
