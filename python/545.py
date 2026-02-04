"""Project Euler Problem 545: Faulhaber's Formulas.

D(k) = product of primes p where (p-1)|k (von Staudt-Clausen theorem).
Find F(10^5) = the 100000th value of k where D(k) = 20010.
20010 = 2*3*5*23*29, so the required primes have (p-1) in {1,2,4,22,28}.
lcm(1,2,4,22,28) = 308. So k = 308*m for valid m.
m is valid if no divisor d of m satisfies: d*g+1 is prime (and not in our set) for some g|308.
"""

import subprocess, tempfile, os

def solve():
    c_code = r"""
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef long long ll;
typedef unsigned long long ull;

/* Divisors of 308 = 2^2 * 7 * 11 */
static const int divs308[] = {1,2,4,7,11,14,22,28,44,77,154,308};
static const int ndivs = 12;

/* Good primes: these are the primes whose product is 20010 */
static int is_good_prime(ll p) {
    return p==2||p==3||p==5||p==23||p==29;
}

int main(void) {
    const int TARGET = 100000;
    const int BASE = 308;
    const int L = 5000000;

    /* We need to check, for each d in [2,L], whether d*g+1 is prime for any g|308.
     * The maximum value to check is L * 308 + 1 ≈ 1.54 * 10^9.
     * We'll use a segmented approach: for each g, collect values d*g+1 and check primality.
     *
     * Better approach: sieve primes up to L*308+1, then for each prime p,
     * compute d = (p-1)/gcd(p-1,308) and if d <= L, mark d as forbidden.
     *
     * Sieving up to 1.54*10^9 is expensive in memory (~200MB for a bitset).
     * Instead, use a segmented sieve.
     */

    /* Alternative: for each divisor g of 308, we need to check if d*g+1 is prime
     * for d in [2, L]. The candidate primes are in arithmetic progression:
     * g+1, 2g+1, 3g+1, ..., L*g+1.
     * For each such AP, sieve primes in that AP using a standard sieve.
     * Actually, it's easier to just sieve all primes up to max(L*g+1) and look them up.
     *
     * Max value = L * 308 + 1 = 1,540,000,001.
     * Segmented sieve with sqrt(1.54e9) ≈ 39244 small primes.
     */

    ll MAX_P = (ll)L * 308 + 2;

    /* Sieve small primes up to sqrt(MAX_P) */
    int sqrt_max = 40000;
    char *small_sieve = (char*)calloc(sqrt_max + 1, 1);
    memset(small_sieve, 1, sqrt_max + 1);
    small_sieve[0] = small_sieve[1] = 0;
    for (int i = 2; (ll)i*i <= sqrt_max; i++) {
        if (small_sieve[i]) {
            for (int j = i*i; j <= sqrt_max; j += i)
                small_sieve[j] = 0;
        }
    }
    int *sprimes = (int*)malloc(sqrt_max * sizeof(int));
    int nsprimes = 0;
    for (int i = 2; i <= sqrt_max; i++)
        if (small_sieve[i]) sprimes[nsprimes++] = i;
    free(small_sieve);

    /* forbidden[d] = 1 means d is a forbidden divisor */
    char *forbidden = (char*)calloc(L + 1, 1);

    /* Segmented sieve to find all primes up to MAX_P */
    /* For each prime p found, compute d = (p-1)/gcd(p-1,308) and mark forbidden */
    const int SEG_SIZE = 1 << 20; /* ~1M */
    char *seg = (char*)malloc(SEG_SIZE);

    for (ll lo = 2; lo < MAX_P; lo += SEG_SIZE) {
        ll hi = lo + SEG_SIZE - 1;
        if (hi >= MAX_P) hi = MAX_P - 1;
        int len = (int)(hi - lo + 1);

        memset(seg, 1, len);

        for (int i = 0; i < nsprimes; i++) {
            int p = sprimes[i];
            ll start = ((lo + p - 1) / p) * p;
            if (start == p) start += p; /* don't mark p itself */
            if (start < lo) start = lo; /* shouldn't happen */
            for (ll j = start; j <= hi; j += p)
                seg[(int)(j - lo)] = 0;
        }

        /* Process primes in this segment */
        for (int i = 0; i < len; i++) {
            if (!seg[i]) continue;
            ll p = lo + i;
            if (is_good_prime(p)) continue;

            /* Compute gcd(p-1, 308) */
            ll pm1 = p - 1;
            ll g = pm1;
            ll b = 308;
            while (b) { ll t = b; b = g % b; g = t; }
            /* g = gcd(pm1, 308) */

            ll d = pm1 / g;
            if (d >= 2 && d <= L) {
                forbidden[d] = 1;
            }
        }
    }
    free(seg);
    free(sprimes);

    /* Now sieve valid array: valid[m] = 1 if no forbidden d divides m */
    char *valid = (char*)malloc(L + 1);
    memset(valid, 1, L + 1);

    for (int d = 2; d <= L; d++) {
        if (forbidden[d]) {
            for (int j = d; j <= L; j += d)
                valid[j] = 0;
        }
    }
    free(forbidden);

    /* Count valid m values */
    int count = 0;
    for (int m = 1; m <= L; m++) {
        if (valid[m]) {
            count++;
            if (count == TARGET) {
                printf("%lld\n", (ll)m * BASE);
                free(valid);
                return 0;
            }
        }
    }

    fprintf(stderr, "Need larger L! Found %d valid values out of %d\n", count, L);
    free(valid);
    return 1;
}
"""
    with tempfile.NamedTemporaryFile(suffix='.c', mode='w', delete=False) as src:
        src.write(c_code)
        src_path = src.name
    bin_path = src_path.replace('.c', '')
    try:
        subprocess.run(['gcc', '-O2', '-o', bin_path, src_path, '-lm'], check=True,
                       capture_output=True, text=True)
        result = subprocess.run([bin_path], capture_output=True, text=True, check=True, timeout=28)
        print(result.stdout.strip())
    except subprocess.CalledProcessError as e:
        print(f"Error: {e.stderr}", flush=True)
        raise
    finally:
        os.unlink(src_path)
        if os.path.exists(bin_path):
            os.unlink(bin_path)

if __name__ == "__main__":
    solve()
