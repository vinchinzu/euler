"""Project Euler Problem 459: Flipping game.

A NxN board starts with all white squares. Two players take turns flipping rectangles.
Find the number of starting moves such that the first player can guarantee being the one
to flip the entire board black. Uses Sprague-Grundy theory with nim-products.
"""

import subprocess, tempfile, os

def solve():
    c_code = r"""
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef long long ll;

#define N 1000000
#define L 512

/* nim-product of two nim-values */
/* Uses the standard construction for nim-multiplication in GF(2^n) */
/* For powers of 2: a*b where a=2^i, b=2^j:
   - If i&j == 0: a*b = a|b (simple)
   - Otherwise: use the Fermat 2-adic valuation to reduce */

static int nim_prod_cache[L][L];
static int nim_prod_computed[L][L];

int nim_prod(int a, int b);

/* nim-square of a Fermat 2-power: 2^(2^k) * 2^(2^k) = 3/2 * 2^(2^k) = 3 * 2^(2^k - 1) */
/* Standard nim-product for single-bit values (powers of 2) */
int nim_prod_single(int a, int b) {
    /* a and b are both powers of 2 */
    /* a = 2^i, b = 2^j */
    if (a == 0 || b == 0) return 0;
    if (a == 1) return b;
    if (b == 1) return a;

    /* Find the exponents */
    int i = __builtin_ctz(a);
    int j = __builtin_ctz(b);

    /* If i and j have no common Fermat-power bit, result is a*b (ordinary) */
    if ((i & j) == 0) return a * b;

    /* Find the highest Fermat power that both share */
    int common = i & j;
    int k = 0;
    while ((1 << k) <= common) {
        if (common & (1 << k)) {
            /* Both i and j have the Fermat-power 2^k */
            /* 2^(2^k) * 2^(2^k) = (3/2) * 2^(2^(k+1)) in nim arithmetic */
            /* Actually: nim-square of 2^(2^k) = 3 * 2^(2^k - 1) */
            /* Let's use: D(2^k) * D(2^k) = (3/2) * D(2^(k+1))
               where D(n) = 2^(2^n - 1) (the "super-power of 2")
               Hmm, this is getting complicated. Let me just use the recursive definition. */
            break;
        }
        k++;
    }

    /* Use recursive nim-product definition */
    /* This shouldn't happen often for our range */
    return nim_prod(a, b);
}

int nim_prod(int a, int b) {
    if (a == 0 || b == 0) return 0;
    if (a == 1) return b;
    if (b == 1) return a;
    if (a < L && b < L && nim_prod_computed[a][b]) return nim_prod_cache[a][b];

    int result;

    /* Decompose using linearity: if a = a1 ^ a2, then a*b = a1*b ^ a2*b */
    if (a & (a-1)) { /* a is not a power of 2 */
        int low = a & (-a);
        result = nim_prod(low, b) ^ nim_prod(a ^ low, b);
    } else if (b & (b-1)) { /* b is not a power of 2 */
        int low = b & (-b);
        result = nim_prod(a, low) ^ nim_prod(a, b ^ low);
    } else {
        /* Both a and b are powers of 2 */
        int i = __builtin_ctz(a);
        int j = __builtin_ctz(b);
        if ((i & j) == 0) {
            result = a * b; /* a | b since no common bits */
        } else {
            /* Find highest common Fermat-power bit */
            int common_bit = 0;
            {
                int c = i & j;
                while ((2 << common_bit) <= c) common_bit++;
                /* common_bit = position of highest set bit in (i&j) */
                int temp = c;
                common_bit = 0;
                while (temp > 1) { temp >>= 1; common_bit++; }
            }
            /* 2^(2^k) * 2^(2^k) = 3/2 * 2^(2^(k+1)) in nim */
            /* Actually the formula: if a = 2^(2^k), then a*a = a + a/2 * a (something like that) */
            /* Standard: 2^(2^k) squared in nim = 3/2 * 2^(2^k)
               No wait. The standard result for Nim multiplication:
               For Fermat 2-powers F_k = 2^(2^k):
               F_k * F_k = (3/2) * F_k in nim? No.

               The correct rule: F_k * F_k = F_k + F_k * (F_k - 1) / 2
               Actually: In GF(2^(2^n)), the Fermat power F_n = 2^(2^n) satisfies
               F_n^2 = F_n * 3/2  NO.

               The correct recursive definition:
               F_0 = 2. F_0 * F_0 = 3 (since 2*2 in nim = 3).
               F_1 = 4. F_1 * F_1 = 6 (since this is the nim square of 4).
               Actually let me just compute from the definition.
            */
            /* Fall back to mex computation for small values */
            /* a*b for powers of 2 with common Fermat bits */
            /* Use the formula:
               nim_prod(2^i, 2^j) where i = a1 + 2^k, j = a2 + 2^k
               = nim_prod(2^a1, 2^a2) * nim_prod(2^(2^k), 2^(2^k))
               ... only if a1 and a2 don't share the 2^k bit.

               nim_prod(2^(2^k), 2^(2^k)) = 3 * 2^(2^k - 1)
               This is the standard result.
            */
            int fk = 1 << common_bit; /* 2^k */
            int Fk = 1 << fk; /* 2^(2^k) - the Fermat power */
            /* nim_square(Fk) = 3 * 2^(2^k - 1) = 3 * (Fk/2) = Fk + Fk/2 */
            int sq_Fk = Fk + (Fk >> 1); /* 3/2 * Fk = Fk | (Fk >> 1) */

            /* a = 2^i = 2^(i - 2^k) * Fk, b = 2^j = 2^(j - 2^k) * Fk */
            int a2 = 1 << (i - fk);
            int b2 = 1 << (j - fk);
            result = nim_prod(nim_prod(a2, b2), sq_Fk);
        }
    }

    if (a < L && b < L) {
        nim_prod_cache[a][b] = result;
        nim_prod_computed[a][b] = 1;
    }
    return result;
}

int main(void) {
    memset(nim_prod_computed, 0, sizeof(nim_prod_computed));

    /* Step sizes: squares and triangular numbers */
    /* squares: 1, 4, 9, 16, ... up to N */
    /* triangular: 1, 3, 6, 10, ... up to N */

    int nsq = 0, ntr = 0;
    int *sq_steps = (int*)malloc((N+1) * sizeof(int));
    int *tr_steps = (int*)malloc((N+1) * sizeof(int));
    for (int i = 1; (ll)i*i <= N; i++) sq_steps[nsq++] = i*i;
    for (int i = 1; (ll)i*(i+1)/2 <= N; i++) tr_steps[ntr++] = i*(i+1)/2;

    /* Compute range nimbers using Sprague-Grundy */
    /* For dimension d (0=squares, 1=triangular):
       range_nimber[j] = range_nimber[j-1] XOR mex({range_nimber[j-1] XOR range_nimber[j-step] : step valid}) */

    int *rnX = (int*)calloc(N+1, sizeof(int));
    int *rnY = (int*)calloc(N+1, sizeof(int));

    /* Dimension 0: squares */
    for (int j = 1; j <= N; j++) {
        int used[L];
        memset(used, 0, sizeof(used));
        for (int k = 0; k < nsq && sq_steps[k] <= j; k++) {
            int val = rnX[j-1] ^ rnX[j - sq_steps[k]];
            if (val < L) used[val] = 1;
        }
        int mex = 0;
        while (mex < L && used[mex]) mex++;
        rnX[j] = rnX[j-1] ^ mex;
    }

    /* Dimension 1: triangular */
    for (int j = 1; j <= N; j++) {
        int used[L];
        memset(used, 0, sizeof(used));
        for (int k = 0; k < ntr && tr_steps[k] <= j; k++) {
            int val = rnY[j-1] ^ rnY[j - tr_steps[k]];
            if (val < L) used[val] = 1;
        }
        int mex = 0;
        while (mex < L && used[mex]) mex++;
        rnY[j] = rnY[j-1] ^ mex;
    }

    /* Count occurrences of each nimber value for valid moves */
    ll *cntX = (ll*)calloc(L, sizeof(ll));
    ll *cntY = (ll*)calloc(L, sizeof(ll));

    for (int j = 1; j <= N; j++) {
        for (int k = 0; k < nsq && sq_steps[k] <= j; k++) {
            int val = rnX[j] ^ rnX[j - sq_steps[k]];
            if (val < L) cntX[val]++;
        }
    }
    for (int j = 1; j <= N; j++) {
        for (int k = 0; k < ntr && tr_steps[k] <= j; k++) {
            int val = rnY[j] ^ rnY[j - tr_steps[k]];
            if (val < L) cntY[val]++;
        }
    }

    /* Target nim value */
    int target = nim_prod(rnX[N], rnY[N]);

    /* Count pairs (n0, n1) where nim_prod(n0, n1) == target */
    ll ans = 0;
    for (int n0 = 0; n0 < L; n0++) {
        if (cntX[n0] == 0) continue;
        for (int n1 = 0; n1 < L; n1++) {
            if (cntY[n1] == 0) continue;
            if (nim_prod(n0, n1) == target) {
                ans += cntX[n0] * cntY[n1];
            }
        }
    }

    printf("%lld\n", ans);

    free(sq_steps); free(tr_steps);
    free(rnX); free(rnY);
    free(cntX); free(cntY);
    return 0;
}
"""
    with tempfile.NamedTemporaryFile(suffix='.c', mode='w', delete=False) as src:
        src.write(c_code)
        src_path = src.name
    bin_path = src_path.replace('.c', '')
    try:
        subprocess.run(['gcc', '-O2', '-o', bin_path, src_path, '-lm'], check=True,
                       capture_output=True, text=True)
        result = subprocess.run([bin_path], capture_output=True, text=True, check=True, timeout=30)
        print(result.stdout.strip())
    except subprocess.CalledProcessError as e:
        print(f"Compile/run error: {e.stderr}", flush=True)
        raise
    finally:
        os.unlink(src_path)
        if os.path.exists(bin_path):
            os.unlink(bin_path)

if __name__ == "__main__":
    solve()
