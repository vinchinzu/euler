"""Project Euler Problem 407: Idempotents.

Find sum_{n=1}^{10^7} M(n), where M(n) is the largest a < n with a^2 = a (mod n).

a^2 = a (mod n) means n | a(a-1). For each n, we find the largest such a.
By CRT, for n = p1^e1 * ... * pk^ek, we need a = 0 or 1 mod each pi^ei.
So M(n) is the maximum over all 2^k combinations minus the trivial a=0,1 choices.

We use C for performance since N=10^7 requires fast iteration.
"""
import subprocess, os, tempfile

def solve():
    c_code = r"""
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

/*
 * For each n, M(n) = max a in [0,n) with a^2 = a (mod n), i.e. n | a(a-1).
 * By CRT, if n = p1^e1 * ... * pk^ek, then a = 0 or 1 mod each pi^ei.
 * We enumerate all 2^k combos via a multiplicative approach.
 *
 * Approach: for each prime power p^e <= N, and for each multiple m of p^e,
 * we know a = 0 mod p^e or a = 1 mod p^e. We combine these using CRT.
 *
 * Better approach: directly compute M(n) for all n using a sieve.
 * For each n, M(n) = max{a : a^2 = a (mod n), 0 <= a < n}.
 * We can compute this by iterating over all divisor pairs.
 *
 * Actually, the most efficient approach:
 * For each n, factor n into prime powers. Then use CRT to find all
 * idempotents. But factoring 10^7 numbers is slow.
 *
 * Efficient method using "lift" approach:
 * Start with M[n] = 1 for all n >= 2 (since a=1 always works).
 * For each prime power p^e, consider multiples n of p^e.
 * For n, one CRT component can be 0 mod p^e instead of 1 mod p^e.
 * The idempotent is constructed via CRT.
 *
 * Even more efficient: multiplicative sieve approach.
 * Actually, let's use the Stern tree / coprime pairs approach from the
 * original but in C for speed.
 *
 * Best approach: For each pair (r,s) with gcd(r,s)=1, n=r*s, compute
 * a = CRT(1 mod r, 0 mod s) = s * (s^{-1} mod r).
 * Then M[n] = max(M[n], a) and M[n] = max(M[n], n-a).
 * (since if a is idempotent, so is n-a+1... no, that's wrong)
 * Actually CRT(0 mod r, 1 mod s) = r * (r^{-1} mod s).
 * Both a and n-a (where a = CRT(1,0)) give idempotents because
 * (n-a)^2 - (n-a) = n^2 - 2na + a^2 - n + a = n(n-2a-1) + a^2-a.
 * If a^2=a mod n, then (n-a)^2-(n-a) = a^2-a + n(n-2a-1) = 0 mod n.
 * Hmm, (n-a)^2 - (n-a) = n^2 - 2na + a^2 - n + a = n(n-1-2a) + (a^2+a).
 * Not obviously idempotent. Let me just use: CRT(1,0) and CRT(0,1).
 *
 * Plan: generate all coprime pairs (r,s) with r*s <= N using a BFS on
 * the Stern-Brocot tree. For each pair, compute the two nontrivial
 * idempotents for n = r*s, and update M[n].
 *
 * Actually simpler: just iterate over all divisors. For each d of n where
 * gcd(d, n/d) = 1, compute CRT(1 mod d, 0 mod n/d).
 *
 * Simplest fast approach: sieve-based.
 * For each n, enumerate its coprime factorizations n = r * s.
 * For each such factorization, a = s * modinv(s, r).
 * Update M[n] = max(M[n], a).
 *
 * To enumerate coprime pairs efficiently: iterate over s from 1 to N,
 * and for each s, iterate multiples n = s, 2s, 3s, ..., and check
 * if gcd(s, n/s) = 1. This is O(N log N).
 *
 * Actually even simpler: for each pair (d, n/d) where d | n and
 * gcd(d, n/d) = 1, compute a = CRT(1 mod d, 0 mod n/d).
 * a = (n/d) * modinv(n/d, d). This gives one idempotent.
 * The other is CRT(0 mod d, 1 mod n/d) = d * modinv(d, n/d).
 *
 * We want: for each n, find max a among all such idempotents.
 * Iterate d from 1 to N. For each multiple n of d (n = d*k for k=1,2,...):
 *   s = n/d = k. If gcd(d, k) = 1:
 *     a1 = k * modinv(k, d) mod n  -> CRT(1 mod d, 0 mod k)  [Wait, need to think]
 *     Actually CRT(1 mod d, 0 mod k): a ≡ 1 (mod d), a ≡ 0 (mod k).
 *     a = k * t where k*t ≡ 1 (mod d), so t = modinv(k, d), a = k * modinv(k, d).
 *     Update M[n] = max(M[n], a % n).
 *
 * This is O(N log N) due to the harmonic sum, plus the modinv calls.
 * modinv can be done with extended gcd in O(log n).
 * Total: O(N log^2 N), should be fast enough in C.
 */

#define MAXN 10000001

int M[MAXN];

long long gcd_func(long long a, long long b) {
    while (b) { long long t = b; b = a % b; a = t; }
    return a;
}

/* Extended GCD: returns gcd, sets *x, *y such that a*x + b*y = gcd */
long long extgcd(long long a, long long b, long long *x, long long *y) {
    if (b == 0) { *x = 1; *y = 0; return a; }
    long long x1, y1;
    long long g = extgcd(b, a % b, &x1, &y1);
    *x = y1;
    *y = x1 - (a / b) * y1;
    return g;
}

/* modular inverse of a mod m, returns -1 if not coprime */
long long modinv(long long a, long long m) {
    long long x, y;
    long long g = extgcd(a, m, &x, &y);
    if (g != 1) return -1;
    return ((x % m) + m) % m;
}

int main() {
    int N = 10000000;

    /* Initialize M[n] = 1 for all n >= 2 (a=1 is always idempotent) */
    for (int n = 0; n <= N; n++) M[n] = 1;
    M[0] = 0;
    M[1] = 0;  /* M(1) = 0 since only a=0 works for n=1 */

    /* For each d >= 2, iterate over multiples n = d*k where gcd(d,k)=1 */
    for (int d = 2; d <= N; d++) {
        for (long long n = (long long)d * 2; n <= N; n += d) {
            int k = (int)(n / d);
            if (gcd_func(d, k) != 1) continue;

            /* CRT(1 mod d, 0 mod k): a = k * modinv(k, d) */
            long long inv = modinv(k, d);
            if (inv < 0) continue;
            long long a = ((long long)k * inv) % n;
            if (a > M[n]) M[n] = (int)a;
        }
    }

    long long sum = 0;
    for (int n = 1; n <= N; n++) {
        sum += M[n];
    }
    printf("%lld\n", sum);
    return 0;
}
"""
    tmpdir = tempfile.mkdtemp()
    src = os.path.join(tmpdir, "sol407.c")
    exe = os.path.join(tmpdir, "sol407")
    with open(src, 'w') as f:
        f.write(c_code)
    subprocess.run(["gcc", "-O2", "-o", exe, src, "-lm"], check=True, capture_output=True)
    result = subprocess.run([exe], capture_output=True, text=True, check=True, timeout=30)
    print(result.stdout.strip())

if __name__ == "__main__":
    solve()
