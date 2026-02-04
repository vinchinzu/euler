"""Project Euler Problem 553: Power Sets of Power Sets.

C(n, k) counts elements of R(n) whose intersection graph has exactly k connected components.
Find C(10^4, 10) mod 10^9+7.

Uses EGF approach:
1. a(i) = 2^(2^i) / i! (EGF for total graphs on subsets of {1..i})
2. Multiply by e^{-x} to get graphs using ALL elements (connected or not)
3. Take log to get EGF of connected graphs
4. Raise to K-th power for K connected components
5. Multiply by e^x to allow unused elements
6. Extract coefficient of x^N, multiply by N!/K!

Uses NTT for fast polynomial multiplication.
"""

import subprocess, tempfile, os

def solve():
    c_code = r"""
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef long long ll;
static const ll MOD = 1000000007;

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

ll inv(ll a, ll mod) { return power(a, mod - 2, mod); }

/* NTT-friendly prime for intermediate computation */
/* We'll use MOD = 10^9+7 directly with Karatsuba or just do O(N^2) carefully */
/* Actually for N=10000, O(N^2) multiply takes ~100M ops, which in C is ~0.3s */
/* With multiple multiplies it might be tight, so let's be smart about it */

static ll fact[10001], inv_fact[10001];
static ll a_coeff[10001]; /* 2^(2^i) / i! */
static ll e_neg[10001];   /* (-1)^i / i! */
static ll e_pos[10001];   /* 1/i! */

void precompute(int N) {
    fact[0] = 1;
    for (int i = 1; i <= N; i++) fact[i] = fact[i-1] * i % MOD;
    inv_fact[N] = inv(fact[N], MOD);
    for (int i = N-1; i >= 0; i--) inv_fact[i] = inv_fact[i+1] * (i+1) % MOD;

    for (int i = 0; i <= N; i++) {
        /* 2^(2^i) mod MOD: compute 2^i mod (MOD-1) first by Fermat */
        ll exp_mod = power(2, i, MOD - 1);
        a_coeff[i] = power(2, exp_mod, MOD) % MOD * inv_fact[i] % MOD;
    }

    for (int i = 0; i <= N; i++) {
        e_neg[i] = (i % 2 == 0) ? inv_fact[i] : (MOD - inv_fact[i]) % MOD;
        e_pos[i] = inv_fact[i];
    }
}

/* Polynomial multiply truncated to degree N, O(N^2) */
void poly_mul(ll *a, ll *b, ll *out, int N) {
    memset(out, 0, (N+1) * sizeof(ll));
    for (int i = 0; i <= N; i++) {
        if (a[i] == 0) continue;
        ll ai = a[i];
        for (int j = 0; j <= N - i; j++) {
            out[i+j] = (out[i+j] + ai * b[j]) % MOD;
        }
    }
}

/* Polynomial inverse: given f with f[0]!=0, find g s.t. f*g = 1 mod x^(N+1) */
/* O(N^2) iterative */
void poly_inv(ll *f, ll *g, int N) {
    memset(g, 0, (N+1) * sizeof(ll));
    g[0] = inv(f[0], MOD);
    for (int i = 1; i <= N; i++) {
        ll s = 0;
        for (int j = 1; j <= i; j++) {
            if (j < N+1 && f[j] != 0)
                s = (s + f[j] * g[i-j]) % MOD;
        }
        g[i] = (MOD - g[0] % MOD * s % MOD) % MOD;
    }
}

/* Polynomial derivative */
void poly_deriv(ll *f, ll *df, int N) {
    for (int i = 0; i < N; i++) {
        df[i] = f[i+1] * (i+1) % MOD;
    }
    df[N] = 0;
}

/* Polynomial integral */
void poly_integ(ll *f, ll *intf, int N) {
    intf[0] = 0;
    for (int i = 0; i < N; i++) {
        intf[i+1] = f[i] % MOD * inv(i+1, MOD) % MOD;
    }
}

/* Polynomial log: log(f) = integral(f'/f), f[0] must be 1 */
void poly_log(ll *f, ll *logf, int N) {
    ll *df = (ll*)calloc(N+1, sizeof(ll));
    ll *finv = (ll*)calloc(N+1, sizeof(ll));
    ll *quot = (ll*)calloc(N+1, sizeof(ll));

    poly_deriv(f, df, N);
    poly_inv(f, finv, N);
    poly_mul(df, finv, quot, N);
    poly_integ(quot, logf, N);

    free(df); free(finv); free(quot);
}

/* Polynomial exp: exp(f), f[0] must be 0, using Newton iteration O(N^2) */
/* Actually simpler: if we know log, we can compute exp iteratively:
   g[0] = 1
   g[n] = (1/n) * sum_{k=1}^{n} k * f[k] * g[n-k]
*/
void poly_exp(ll *f, ll *g, int N) {
    memset(g, 0, (N+1) * sizeof(ll));
    g[0] = 1;
    /* Precompute k*f[k] */
    ll *kf = (ll*)calloc(N+1, sizeof(ll));
    for (int k = 1; k <= N; k++) kf[k] = (ll)k % MOD * f[k] % MOD;

    for (int n = 1; n <= N; n++) {
        ll s = 0;
        for (int k = 1; k <= n; k++) {
            s = (s + kf[k] * g[n-k]) % MOD;
        }
        g[n] = s % MOD * inv(n, MOD) % MOD;
    }
    free(kf);
}

/* Precompute all inverses 1..N */
static ll inv_table[10001];
void precompute_inv(int N) {
    inv_table[1] = 1;
    for (int i = 2; i <= N; i++) {
        inv_table[i] = (MOD - MOD / i) * inv_table[MOD % i] % MOD;
    }
}

/* Faster polynomial exp using precomputed inverses */
void poly_exp_fast(ll *f, ll *g, int N) {
    memset(g, 0, (N+1) * sizeof(ll));
    g[0] = 1;
    ll *kf = (ll*)calloc(N+1, sizeof(ll));
    for (int k = 1; k <= N; k++) kf[k] = (ll)k % MOD * f[k] % MOD;

    for (int n = 1; n <= N; n++) {
        ll s = 0;
        for (int k = 1; k <= n; k++) {
            s = (s + kf[k] * g[n-k]) % MOD;
        }
        g[n] = s % MOD * inv_table[n] % MOD;
    }
    free(kf);
}

/* Polynomial power: f^K using exp(K * log(f)) */
/* But f[0] might not be 1. For our case, the connected-graph EGF has f[0]=0,
   so we shift and handle specially. Actually for the log of p, the result has [0]=0.
   We raise that to K-th power. Since [0]=0, we can use:
   f^K where f = x * h(x), then f^K = x^K * h(x)^K.
   h(x) = f(x)/x shifted down by 1. Then h[0] = f[1].
   h^K = exp(K * log(h)).
*/

int main(void) {
    const int N = 10000;
    const int K = 10;

    precompute(N);
    precompute_inv(N);

    /* Step 1: p = a * e^{-x} (graphs using ALL elements) */
    ll *p = (ll*)calloc(N+1, sizeof(ll));
    poly_mul(a_coeff, e_neg, p, N);

    /* Step 2: log(p) = EGF of connected graphs */
    /* p[0] should be 1 (empty graph has 1 way) */
    /* Actually a_coeff[0] = 2^1 / 0! = 2, e_neg[0] = 1, so p[0] = 2*1 = 2 */
    /* Hmm, let's check: for i=0, 2^(2^0) = 2^1 = 2, /0! = 2. That's the number of subsets of Q(0)={}.
       Actually Q(0) has no non-empty subsets, so R(0) = {{}} has 1 element.
       Hmm, need to be careful about indexing. */
    /* For the EGF approach, the standard formulation has:
       a(0) = 1 (the empty graph)
       The 2^(2^0) = 2 includes the empty set as a "vertex", but in this problem
       Q(n) only has non-empty subsets, so the number of elements in R(n) is 2^(2^n - 1)
       (non-empty subsets of Q(n)).
       But for EGF, a(i)/i! should be the number of R-structures on an i-element set.

       Actually the coefficient a(i) in the EGF is the number of non-empty subsets of
       the non-empty subsets of {1..i}, which is 2^(2^i - 1) - 1 + 1 = 2^(2^i - 1).
       Wait, R(n) = non-empty subsets of Q(n), |Q(n)| = 2^n - 1, so |R(n)| = 2^(2^n-1) - 1.

       Hmm, but the EGF approach as described should have a[0] = 1 for the theory to work
       (empty structure). Let me reconsider.

       Actually the standard EGF for graphs: a_n = total number of labeled graphs on n vertices.
       Here each "vertex set" is a non-empty subset of {1..n}. The total number of "graphs"
       (elements of R(n)) is 2^(2^n - 1) (each non-empty subset of Q(n), but we also include
       the empty subset if we want the EGF to work).

       Let me just set a[0] = 1 and see.
    */
    /* Actually, let's reconsider. In the exponential formula:
       - The total number of set systems on {1..n} (possibly empty) = 2^(2^n - 1)
         (subsets of Q(n) which has 2^n - 1 elements)
         But we also count the empty set system.
         Total including empty: 2^(2^n - 1). Actually 2^|Q(n)| = 2^(2^n - 1).

       But the standard exponential formula for species says:
       F(x) = sum_n f_n * x^n / n!
       where f_n = number of F-structures on an n-element set.

       For our problem, we're looking at set systems on ground set {1..n}.
       Hmm, this is getting complicated. Let me just trust the existing algorithm structure
       and focus on making it fast.

       The existing code computes a_coeff[i] = 2^(2^i) / i!, which for i=0 gives 2.
       Then multiplies by e^{-x}, takes log, powers, etc. Let me just trust it and
       make the polynomial ops fast.
    */

    ll *logp = (ll*)calloc(N+1, sizeof(ll));
    poly_log(p, logp, N);

    /* Step 3: Raise to K-th power: (log p)^K
       logp[0] = 0 (since log of series starting with 1... but p[0]=2 not 1)
       Actually for log to work, p[0] must be 1. If p[0] = 2, we need to adjust.

       Let me normalize: p_norm = p / p[0], then log(p) = log(p[0]) + log(p_norm)
       But log(2) doesn't make sense in modular arithmetic for the constant term.

       Actually, the exponential formula: if F = exp(C) where C is the connected part,
       then C = log(F). For this to work, F[0] must equal 1 (since exp(0)=1 for the
       constant term). So we need F[0] = 1.

       In the original code, a_coeff is computed and then multiplied by e^{-x}.
       Let me check what p[0] equals:
       p[0] = sum_{k=0}^{0} a_coeff[k] * e_neg[0-k] = a_coeff[0] * e_neg[0]
       a_coeff[0] = 2^(2^0) * inv_fact[0] = 2 * 1 = 2
       e_neg[0] = 1
       So p[0] = 2.

       For the EGF to work, we should have p[0] = 1 (the empty structure on 0 elements).
       The issue: 2^(2^0) = 2, which counts all subsets of Q(0) = {} (the power set of the
       empty set). The subsets of {} are just {} itself, so there's 1 subset. But 2^(2^0) = 2^1 = 2.

       Hmm, 2^n for n=0: Q(0) = all non-empty subsets of {} = empty set.
       R(0) = all non-empty subsets of Q(0) = all non-empty subsets of {} = nothing.

       So a[0] should be 1 (counting the empty graph/empty family as the base case).
       The formula 2^(2^i) is wrong for i=0. It should be 2^(|Q(i)|) = 2^(2^i - 1).
       For i=0: 2^(2^0 - 1) = 2^0 = 1. That makes a[0] = 1.
       For i=1: 2^(2^1 - 1) = 2^1 = 2.
       For i=2: 2^(2^2 - 1) = 2^3 = 8.

       Let me redo with 2^(2^i - 1).
    */

    /* Redo a_coeff with correct formula: 2^(2^i - 1) / i! */
    for (int i = 0; i <= N; i++) {
        ll exp_val = power(2, i, MOD - 1);  /* 2^i mod (MOD-1) */
        exp_val = (exp_val - 1 + MOD - 1) % (MOD - 1);  /* 2^i - 1 mod (MOD-1) */
        a_coeff[i] = power(2, exp_val, MOD) % MOD * inv_fact[i] % MOD;
    }

    /* Redo p = a * e^{-x} */
    poly_mul(a_coeff, e_neg, p, N);
    /* Now p[0] should be 1 */

    /* log(p) */
    poly_log(p, logp, N);

    /* logp is the EGF of connected graphs. logp[0] should be 0. */

    /* Step 3: compute logp^K. Since logp[0] = 0, write logp = x * h(x).
       Then logp^K = x^K * h(x)^K.
       h(x)^K = exp(K * log(h(x))), and h[0] = logp[1] which should be nonzero. */

    /* h = logp shifted down by 1 */
    ll *h = (ll*)calloc(N+1, sizeof(ll));
    for (int i = 0; i <= N-1; i++) h[i] = logp[i+1];

    /* If h[0] = 0, there might be a higher-order zero. Let's just handle it. */
    /* Compute h^K = exp(K * log(h)) where h[0] != 0 */
    /* First normalize: h_norm = h / h[0] so h_norm[0] = 1 */
    ll h0 = h[0];
    ll h0_inv = inv(h0, MOD);
    ll *h_norm = (ll*)calloc(N+1, sizeof(ll));
    for (int i = 0; i <= N-K; i++) h_norm[i] = h[i] * h0_inv % MOD;

    /* log(h_norm) */
    ll *log_h = (ll*)calloc(N+1, sizeof(ll));
    poly_log(h_norm, log_h, N-K);

    /* K * log(h_norm) */
    for (int i = 0; i <= N-K; i++) log_h[i] = log_h[i] * K % MOD;

    /* exp(K * log(h_norm)) */
    ll *h_pow = (ll*)calloc(N+1, sizeof(ll));
    poly_exp_fast(log_h, h_pow, N-K);

    /* h^K = h0^K * h_pow */
    ll h0K = power(h0, K, MOD);
    for (int i = 0; i <= N-K; i++) h_pow[i] = h_pow[i] * h0K % MOD;

    /* logp^K: shift h_pow up by K */
    ll *logpK = (ll*)calloc(N+1, sizeof(ll));
    for (int i = K; i <= N; i++) logpK[i] = h_pow[i-K];

    /* Step 4: multiply by e^x */
    ll *result = (ll*)calloc(N+1, sizeof(ll));
    poly_mul(logpK, e_pos, result, N);

    /* Step 5: answer = result[N] * N! / K! */
    ll ans = result[N] * fact[N] % MOD * inv_fact[K] % MOD;
    printf("%lld\n", ans);

    free(p); free(logp); free(h); free(h_norm); free(log_h); free(h_pow); free(logpK); free(result);
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
        print(f"Error: {e.stderr}", flush=True)
        raise
    finally:
        os.unlink(src_path)
        if os.path.exists(bin_path):
            os.unlink(bin_path)

if __name__ == "__main__":
    solve()
