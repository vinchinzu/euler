"""Project Euler Problem 378: Triangle Triples.

Compute Tr(60000000): count triples (i,j,k) with 1<=i<j<k<=N and dT(i)>dT(j)>dT(k),
where dT(n) = number of divisors of T(n) = n(n+1)/2.

Output last 18 digits (i.e., mod 10^18).
Uses C for performance.
"""
import subprocess, os, tempfile

def solve():
    c_code = r"""
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define N 60000000
#define MOD 1000000000000000000LL

typedef long long ll;
typedef unsigned short us;

int *spf;

void sieve_spf() {
    spf = (int*)calloc(N + 2, sizeof(int));
    if (!spf) { fprintf(stderr, "Failed to allocate spf\n"); exit(1); }
    for (int i = 0; i <= N + 1; i++) spf[i] = i;
    for (int i = 2; (ll)i * i <= N + 1; i++) {
        if (spf[i] != i) continue;
        for (int j = i * i; j <= N + 1; j += i)
            if (spf[j] == j) spf[j] = i;
    }
}

int count_divisors(int n) {
    if (n <= 1) return 1;
    int result = 1;
    while (n > 1) {
        int p = spf[n], e = 0;
        while (n % p == 0) { e++; n /= p; }
        result *= (e + 1);
    }
    return result;
}

int dT_func(int n) {
    int a = n, b = n + 1;
    if (a % 2 == 0) a /= 2; else b /= 2;
    return count_divisors(a) * count_divisors(b);
}

#define MAXVAL 25000
ll fenwick[MAXVAL + 2];
int fenwick_size;

void fenwick_init(int sz) {
    fenwick_size = sz;
    memset(fenwick, 0, (sz + 2) * sizeof(ll));
}

void fenwick_update(int pos, ll val) {
    for (pos++; pos <= fenwick_size; pos += pos & (-pos))
        fenwick[pos] = (fenwick[pos] + val) % MOD;
}

ll fenwick_query(int pos) {
    ll s = 0;
    for (pos++; pos > 0; pos -= pos & (-pos))
        s += fenwick[pos];
    return s % MOD;
}

ll fenwick_prefix(int pos) {
    if (pos < 0) return 0;
    return fenwick_query(pos);
}

int main() {
    sieve_spf();

    us *dt = (us*)malloc((N + 1) * sizeof(us));
    if (!dt) { fprintf(stderr, "Failed to allocate dt\n"); exit(1); }

    int max_dt = 0;
    for (int i = 1; i <= N; i++) {
        int d = dT_func(i);
        dt[i] = (us)d;
        if (d > max_dt) max_dt = d;
    }
    free(spf);  /* Free early to save memory */

    /* Use int instead of ll for right[] to save memory (counts fit in int) */
    int *right_arr = (int*)malloc((N + 1) * sizeof(int));
    if (!right_arr) { fprintf(stderr, "Failed to allocate right\n"); exit(1); }

    fenwick_init(max_dt + 2);
    for (int j = N; j >= 1; j--) {
        right_arr[j] = (dt[j] > 0) ? (int)(fenwick_prefix(dt[j] - 1)) : 0;
        fenwick_update(dt[j], 1);
    }

    fenwick_init(max_dt + 2);
    ll answer = 0;

    for (int j = 1; j <= N; j++) {
        ll left_j = (dt[j] < max_dt) ?
            (fenwick_prefix(max_dt) - fenwick_prefix(dt[j]) + MOD) % MOD : 0;
        answer = (answer + left_j * right_arr[j]) % MOD;
        fenwick_update(dt[j], 1);
    }

    printf("%lld\n", answer);

    free(dt);
    free(right_arr);
    return 0;
}
"""
    tmpdir = tempfile.mkdtemp()
    src = os.path.join(tmpdir, "sol378.c")
    exe = os.path.join(tmpdir, "sol378")
    with open(src, 'w') as f:
        f.write(c_code)
    subprocess.run(["gcc", "-O2", "-o", exe, src], check=True, capture_output=True)
    result = subprocess.run([exe], capture_output=True, text=True, check=True, timeout=30)
    print(result.stdout.strip())

if __name__ == "__main__":
    solve()
