"""Project Euler Problem 400: Fibonacci Tree Game.

Compute number of winning first moves g(N, 1) for N=10000.

f(k) = (f(k-1) XOR f(k-2)) + 1, f(0)=0, f(1)=1.
g(k, n) counts moves from T_k resulting in Nim value n.
Recurrence:
  g(k, (n XOR f(k-2)) + 1) += g(k-1, n)  for all n
  g(k, (f(k-1) XOR n) + 1) += g(k-2, n)  for all n
  g(k, 0) = 1  (clip entire tree)

Answer is g(10000, 1) mod 10^18.
Uses C for speed since the inner loops are large.
"""
import subprocess, os, tempfile

def solve():
    c_code = r"""
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define N 10000
#define L 8192  /* max Nim value is 8191 */
#define MOD 1000000000000000000LL  /* 10^18 */

typedef long long ll;

/* We only need g[k-1] and g[k-2] at any time */
ll g_prev[L + 1];   /* g[k-1] */
ll g_prev2[L + 1];  /* g[k-2] */
ll g_cur[L + 1];    /* g[k] */

int f[N + 1];

int main() {
    /* Compute f values */
    f[0] = 0;
    f[1] = 1;
    for (int k = 2; k <= N; k++)
        f[k] = (f[k-1] ^ f[k-2]) + 1;

    /* Initialize g[0] and g[1] */
    /* g[0]: T(0) is empty, only move is "no move" -> not applicable */
    /* Actually g[0] is never used since T(0) is empty */
    /* g[1]: T(1) is one node. Moves: clip the node -> empty tree -> Nim value 0 */
    /* So g[1][0] = 1, rest = 0 */
    memset(g_prev2, 0, sizeof(g_prev2));  /* g[0] - all zeros */
    memset(g_prev, 0, sizeof(g_prev));    /* g[1] */
    g_prev[0] = 1;

    for (int k = 2; k <= N; k++) {
        memset(g_cur, 0, sizeof(g_cur));

        /* From left subtree T(k-1): g(k, (n ^ f(k-2)) + 1) += g(k-1, n) */
        int fk2 = f[k-2];
        for (int n = 0; n < L; n++) {
            if (g_prev[n] == 0) continue;
            int target = (n ^ fk2) + 1;
            if (target <= L)
                g_cur[target] = (g_cur[target] + g_prev[n]) % MOD;
        }

        /* From right subtree T(k-2): g(k, (f(k-1) ^ n) + 1) += g(k-2, n) */
        int fk1 = f[k-1];
        for (int n = 0; n < L; n++) {
            if (g_prev2[n] == 0) continue;
            int target = (fk1 ^ n) + 1;
            if (target <= L)
                g_cur[target] = (g_cur[target] + g_prev2[n]) % MOD;
        }

        /* Clip entire tree -> Nim value 0 */
        g_cur[0] = (g_cur[0] + 1) % MOD;

        /* Rotate: g_prev2 <- g_prev, g_prev <- g_cur */
        memcpy(g_prev2, g_prev, sizeof(g_prev));
        memcpy(g_prev, g_cur, sizeof(g_cur));
    }

    printf("%lld\n", g_cur[1]);
    return 0;
}
"""
    tmpdir = tempfile.mkdtemp()
    src = os.path.join(tmpdir, "sol400.c")
    exe = os.path.join(tmpdir, "sol400")
    with open(src, 'w') as f:
        f.write(c_code)
    subprocess.run(["gcc", "-O2", "-o", exe, src], check=True, capture_output=True)
    result = subprocess.run([exe], capture_output=True, text=True, check=True, timeout=30)
    print(result.stdout.strip())

if __name__ == "__main__":
    solve()
