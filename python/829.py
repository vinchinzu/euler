#!/usr/bin/env python3
"""
Project Euler 829 - Integral Fusion

M(n) = smallest number with same factor tree shape as n!!.
Find sum_{n=2}^{31} M(n).

Uses C extension for fast tree shape computation and number enumeration.
"""

import subprocess, tempfile, os
from math import isqrt
from bisect import bisect_right

C_CODE = r"""
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

#define MAX_BUF 512
#define MAX_TARGETS 30
#define MAX_PRIMES 16

static int primes[] = {2,3,5,7,11,13,17,19,23,29,31,37,41,43,47,53};
static int nprimes = 16;

/* Is n prime? Trial division. */
int is_prime(long long n) {
    if (n < 2) return 0;
    if (n < 4) return 1;
    if (n % 2 == 0 || n % 3 == 0) return 0;
    for (long long d = 5; d * d <= n; d += 6)
        if (n % d == 0 || n % (d+2) == 0) return 0;
    return 1;
}

/* Get all divisors of n, store in divs[], return count.
   divs must be large enough. For n with at most ~40 prime factors,
   max divisors is bounded. */
int get_divisors(long long n, long long *divs) {
    int ps[64], es[64], nf = 0;
    long long temp = n;
    for (long long d = 2; d * d <= temp; d++) {
        if (temp % d == 0) {
            ps[nf] = d; es[nf] = 0;
            while (temp % d == 0) { es[nf]++; temp /= d; }
            nf++;
        }
    }
    if (temp > 1) { ps[nf] = temp; es[nf] = 1; nf++; }

    divs[0] = 1;
    int nd = 1;
    for (int i = 0; i < nf; i++) {
        int old = nd;
        long long pk = 1;
        for (int j = 0; j < es[i]; j++) {
            pk *= ps[i];
            for (int k = 0; k < old; k++)
                divs[nd++] = divs[k] * pk;
        }
    }
    /* Sort using insertion sort (fast for moderate nd) */
    for (int i = 1; i < nd; i++) {
        long long key = divs[i];
        int j = i - 1;
        while (j >= 0 && divs[j] > key) { divs[j+1] = divs[j]; j--; }
        divs[j+1] = key;
    }
    return nd;
}

/* Best divisor of n: largest divisor <= sqrt(n) */
long long best_divisor(long long n) {
    long long divs[131072];
    int nd = get_divisors(n, divs);
    long long s = (long long)sqrtl((long double)n);
    while ((s+1)*(s+1) <= n) s++;
    while (s*s > n) s--;
    /* Binary search for largest div <= s */
    int lo = 0, hi = nd - 1, res = 0;
    while (lo <= hi) {
        int mid = (lo + hi) / 2;
        if (divs[mid] <= s) { res = mid; lo = mid + 1; }
        else hi = mid - 1;
    }
    return divs[res];
}

/* Tree shape as string */
int tree_shape(long long n, char *buf) {
    if (is_prime(n)) {
        buf[0] = 'L'; buf[1] = 0; return 1;
    }
    long long a = best_divisor(n);
    long long b = n / a;
    buf[0] = '(';
    int len = 1;
    len += tree_shape(a, buf + len);
    len += tree_shape(b, buf + len);
    buf[len++] = ')';
    buf[len] = 0;
    return len;
}

/* Count 'L' in string = number of leaves */
int count_leaves(const char *s) {
    int c = 0;
    for (; *s; s++) if (*s == 'L') c++;
    return c;
}

/* Target shapes */
char targets[MAX_TARGETS][MAX_BUF];
int target_k[MAX_TARGETS];
long long target_M[MAX_TARGETS];
int ntargets = 0;
int nremaining = 0;

/* Check if n matches any target */
void check_number(long long n) {
    char buf[MAX_BUF];
    tree_shape(n, buf);
    int k = count_leaves(buf);

    for (int i = 0; i < ntargets; i++) {
        if (target_M[i] != -1) continue; /* Already found */
        if (target_k[i] != k) continue;
        if (strcmp(buf, targets[i]) == 0) {
            target_M[i] = n;
            nremaining--;
        }
    }
}

/* Generate all products of exactly k primes (from first np primes)
   that are <= limit, in increasing order, using recursive enumeration.
   Call check_number for each. */
void gen_products(int depth, int k, int min_pi, long long cur, long long limit) {
    if (nremaining == 0) return;
    if (depth == k) {
        check_number(cur);
        return;
    }
    int remaining = k - depth;
    for (int pi = min_pi; pi < nprimes; pi++) {
        long long next = cur * primes[pi];
        if (next > limit) break;
        /* Check if we can still fill remaining-1 slots */
        /* Minimum remaining product: primes[pi]^(remaining-1) */
        long long min_rest = 1;
        int ok = 1;
        for (int r = 0; r < remaining - 1; r++) {
            min_rest *= primes[pi];
            if (min_rest > limit / next) { ok = 0; break; }
        }
        if (!ok && remaining > 1) break;
        gen_products(depth + 1, k, pi, next, limit);
    }
}

int main() {
    char line[MAX_BUF];

    /* Read targets from stdin: "k shape" per line, then empty line to start */
    while (fgets(line, sizeof(line), stdin)) {
        if (line[0] == '\n' || line[0] == '\r') break;
        int k;
        char shape[MAX_BUF];
        if (sscanf(line, "%d %s", &k, shape) == 2) {
            target_k[ntargets] = k;
            strcpy(targets[ntargets], shape);
            target_M[ntargets] = -1;
            ntargets++;
            nremaining++;
        }
    }

    /* For each k value, enumerate products with limit = 2^k * factor */
    /* Group targets by k */
    int max_k = 0;
    for (int i = 0; i < ntargets; i++)
        if (target_k[i] > max_k) max_k = target_k[i];

    /* Process each k value */
    for (int k = 2; k <= max_k && nremaining > 0; k++) {
        /* Check if any target has this k */
        int has_k = 0;
        for (int i = 0; i < ntargets; i++)
            if (target_k[i] == k && target_M[i] == -1) { has_k = 1; break; }
        if (!has_k) continue;

        /* Try increasing limits */
        long long base_limit = 1;
        for (int i = 0; i < k; i++) {
            base_limit *= 2;
            if (base_limit > (long long)2e15) { base_limit = (long long)2e15; break; }
        }

        for (int mult = 1; mult <= 256 && nremaining > 0; mult *= 2) {
            long long limit = base_limit * mult;
            if (limit > (long long)2e15) limit = (long long)2e15;

            /* Check if any k-target is still unfound */
            int still_need = 0;
            for (int i = 0; i < ntargets; i++)
                if (target_k[i] == k && target_M[i] == -1) { still_need = 1; break; }
            if (!still_need) break;

            gen_products(0, k, 0, 1, limit);
            if (limit >= (long long)2e15) break;
        }
    }

    /* Output results */
    for (int i = 0; i < ntargets; i++) {
        printf("%lld\n", target_M[i]);
    }

    return 0;
}
"""

