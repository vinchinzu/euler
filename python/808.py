"""Project Euler Problem 808: Reversible Prime Squares.

Find the sum of the first 50 prime squares whose digit-reversal is also
a (different) prime square.  Embedded C with bitset sieve for speed.
"""
import subprocess, tempfile, os

def solve():
    c_code = r"""
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

typedef long long ll;

#define LIMIT 100000001

static unsigned char sieve[(LIMIT >> 3) + 2];

#define IS_COMPOSITE(n) (sieve[(n) >> 3] & (1 << ((n) & 7)))
#define SET_COMPOSITE(n) (sieve[(n) >> 3] |= (1 << ((n) & 7)))

void do_sieve(void) {
    memset(sieve, 0, sizeof(sieve));
    SET_COMPOSITE(0);
    SET_COMPOSITE(1);
    for (ll i = 2; i * i < LIMIT; i++) {
        if (!IS_COMPOSITE(i)) {
            for (ll j = i * i; j < LIMIT; j += i)
                SET_COMPOSITE(j);
        }
    }
}

ll reverse_num(ll n) {
    ll rev = 0;
    while (n > 0) { rev = rev * 10 + n % 10; n /= 10; }
    return rev;
}

ll isqrt_ll(ll n) {
    ll x = (ll)sqrtl((long double)n);
    while (x > 0 && x * x > n) x--;
    while ((x + 1) * (x + 1) <= n) x++;
    return x;
}

int main(void) {
    do_sieve();

    int N = 50, count = 0;
    ll sum = 0;

    for (ll p = 2; p < LIMIT && count < N; p++) {
        if (IS_COMPOSITE(p)) continue;
        ll sq = p * p;
        ll rev = reverse_num(sq);
        if (rev == sq) continue;            /* palindrome */
        ll sr = isqrt_ll(rev);
        if (sr * sr != rev) continue;       /* not a perfect square */
        if (sr >= LIMIT) continue;
        if (!IS_COMPOSITE((int)sr)) {       /* sqrt is prime */
            sum += sq;
            count++;
        }
    }

    printf("%lld\n", sum);
    return 0;
}
"""
    with tempfile.NamedTemporaryFile(suffix='.c', mode='w', delete=False) as f:
        f.write(c_code)
        c_path = f.name
    exe_path = c_path.replace('.c', '')
    try:
        subprocess.run(['gcc', '-O2', '-o', exe_path, c_path, '-lm'],
                       check=True, capture_output=True)
        result = subprocess.run([exe_path], capture_output=True, text=True, timeout=280)
        return result.stdout.strip()
    finally:
        for p in [c_path, exe_path]:
            if os.path.exists(p):
                os.unlink(p)

if __name__ == "__main__":
    print(solve())
