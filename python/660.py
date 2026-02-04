"""Project Euler Problem 660: Pandigital Triangles.

Find the sum of the longest sides of all triangles with a 120-degree angle
and integer sides where the three sides together are pandigital in some base
b with 9 <= b <= 18.

A 120-degree triangle has sides (a, b, c) with c^2 = a^2 + ab + b^2.
Using the parameterization: for coprime m > n with m+n not divisible by 3:
  a = k*(m^2 - n^2), b = k*m*(2n - m), c = k*(m^2 - mn + n^2)
with n < m < 2n.
"""

import subprocess, tempfile, os

def solve():
    c_code = r"""
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef long long ll;

int gcd(int a, int b) {
    while (b) { int t = b; b = a % b; a = t; }
    return a;
}

/* Check if the three sides together use each digit 0..base-1 exactly once */
int is_pandigital(ll a, ll b, ll c, int base) {
    int used[20] = {0};
    int count = 0;
    ll sides[3] = {a, b, c};
    for (int s = 0; s < 3; s++) {
        ll x = sides[s];
        if (x == 0) {
            if (used[0]) return 0;
            used[0] = 1;
            count++;
        }
        while (x > 0) {
            int d = (int)(x % base);
            if (used[d]) return 0;
            used[d] = 1;
            x /= base;
            count++;
        }
    }
    return (count == base);
}

/* Hash set for storing (a, b, c) triangles to avoid duplicates */
#define HASH_SIZE (1 << 20)
#define HASH_MASK (HASH_SIZE - 1)

typedef struct Entry {
    ll a, b, c;
    struct Entry *next;
} Entry;

Entry *table[HASH_SIZE];
int entry_pool_size = 0;
Entry *entry_pool = NULL;

unsigned int hash3(ll a, ll b, ll c) {
    unsigned long long h = (unsigned long long)a * 1000000007ULL + (unsigned long long)b * 999999937ULL + (unsigned long long)c;
    return (unsigned int)(h ^ (h >> 32)) & HASH_MASK;
}

/* Returns 1 if newly inserted, 0 if already existed */
int insert(ll a, ll b, ll c) {
    unsigned int h = hash3(a, b, c);
    Entry *e = table[h];
    while (e) {
        if (e->a == a && e->b == b && e->c == c) return 0;
        e = e->next;
    }
    Entry *ne = &entry_pool[entry_pool_size++];
    ne->a = a; ne->b = b; ne->c = c;
    ne->next = table[h];
    table[h] = ne;
    return 1;
}

int ceil_div(int a, int b) {
    return (a + b - 1) / b;
}

int main() {
    int A = 9, B = 18;

    /* Allocate entry pool - generous size */
    entry_pool = (Entry *)malloc(5000000 * sizeof(Entry));
    memset(table, 0, sizeof(table));

    ll total = 0;

    for (int base = A; base <= B; base++) {
        /* Compute limit for longest side */
        ll limit = 1;
        int e1 = ceil_div(base, 3);
        for (int i = 0; i < e1; i++) limit *= base;
        ll limit2 = 1;
        for (int i = 0; i < e1 - 1; i++) limit2 *= base;
        limit += limit2;

        int tri_base = (base * (base - 1)) / 2;
        int expected_pmod = tri_base % (base - 1);

        for (int n = 1; ; n++) {
            /* longest_side(n, n, 1) = n^2 - n*n + n^2 = n^2 */
            if ((ll)n * n > limit) break;

            for (int m = n + 1; m < 2 * n; m++) {
                ll ls1 = (ll)m*m - (ll)m*n + (ll)n*n;
                if (ls1 > limit) break;

                if ((m + n) % 3 == 0) continue;
                if (gcd(m, n) != 1) continue;

                for (int k = 1; ; k++) {
                    ll c = (ll)k * ls1;
                    if (c > limit) break;

                    /* Check perimeter mod (base-1) */
                    /* perimeter = k*m*(m+n), a+b+c = k*(m^2-n^2 + m*(2n-m) + m^2-mn+n^2) */
                    /* = k*(m^2-n^2+2mn-m^2+m^2-mn+n^2) = k*(m^2+mn) = k*m*(m+n) */
                    ll peri = (ll)k * m * (m + n);
                    if (peri % (base - 1) != expected_pmod) continue;

                    ll a = (ll)k * ((ll)m*m - (ll)n*n);
                    ll b = (ll)k * m * (2*n - m);

                    if (is_pandigital(a, b, c, base)) {
                        if (insert(a, b, c)) {
                            total += c;
                        }
                    }
                }
            }
        }
    }

    printf("%lld\n", total);

    free(entry_pool);
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
        result = subprocess.run([exe], input="", capture_output=True, text=True, timeout=28)
        if result.returncode != 0:
            import sys; sys.stderr.write("Runtime: " + result.stderr + "\n"); return -1
        return int(result.stdout.strip())


if __name__ == "__main__":
    print(solve())
