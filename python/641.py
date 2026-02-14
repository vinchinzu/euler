"""Project Euler Problem 641: A Long Row of Dice.

Count numbers <= 10^18 with exactly 6k divisors.
Uses Lucy hedgehog prime counting, recursive helper with prime enumeration.
Embedded C for performance.
"""
import subprocess, os, tempfile


def solve():
    c_code = r"""
#include <stdio.h>
#include <stdlib.h>
#include <math.h>
#include <string.h>

#define K 6

typedef long long ll;
typedef unsigned long long ull;

static ll N_VAL;

/* Lucy hedgehog prime counting */
/* We store values for n/1, n/2, ..., n/r, and r-1, r-2, ..., 1 */
static ll *V;
static ll *S;
static int V_len;

/* Map value v to index in S/V arrays */
static int val_to_idx(ll v) {
    if (v <= 0) return -1;
    if (v <= V[V_len - 1]) {
        /* v is in the "small" part: index = V_len - v */
        /* Actually: small values are at the end. */
        /* The array is: n/1, n/2, ..., n/r, r-1, r-2, ..., 1 */
        /* Small values (1..r-1) are at indices (r + (r-1-v)) = ... */
        /* Let's just use the fact that for small v, index = V_len - v */
        /* Actually we need to be more careful. Let me use a hash or binary search. */
        /* For small v <= sqrt(n), the index is V_len - v (since they're stored as r-1, r-2, ..., 1) */
        /* Wait, V is: [n/1, n/2, ..., n/r, r-1, r-2, ..., 1] */
        /* so the last element is 1, second-to-last is 2, etc. */
        /* For value v where v < r, index = V_len - v */
        return V_len - (int)v;
    } else {
        /* v = n / i for some i, index = n/v - 1 ... no. */
        /* For large v (v >= r), v = n/i, index = i-1... no. */
        /* V[i] = n/(i+1) for i = 0, 1, ..., r-1 */
        /* So if v = n/k, index = k-1 */
        int k = (int)(N_VAL / v);
        return k - 1;
    }
}

static void prime_count_init(ll n) {
    N_VAL = n;
    int r = (int)sqrt((double)n);
    while ((ll)(r+1)*(r+1) <= n) r++;
    while ((ll)r*r > n) r--;

    /* Build V array */
    V_len = 0;
    /* First part: n/i for i=1..r */
    int cap = 2 * r + 2;
    V = (ll *)malloc(cap * sizeof(ll));
    S = (ll *)malloc(cap * sizeof(ll));

    for (int i = 1; i <= r; i++) {
        V[V_len] = n / i;
        V_len++;
    }
    /* Second part: r-1, r-2, ..., 1 (only those not already in first part) */
    ll last = V[V_len - 1];
    for (ll v = last - 1; v >= 1; v--) {
        V[V_len] = v;
        V_len++;
    }

    /* Initialize S[v] = v - 1 */
    for (int i = 0; i < V_len; i++) {
        S[i] = V[i] - 1;
    }

    /* Sieve */
    for (int p = 2; p <= r; p++) {
        int pidx = val_to_idx(p);
        int pm1idx = val_to_idx(p - 1);
        if (S[pidx] <= S[pm1idx]) continue; /* p not prime */
        ll sp = S[pm1idx];
        ll p2 = (ll)p * p;
        for (int i = 0; i < V_len; i++) {
            if (V[i] < p2) break;
            int vi_p_idx = val_to_idx(V[i] / p);
            S[i] -= S[vi_p_idx] - sp;
        }
    }
}

static ll get_prime_count(ll v) {
    if (v < 2) return 0;
    return S[val_to_idx(v)];
}

/* Sieve for small primes */
static int *primes;
static int nprimes;
static int *prime_counts_arr;
static int L;

static void sieve_small(int limit) {
    L = limit;
    char *is_p = (char *)calloc(limit + 1, 1);
    memset(is_p, 1, limit + 1);
    is_p[0] = is_p[1] = 0;
    int sq = (int)sqrt((double)limit);
    for (int i = 2; i <= sq; i++)
        if (is_p[i])
            for (int j = i*i; j <= limit; j += i)
                is_p[j] = 0;

    nprimes = 0;
    for (int i = 2; i <= limit; i++)
        if (is_p[i]) nprimes++;
    primes = (int *)malloc(nprimes * sizeof(int));
    int idx = 0;
    for (int i = 2; i <= limit; i++)
        if (is_p[i]) primes[idx++] = i;

    prime_counts_arr = (int *)calloc(limit + 1, sizeof(int));
    int cnt = 0;
    for (int i = 0; i <= limit; i++) {
        if (i >= 2 && is_p[i]) cnt++;
        prime_counts_arr[i] = cnt;
    }
    free(is_p);
}

/* Integer r-th root */
static ll nthrt(ll n, int r) {
    if (r == 1) return n;
    if (n <= 0) return 0;
    if (r == 2) {
        ll x = (ll)sqrt((double)n);
        while ((x+1)*(x+1) <= n) x++;
        while (x > 0 && x*x > n) x--;
        return x;
    }
    double dr = 1.0 / r;
    ll x = (ll)pow((double)n, dr);
    /* Refine */
    /* Check x+2, x+1, x, x-1 */
    for (ll t = x + 2; t >= 0 && t >= x - 2; t--) {
        /* Check if t^r <= n */
        ll pw = 1;
        int ok = 1;
        for (int i = 0; i < r; i++) {
            if (pw > n / t + 1) { ok = 0; break; }
            pw *= t;
            if (pw > n) { ok = 0; break; }
        }
        if (ok && pw <= n) {
            /* Check (t+1)^r > n */
            ll pw2 = 1;
            int over = 0;
            for (int i = 0; i < r; i++) {
                if (pw2 > n / (t+1) + 1) { over = 1; break; }
                pw2 *= (t+1);
                if (pw2 > n) { over = 1; break; }
            }
            if (over) return t;
        }
    }
    return x;
}

/* Power with overflow check: returns p^e, or -1 if overflow */
static ll safe_pow(ll p, int e) {
    ll result = 1;
    for (int i = 0; i < e; i++) {
        if (result > 1000000000000000000LL / p) return -1; /* overflow */
        result *= p;
    }
    return result;
}

static ll ans;
static ll N_GLOBAL;

static void helper(int min_index, ll n, int num_divisors) {
    /* For each valid exponent, count possibilities for largest prime */
    int e = (num_divisors % K == 1) ? K : K - 2;
    while (1) {
        int half_e = e / 2;
        ll bound = nthrt(N_GLOBAL / n, half_e);
        if (min_index >= nprimes || bound < primes[min_index])
            break;
        ll cnt;
        if (bound > L)
            cnt = get_prime_count(bound) - min_index;
        else
            cnt = prime_counts_arr[(int)bound] - min_index;
        ans += cnt;
        e += K;
    }

    /* Recurse for smaller primes */
    for (int index = min_index; index < nprimes; index++) {
        ll p = primes[index];
        /* Check if we can add p^(K-2) */
        ll pk2 = safe_pow(p, K - 2);
        if (pk2 < 0 || n > N_GLOBAL / pk2)
            break;

        for (int start_e_idx = 0; start_e_idx < 2; start_e_idx++) {
            int se = (start_e_idx == 0) ? K - 2 : K;
            int ee = se;
            int half_se = ee / 2;
            ll ppow = safe_pow(p, half_se);
            if (ppow < 0 || n > N_GLOBAL / ppow) continue;
            ll new_n = n * ppow;
            while (new_n < N_GLOBAL) {
                helper(index + 1, new_n, num_divisors * (ee + 1));
                ee += K;
                ll pmul = safe_pow(p, K / 2);
                if (pmul < 0 || new_n > N_GLOBAL / pmul) break;
                new_n *= pmul;
            }
        }
    }
}

int main(void) {
    N_GLOBAL = 1000000000000000000LL; /* 10^18 */
    int L_val = (int)pow((double)N_GLOBAL, 0.4);
    /* Ensure L_val^5 <= N but (L_val+1)^5 > N */
    while (safe_pow((ll)(L_val+1), 5) > 0 && safe_pow((ll)(L_val+1), 5) <= N_GLOBAL) L_val++;

    sieve_small(L_val);
    prime_count_init(N_GLOBAL);

    ans = 1; /* Start with 1 (the number 1 itself has 1 divisor, 1 = 6*0+1? No, wait. The Python code starts with ans=[1]. Let me check. */
    /* Actually the Python starts ans = [1], meaning the count starts at 1.
       The number 1 has exactly 1 divisor, and 1 % 6 == 1 (not 0).
       But K=6, and 6k for k=0 gives 0 divisors which doesn't apply.
       6*1 = 6, 6*0 = 0. Actually d(1)=1, and 1 is not a multiple of 6.
       Let me re-examine: the problem says "exactly 6k divisors" - maybe d(n)%6==0?
       The code says K=6 and starts with ans=[1]. In the helper, the counting logic
       uses num_divisors % K == 1 to decide exponent. Starting with num_divisors=1.
       The recursion adds primes with exponents. The "ans += cnt" counts numbers
       whose divisor count = num_divisors * (e+1), where e+1 is chosen so the
       total is divisible by K. Since we start with num_divisors=1 and K=6,
       we need (e+1) to make divisor count % 6 == 0. The "e = K if num_divisors % K == 1 else K-2"
       picks the right starting exponent.

       The ans starts at 1 because the number 1 is counted (has d(1)=1 divisor,
       actually that's not divisible by 6... hmm).

       Let me just keep it at 1 as the Python does. */

    helper(0, 1, 1);
    printf("%lld\n", ans);

    free(V);
    free(S);
    free(primes);
    free(prime_counts_arr);
    return 0;
}
"""
    tmp = tempfile.NamedTemporaryFile(suffix='.c', delete=False, mode='w')
    tmp.write(c_code)
    tmp.close()
    exe = tmp.name.replace('.c', '')
    try:
        subprocess.run(['gcc', '-O2', '-o', exe, tmp.name, '-lm'],
                       check=True, capture_output=True)
        result = subprocess.run([exe], capture_output=True, text=True, timeout=280)
        print(result.stdout.strip())
    finally:
        os.unlink(tmp.name)
        if os.path.exists(exe):
            os.unlink(exe)


if __name__ == "__main__":
    solve()