def sieve(limit):
    is_p = bytearray(b'\x01') * (limit + 1)
    is_p[0] = is_p[1] = 0
    for i in range(2, int(limit**0.5) + 1):
        if is_p[i]:
            is_p[i*i::i] = bytearray(len(is_p[i*i::i]))
    return [i for i in range(2, limit + 1) if is_p[i]]

def is_prime_py(n):
    if n < 2: return False
    if n == 2 or n == 3: return True
    if n % 2 == 0 or n % 3 == 0: return False
    d = 5
    while d * d <= n:
        if n % d == 0 or n % (d + 2) == 0: return False
        d += 6
    return True

def factorize_py(n):
    f = {}
    d = 2
    while d * d <= n:
        while n % d == 0: f[d] = f.get(d, 0) + 1; n //= d
        d += 1 if d == 2 else 2
    if n > 1: f[n] = 1
    return f

def get_divs_py(n):
    if n == 1: return [1]
    f = factorize_py(n)
    d = [1]
    for p, e in f.items():
        nd = []
        for dd in d:
            pk = 1
            for _ in range(e + 1): nd.append(dd * pk); pk *= p
        d = nd
    return sorted(d)

def best_div_py(n):
    d = get_divs_py(n)
    s = isqrt(n)
    return d[bisect_right(d, s) - 1]

_sc = {}
def tree_shape_py(n):
    if n in _sc: return _sc[n]
    if is_prime_py(n): _sc[n] = 'L'; return 'L'
    a = best_div_py(n); b = n // a
    r = '(' + tree_shape_py(a) + tree_shape_py(b) + ')'
    _sc[n] = r; return r

def solve():
    # Compute target shapes
    targets = []
    for n in range(2, 32):
        ndf = 1
        for i in range(n, 0, -2): ndf *= i
        shape = tree_shape_py(ndf)
        k = shape.count('L')
        targets.append((n, k, shape))

    # Group by unique shapes
    unique = {}
    for n, k, shape in targets:
        if shape not in unique:
            unique[shape] = (k, [])
        unique[shape][1].append(n)

    # Compile C
    src = tempfile.NamedTemporaryFile(suffix='.c', delete=False, mode='w')
    src.write(C_CODE)
    src.close()
    exe = src.name.replace('.c', '')
    subprocess.run(['gcc', '-O2', '-o', exe, src.name, '-lm'], check=True)
    os.unlink(src.name)

    shape_to_M = {}
    try:
        # Prepare input
        input_lines = []
        shape_order = []
        for shape, (k, ns) in unique.items():
            if shape == 'L':
                shape_to_M[shape] = 2
            else:
                input_lines.append(f'{k} {shape}')
                shape_order.append(shape)

        input_data = '\n'.join(input_lines) + '\n\n'

        proc = subprocess.run(
            [exe],
            input=input_data,
            capture_output=True,
            text=True,
            timeout=25
        )
        results = proc.stdout.strip().split('\n')
        for i, shape in enumerate(shape_order):
            if i < len(results):
                shape_to_M[shape] = int(results[i])
    finally:
        if os.path.exists(exe):
            os.unlink(exe)

    # Compute total
    total = 0
    for n, k, shape in targets:
        m = shape_to_M.get(shape, 0)
        total += m

    return total

if __name__ == "__main__":
    print(solve())
